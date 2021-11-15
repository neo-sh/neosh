use std::ffi;
use std::fs;
use std::io;
use std::path::PathBuf;

pub struct NeoshPaths {
    pub data: PathBuf,
    pub cache: PathBuf,
    pub config: PathBuf,
}

impl NeoshPaths {
    // TODO: expose this stdpath function as a Lua function
    pub fn stdpath(self, kind: &str) -> Result<String, ffi::OsString> {
        let stdpath: PathBuf = match kind {
            "data" => self.data,
            "cache" => self.cache,
            "config" => self.config,
            _ => return Err(ffi::OsString::from("Unknown stdpath kind provided")),
        };
        let path = stdpath.into_os_string().into_string()?;
        Ok(path)
    }
    pub fn create_neosh_dirs(&self) -> Result<(), io::Error> {
        let paths = vec![&self.data, &self.cache, &self.config];
        for path in paths.iter() {
            fs::create_dir_all(path)?;
        }
        Ok(())
    }
}
