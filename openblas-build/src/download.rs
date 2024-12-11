use anyhow::Result;
use std::path::{Path, PathBuf};

const OPENBLAS_VERSION: &str = "0.3.28";

pub fn openblas_source_url() -> String {
    format!(
        "https://github.com/OpenMathLib/OpenBLAS/releases/download/v{}/OpenBLAS-{}.tar.gz",
        OPENBLAS_VERSION, OPENBLAS_VERSION
    )
}

fn openblas_lib_url() -> String {
    let pointer_width = std::env::var("TARGET").unwrap_or_default();
    let arch = if pointer_width == "32" {
        "x86"
    } else {
        "x64"
    };
    format!(
        "https://github.com/OpenMathLib/OpenBLAS/releases/download/v{}/OpenBLAS-{}-{}.zip",
        OPENBLAS_VERSION, OPENBLAS_VERSION, arch
    )
}

pub fn download(out_dir: &Path) -> Result<PathBuf> {
    let dest = out_dir.join(format!("OpenBLAS-{}", OPENBLAS_VERSION));
    if !dest.exists() {
        let buf = get_agent()
            .get(&openblas_source_url())
            .call()?
            .into_reader();
        let gz_stream = flate2::read::GzDecoder::new(buf);
        let mut ar = tar::Archive::new(gz_stream);
        ar.unpack(out_dir)?;
        assert!(dest.exists());
    }
    Ok(dest)
}

fn get_agent() -> ureq::Agent {
    ureq::AgentBuilder::new()
        .tls_connector(std::sync::Arc::new(
            native_tls::TlsConnector::new().expect("failed to create TLS connector"),
        ))
        .try_proxy_from_env(true)
        .build()
}

pub fn download_openblas_windows<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    let dest = path.as_ref().join(format!("OpenBLAS-{}", OPENBLAS_VERSION));
    let url = openblas_lib_url();
    let buf = get_agent()
        .get(&url)
        .call()?
        .into_reader();
    let zip_file = zip::unstable::stream::ZipStreamReader::new(buf);
    zip_file.extract(dest)?;
    Ok(dest.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let out_dir = root.join("test_build/download");
        download_openblas_windows(out_dir).unwrap();
    }
}