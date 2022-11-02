use super::path::Path;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Tree {
    InternalVertex(HashMap<String, Tree>),
    ExternalVertex,
}

impl Tree {
    pub fn from(paths: Vec<Path>) -> Self {
        let mut parents = HashMap::new();

        for path in paths {
            match path {
                Path::InternalVertex(name, child) => {
                    parents
                        .entry(name.clone())
                        .or_insert_with(Vec::new)
                        .push(*child);
                }
                Path::ExternalVertex(name) => {
                    parents.insert(name.clone(), Vec::new());
                }
            }
        }

        if parents.contains_key("*") {
            let all = parents["*"].clone();
            for value in parents.values_mut() {
                value.append(&mut all.clone());
            }
        }

        let mut output = HashMap::new();
        for (name, children) in parents {
            output.insert(
                name,
                if children.is_empty() {
                    Tree::ExternalVertex
                } else {
                    Tree::from(children)
                },
            );
        }

        Tree::InternalVertex(output)
    }

    pub fn walk(&self, path: &Path) -> Option<&Tree> {
        match self {
            Self::ExternalVertex => None,
            Self::InternalVertex(map) => match path {
                Path::InternalVertex(name, path) => map
                    .get(name)
                    .or_else(|| map.get("*"))
                    .and_then(|v| v.walk(path)),
                Path::ExternalVertex(name) => map.get(name).or_else(|| map.get("*")),
            },
        }
    }

    pub fn step(&self, name: &str) -> Option<&Tree> {
        match self {
            Self::ExternalVertex => None,
            Self::InternalVertex(map) => map.get(name).or_else(|| map.get("*")),
        }
    }

    pub fn next(&self) -> Option<&Tree> {
        match self {
            Self::ExternalVertex => None,
            Self::InternalVertex(map) => map.get("*"),
        }
    }
}
