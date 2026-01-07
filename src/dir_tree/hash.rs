use super::DirTree;
use std::hash::{Hash, Hasher};

impl Hash for DirTree {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dir_name.hash(state);

        for file in &self.files {
            file.hash(state);
        }

        for dir in &self.dirs {
            dir.hash(state);
        }
    }
}
