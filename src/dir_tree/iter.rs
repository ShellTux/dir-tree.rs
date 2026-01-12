use super::DirTree;
use std::{collections::hash_set, ffi::OsString};

pub struct DirTreeIter<'dirtree, T> {
    files_iter: hash_set::Iter<'dirtree, T>,
    stack_dirs_iter: Vec<DirTreeIter<'dirtree, T>>,
}

impl<'dirtree, T> Iterator for DirTreeIter<'dirtree, T> {
    type Item = &'dirtree T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(file) = self.files_iter.next() {
            return Some(file);
        }

        while let Some(dir_tree) = self.stack_dirs_iter.last_mut() {
            if let Some(file) = dir_tree.next() {
                return Some(file);
            } else {
                self.stack_dirs_iter.pop();
            }
        }

        None
    }
}

impl<'a> IntoIterator for &'a DirTree {
    type Item = &'a OsString;
    type IntoIter = DirTreeIter<'a, OsString>;

    fn into_iter(self) -> Self::IntoIter {
        DirTreeIter {
            files_iter: self.files.iter(),
            stack_dirs_iter: self.dirs.iter().map(|i| i.into_iter()).collect(),
        }
    }
}
