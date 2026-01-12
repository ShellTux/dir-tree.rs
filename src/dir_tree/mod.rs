use custom_debug::Debug;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, ffi::OsString, fs, io, path::Path};

mod hash;
pub mod iter;

#[derive(Clone, Debug, Serialize, Deserialize, Getters, PartialEq, Eq)]
pub struct DirTree {
    dir_name: OsString,

    #[getter(skip)]
    #[debug(skip_if = HashSet::is_empty)]
    files: HashSet<OsString>,

    #[getter(skip)]
    #[serde(skip_serializing_if = "HashSet::is_empty", default)]
    #[debug(skip_if = HashSet::is_empty)]
    dirs: HashSet<DirTree>,
}

impl DirTree {
    /// # Errors
    ///
    /// Will return `io::Error`.
    pub fn from<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut tree = DirTree {
            files: HashSet::new(),
            dirs: HashSet::new(),
            dir_name: path.as_ref().as_os_str().to_os_string(),
        };

        for entry in fs::read_dir(path)? {
            let path = entry?.path();

            if path.is_symlink() {
                continue;
            }

            if path.is_dir() {
                tree.dirs.insert(DirTree::from(path)?);
            } else if path.is_file() {
                tree.files.insert(path.into());
            }
        }

        Ok(tree)
    }

    #[must_use]
    pub fn file_count(&self) -> usize {
        self.into_iter().count()
    }
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//}
