use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    fs, io,
    path::{Path, PathBuf},
};

mod hash;
pub mod iter;

#[derive(Clone, Debug, Serialize, Deserialize, Getters, PartialEq, Eq)]
pub struct DirTree {
    dir_name: PathBuf,

    #[getter(skip)]
    files: HashSet<PathBuf>,

    #[getter(skip)]
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
            dir_name: path.as_ref().to_path_buf(),
        };

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                tree.dirs.insert(DirTree::from(path)?);
            } else if path.is_file() {
                tree.files.insert(path);
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
