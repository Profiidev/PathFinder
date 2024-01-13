use std::cmp::Ordering;

use serde::{Serialize, Deserialize};
use crate::file::File;

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Tree {
    pub root: Node,
    pub id: String,
}

#[derive(Debug)]
pub enum TreeSearchType {
    Exact,
    Contains,
    StartsWith,
    ExactNoType,
    ContainsNoType,
    StartsWithNoType,
}

impl Tree {
    pub fn new(root: Node, id: String) -> Self {
        Self { root: root, id: id }
    }

    pub fn get_root(&self) -> &Node {
        &self.root
    }

    pub fn get_root_mut(&mut self) -> &mut Node {
        &mut self.root
    }

    pub fn get_root_location(&self) -> String {
        self.root.data.name.clone()
    }

    fn search(origin: &Node, data: &File, search_type: &TreeSearchType) -> Vec<File>{
        match search_type {
            TreeSearchType::Exact => origin.search(data, &|a, b| a.name == b.name && a.is_dir == b.is_dir),
            TreeSearchType::Contains => origin.search(data, &|a, b| a.name.contains(&b.name) && a.is_dir == b.is_dir),
            TreeSearchType::StartsWith => origin.search(data, &|a, b| a.name.starts_with(&b.name) && a.is_dir == b.is_dir),
            TreeSearchType::ExactNoType => origin.search(data, &|a, b| a.name == b.name),
            TreeSearchType::ContainsNoType => origin.search(data, &|a, b| a.name.contains(&b.name)),
            TreeSearchType::StartsWithNoType => origin.search(data, &|a, b| a.name.starts_with(&b.name)),
        }
    }

    pub fn search_partial(&self, path: &str, data: &File, search_type: &TreeSearchType) -> Vec<File> {
        let path = parse_path(path, &self.root.data.name);
        match self.root.find_from_path(path) {
            Some(node) => {
                Tree::search(node, data, search_type)
            },
            None => Vec::new(),
        }
    }

    pub fn is_in_tree(&self, path: &str) -> bool {
        return path.starts_with(&self.root.data.name);
    }

    pub fn add_node(&mut self, path: &str, file: File) {
        let path = parse_path(path, &self.root.data.name);
        self.root.add_node(path, file);
    }

    pub fn remove_node(&mut self, path: &str) {
        let path = parse_path(path, &self.root.data.name);
        self.root.remove_node(path);
    }

    pub fn edit_node(&mut self, path: &str, file: File) {
        let path = parse_path(path, &self.root.data.name);
        self.root.edit_node(path, file);
    }
}

fn parse_path(path: &str, root_name: &str) -> Vec<String> {
    if path == root_name {
        return Vec::new();
    }
    let mut path = path.to_string();
    path.replace_range(0..root_name.len() + 1, "");
    path.split("/").map(|s| s.to_string()).collect::<Vec<String>>()
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Node {
    pub data: File,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(data: File) -> Self {
        Self {
            data,
            children: Vec::new(),
        }
    }

    pub fn add_children(&mut self, children: Vec<Node>) {
        self.children.extend(children);
    }

    pub fn compare(&self, data: &File) -> Ordering {
        if self.data.is_dir && !data.is_dir {
            return std::cmp::Ordering::Less;
        } else if !self.data.is_dir && data.is_dir {
            return std::cmp::Ordering::Greater;
        } else {
            return self.data.name.cmp(&data.name);
        }
    }

    pub fn search(&self, data: &File, comparator: &dyn Fn(File, File) -> bool) -> Vec<File> {
        let mut found = Vec::new();
        if comparator(self.data.clone(), data.clone()) {
            found.push(self.data.clone());
        }
        for child in &self.children {
            found.extend(child.search(data, comparator));
        }
        found
    }

    pub fn find_from_path(&self, path: Vec<String>) -> Option<&Node> {
        if path.len() == 0 {
            return Some(self);
        }
        let mut path = path;
        let next = path.remove(0);
        for child in &self.children {
            if child.data.name == next {
                return child.find_from_path(path);
            }
        }
        None
    }

    pub fn add_node(&mut self, mut path: Vec<String>, file: File) {
        if path.len() == 1 {
            self.children.push(Node::new(file));
            self.children.sort_by(|a, b| a.compare(&b.data));
            return;
        }
        let next = path.remove(0);
        for child in &mut self.children {
            if child.data.name == next {
                child.add_node(path, file);
                return;
            }
        }
    }

    pub fn remove_node(&mut self, mut path: Vec<String>) {
        let next = path.remove(0);
        for child in &mut self.children {
            if child.data.name == next {
                if path.len() == 0 {
                    self.children.retain(|x| x.data.name != next);
                    return;
                }
                child.remove_node(path);
                return;
            }
        }
    }

    pub fn edit_node(&mut self, mut path: Vec<String>, file: File) {
        let next = path.remove(0);
        for child in &mut self.children {
            if child.data.name == next {
                if path.len() == 0 {
                    child.data = file;
                    return;
                }
                child.edit_node(path, file);
                return;
            }
        }
    }
}