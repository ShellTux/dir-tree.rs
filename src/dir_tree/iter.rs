use super::DirTree;
use std::{collections::hash_set, path::PathBuf};

pub struct DirTreeIter<'dirtree> {
    files_iter: hash_set::Iter<'dirtree, PathBuf>,
    stack_dirs_iter: Vec<DirTreeIter<'dirtree>>,
}

impl<'dirtree> Iterator for DirTreeIter<'dirtree> {
    type Item = &'dirtree PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(file) = self.files_iter.next() {
            return Some(file);
        }

        while let Some(mut dir_iter) = self.stack_dirs_iter.pop() {
            if let Some(new_file) = dir_iter.next() {
                self.stack_dirs_iter.push(dir_iter);
                return Some(new_file);
            }
        }

        None
    }
}

impl<'a> IntoIterator for &'a DirTree {
    type Item = &'a PathBuf;
    type IntoIter = DirTreeIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        DirTreeIter {
            files_iter: self.files.iter(),
            stack_dirs_iter: self.dirs.iter().map(|i| i.into_iter()).collect(),
        }
    }
}
