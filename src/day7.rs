//! Advent of Code 2022, Day 7

// We will get a shell log as input, and we need to basically implement our own du -s
// We need to read the prompts and traverse the filesystem to find the size of the directories

use path_absolutize::*;
use std::{collections::BTreeMap, path::PathBuf, str::Lines};

pub fn generator(input: &str) -> Tree {
    let mut sizes = BTreeMap::new();

    let mut lines = input.lines();

    let mut pbuf = PathBuf::from("/");
    while let Some(out) = lines.next() {
        if out.starts_with("$ cd") {
            let cd = &out[5..];
            // println!("cd: {:?}", cd);

            if cd.starts_with('/') {
                pbuf = PathBuf::from(cd);
            } else {
                pbuf = pbuf.join(cd).absolutize().unwrap().to_path_buf();
            }

            println!("cd: {:?}", cd);

            println!("dir_buf: {:?}", pbuf);
        }

        if out.starts_with("$ ls") {
            // the following lines are the output of ls
            for out in lines.by_ref() {
                if out.starts_with('$') {
                    break;
                }
                println!("out: {:?}", out);

                let mut parts = out.split_whitespace();
                let size = parts.next().unwrap();

                // if the size is a number, then it's a file
                if let Ok(size) = size.parse::<usize>() {
                    let name = parts.next().unwrap();
                    let path = pbuf.join(name);
                    // println!("path: {:?}", path);
                    sizes.insert(path, Entry::File(size));
                } else {
                    // dir
                    let name = parts.next().unwrap();
                    let path = pbuf.join(name);
                    // println!("path: {:?}", path);
                    sizes.insert(path, Entry::Dir);
                }
            }
        }
    }
    println!("sizes: {:#?}", sizes);
    Tree { inner: sizes }
}

#[derive(Debug)]
pub enum Entry {
    Dir,
    File(usize),
}

#[derive(Debug)]
pub struct Tree {
    pub inner: BTreeMap<PathBuf, Entry>,
}

impl Tree {
    pub fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, path: PathBuf, entry: Entry) {
        self.inner.insert(path, entry);
    }

    pub fn get(&self, path: &PathBuf) -> Option<&Entry> {
        self.inner.get(path)
    }

    pub fn get_mut(&mut self, path: &PathBuf) -> Option<&mut Entry> {
        self.inner.get_mut(path)
    }

    pub fn iter(&self) -> std::collections::btree_map::Iter<PathBuf, Entry> {
        self.inner.iter()
    }

    pub fn iter_mut(&mut self) -> std::collections::btree_map::IterMut<PathBuf, Entry> {
        self.inner.iter_mut()
    }

    pub fn is_empty(&self, path: &PathBuf) -> bool {
        match self.get(path) {
            Some(Entry::File(_)) => false,
            Some(Entry::Dir) => {
                for (p, _) in self.iter() {
                    // if starts with and is not the same path
                    if p.starts_with(path) && p != path {
                        return false;
                    }
                }
                true
            }
            None => true,
        }
    }

    pub fn size(&self, path: &PathBuf) -> usize {
        // let mut already_counted = Vec::new();
        // println!("du -s {:?}", path);

        // println!("{:#?}", self.ls(path));

        match self.get(path) {
            Some(Entry::File(size)) => *size,
            Some(Entry::Dir) => {
                let mut size = 0;
                for (p, e) in self.iter() {
                    // println!("size: {:?}", p);
                    if p == path {
                        continue;
                    }
                    if p.starts_with(path) {
                        // println!("p: {:?}", p);
                        size += match e {
                            Entry::File(size) => *size,
                            // found the infinite loop
                            Entry::Dir => {
                                // println!("found dir: {:?}", p);
                                // if it's trying to size itself, skip it
                                if p == path || self.is_empty(p) {
                                    // println!("empty: {:?}", p);
                                    0
                                } else {
                                    let mut s = 0;

                                    // recursively add the size of the directory
                                    for (p, e) in self.ls(p) {
                                        // println!("p2: {:?}", p);
                                        s += match e {
                                            Entry::File(size) => {
                                                // println!("size: {:?}", size);
                                                *size
                                            }
                                            Entry::Dir => self.size(&p),
                                        }
                                    }
                                    s
                                }
                            }
                        }
                    }
                    // println!("size: {:?}", size);
                }
                size
            }
            None => 0,
        }
    }
    pub fn size_shallow(&self, path: &PathBuf) -> usize {
        match self.get(path) {
            Some(Entry::File(size)) => *size,
            Some(Entry::Dir) => {
                let mut size = 0;
                for (p, e) in self.iter() {
                    if p.starts_with(path) {
                        size += match e {
                            Entry::File(size) => *size,
                            Entry::Dir => 0,
                        }
                    }
                }
                size
            }
            None => 0,
        }
    }

    pub fn ls(&self, path: &PathBuf) -> BTreeMap<PathBuf, &Entry> {
        let mut out = BTreeMap::new();
        for (p, _) in self.iter() {
            if p.starts_with(path) {
                out.insert(p.clone(), self.get(p).unwrap());
            }
        }
        out
    }
}

pub fn part_1(input: &Tree) -> usize {
    // we have a btreemap of paths and sizes
    // part 1 question is to find all directories over 100000 bytes and sum all of them
    // process can count files more than once

    // let mut already_counted = Vec::new();

    let bigfolders = {
        // let mut files = Vec::new();

        let res = input.inner.values().filter(|e| {
            if let Entry::File(size) = e {
                size <= &100_000
            } else {
                false
            }

        }).map(|e| {
            if let Entry::File(size) = e {
                *size
            } else {
                0
            }
        }).sum::<usize>();
        res
    };

    // println!("bigfolders: {:#?}", bigfolders);
    bigfolders

    // bigfolders.iter().map(|(_, size, _)| size).sum()
}
