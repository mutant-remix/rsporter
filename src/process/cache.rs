use std::{fs, path::PathBuf, fmt};
use crate::process::encode::EncodeTarget;

pub struct Cache {
    path: PathBuf,
}

impl Cache {
    pub fn new(output_path: &PathBuf) -> Self {
        let path = output_path.join("cache");
        Self { path }
    }

    pub fn try_get(&self, svg: &str, format: &EncodeTarget, size: u32) -> Option<Vec<u8>> {
        let hash = md5::compute(format!("{}-{:?}-{}", svg, format, size));
        let hash = format!("{:x}", hash);

        match fs::read(self.path.join(hash)) {
            Ok(data) => Some(data),
            Err(_) => None,
        }
    }

    pub fn save(&self, svg: &str, format: &EncodeTarget, size: u32, raster: &Vec<u8>) {
        match fs::create_dir_all(&self.path) {
            Ok(_) => {},
            Err(err) => panic!("Failed to create cache directory: {}", err),
        }

        let hash = md5::compute(format!("{}-{:?}-{}", svg, format, size));
        let hash = format!("{:x}", hash);

        match fs::write(self.path.join(hash), raster) {
            Ok(_) => {},
            Err(err) => panic!("Failed to write cache file: {}", err),
        }
    }
}

impl fmt::Debug for Cache {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("<Cache {:?}>", self.path))
    }
}
