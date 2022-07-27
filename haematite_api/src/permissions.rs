use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub enum Vertex {
    Internal(Tree),
    External,
}

#[derive(Debug, Default)]
pub struct Tree {
    hashmap: HashMap<String, Vertex>,
}

fn split_path(path: &str) -> Vec<String> {
    path.split('/').map(String::from).collect()
}

fn to_tree(paths: &HashSet<&str>) -> Tree {
    let mut hashmap = HashMap::new();

    let mut collected_children = HashMap::new();
    let mut paths = Vec::from_iter(paths.clone());
    // sort this so that if we have a "*", it comes first
    paths.sort_unstable();

    for path in paths {
        let (parent, child) = match path.split_once('/') {
            Some((parent, child)) => (parent, Some(child)),
            None => (path, None),
        };

        let children = collected_children
            .entry(parent.to_string())
            .or_insert_with(Vec::new);

        if let Some(child) = child {
            children.push(child);
        }
    }

    if collected_children.contains_key("*") {
        let collected_all = collected_children["*"].clone();
        for value in collected_children.values_mut() {
            value.append(&mut collected_all.clone());
        }
    }

    for (parent, children) in collected_children.into_iter() {
        hashmap.insert(
            parent,
            match children.is_empty() {
                true => Vertex::External,
                false => Vertex::Internal(to_tree(&HashSet::from_iter(children))),
            },
        );
    }

    Tree { hashmap }
}

#[derive(Debug)]
pub enum MergeError {
    MismatchInternal,
    MismatchExternal,
}

impl Tree {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_paths(paths: &HashSet<&str>) -> Self {
        to_tree(paths)
    }

    pub fn get(&self, key: &str) -> Option<&Vertex> {
        self.hashmap.get(key)
    }

    pub fn update(&mut self, other: Tree) -> Result<(), MergeError> {
        for (key, value) in other.hashmap.into_iter() {
            match self.hashmap.get_mut(&key) {
                None => {
                    self.hashmap.insert(key, value);
                }
                Some(Vertex::Internal(tree_us)) => {
                    if let Vertex::Internal(tree_them) = value {
                        tree_us.update(tree_them)?;
                    } else {
                        return Err(MergeError::MismatchInternal);
                    }
                }
                Some(Vertex::External) => {
                    if matches!(value, Vertex::External) {
                        self.hashmap.insert(key, value);
                    } else {
                        return Err(MergeError::MismatchExternal);
                    }
                }
            };
        }

        Ok(())
    }

    pub fn add_path(&mut self, path: &str) -> Result<(), MergeError> {
        let tree = to_tree(&HashSet::from([path]));
        self.update(tree)
    }

    pub fn find_tree(&self, path: &str) -> Option<&Tree> {
        let query = split_path(path);

        let mut current_tree = self;
        for part in query {
            if let Some(Vertex::Internal(tree)) = current_tree.hashmap.get(&part) {
                current_tree = tree;
            } else {
                return None;
            }
        }

        Some(current_tree)
    }
}
