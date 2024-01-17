use std::{cmp::Ordering, fs, time::UNIX_EPOCH, os::windows::prelude::*};
use serde::{Serialize, Deserialize};
use crate::file::{File, FileData};

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

    pub fn get_files(&self, path: &str) -> Vec<FileData> {
        let path_vec = parse_path(path, &self.root.data.name);
        self.root.get_files(path_vec, path)
    }
}

fn parse_path(path: &str, root_name: &str) -> Vec<String> {
    if path == root_name {
        return Vec::new();
    }
    let mut path = path.to_string();
    path.replace_range(0..root_name.len(), "");
    path.split("/").map(|s| s.to_string()).filter(|s| s != "").collect::<Vec<String>>()
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

    pub fn get_files(&self, mut path: Vec<String>, absolute_path: &str) -> Vec<FileData> {
        if path.len() == 0 {
            return self.children.iter().map(|x| x.get_file_data(absolute_path)).filter(|x| x.is_some()).map(|x| x.unwrap()).collect::<Vec<FileData>>();
        }
        let next = path.remove(0);
        for child in &self.children {
            if child.data.name == next {
                return child.get_files(path, absolute_path);
            }
        }
        Vec::new()
    }

    fn get_file_data(&self, absolute_path: &str) -> Option<FileData> {
        let metadata = match fs::metadata(format!("{}/{}", absolute_path, self.data.name)) {
            Ok(metadata) => metadata,
            Err(_) => {
                println!("Error reading metadata for: {}/{}", absolute_path, self.data.name);
                return None;
            }
        };

        let modified = match metadata.modified() {
            Ok(modified) => match modified.duration_since(UNIX_EPOCH) {
                Ok(duration) => duration.as_secs(),
                Err(_) => 0,
            },
            Err(_) => 0,
        };

        let created = match metadata.created() {
            Ok(created) => match created.duration_since(UNIX_EPOCH) {
                Ok(duration) => duration.as_secs(),
                Err(_) => 0,
            },
            Err(_) => 0,
        };

        let attributes = metadata.file_attributes();
        if attributes & 4 == 4 {
            return None;
        }
        
        Some(FileData {
            file: File {
                name: self.data.name.clone(),
                is_dir: self.data.is_dir,
            },
            path: absolute_path.to_string().clone(),
            size: Some(metadata.len()),
            last_modified_date: Some(modified),
            created_date: Some(created),
            permissions: Some(metadata.permissions().readonly()),
            hidden: Some(attributes & 2 == 2),
        })
    }
}