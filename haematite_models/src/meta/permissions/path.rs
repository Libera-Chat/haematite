#[derive(Clone, Debug)]
pub enum Path {
    InternalVertex(String, Box<Path>),
    ExternalVertex(String),
}

impl Path {
    fn from_with_depth(path: &str, depth: u8) -> Self {
        if let Some((parent, child)) = path.split_once('/') {
            let child = if depth < 8 {
                Self::from_with_depth(child, depth + 1)
            } else {
                Self::ExternalVertex(child.to_string())
            };
            Self::InternalVertex(parent.to_string(), Box::new(child))
        } else {
            Self::ExternalVertex(path.to_string())
        }
    }

    pub fn from(path: &str) -> Self {
        Self::from_with_depth(path, 0)
    }

    pub fn walk(&self, other: &Path) -> Option<&Path> {
        match other {
            Self::ExternalVertex(other_name) => {
                let (name, vertex) = match self {
                    Self::InternalVertex(name, child) => (name, &**child),
                    Self::ExternalVertex(name) => (name, self),
                };

                (other_name == name).then(|| vertex)
            }
            Self::InternalVertex(other_name, other_child) => {
                if let Self::InternalVertex(name, child) = self {
                    if other_name == name {
                        return child.walk(other_child);
                    }
                }
                None
            }
        }
    }
}

impl ToString for Path {
    fn to_string(&self) -> String {
        match self {
            Self::InternalVertex(name, child) => format!("{}/{}", name, child.as_ref().to_string()),
            Self::ExternalVertex(name) => name.clone(),
        }
    }
}
