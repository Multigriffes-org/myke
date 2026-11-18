use std::path::PathBuf;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourcePath(PathBuf);
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct HeaderPath(PathBuf);
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ObjectPath(PathBuf);
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BinaryPath(PathBuf);

pub trait Path {
    fn new(path: PathBuf) -> Self;
    fn path(&self) -> &PathBuf;
    fn path_mut(&mut self) -> &mut PathBuf;
}

impl Path for ObjectPath {
    fn new(path: PathBuf) -> Self {
        Self { 0: path }
    }
    fn path(&self) -> &PathBuf {
        &self.0
    }
    fn path_mut(&mut self) -> &mut PathBuf {
        &mut self.0
    }
}
impl Path for HeaderPath {
    fn new(path: PathBuf) -> Self {
        Self { 0: path }
    }
    fn path(&self) -> &PathBuf {
        &self.0
    }
    fn path_mut(&mut self) -> &mut PathBuf {
        &mut self.0
    }
}
impl Path for BinaryPath {
    fn new(path: PathBuf) -> Self {
        Self { 0: path }
    }
    fn path(&self) -> &PathBuf {
        &self.0
    }
    fn path_mut(&mut self) -> &mut PathBuf {
        &mut self.0
    }
}
impl Path for SourcePath {
    fn new(path: PathBuf) -> Self {
        Self { 0: path }
    }
    fn path(&self) -> &PathBuf {
        &self.0
    }
    fn path_mut(&mut self) -> &mut PathBuf {
        &mut self.0
    }
}

impl From<PathBuf> for SourcePath {
    fn from(value: PathBuf) -> Self {
        Self { 0: value }
    }
}
impl From<PathBuf> for HeaderPath {
    fn from(value: PathBuf) -> Self {
        Self { 0: value }
    }
}
impl From<PathBuf> for BinaryPath {
    fn from(value: PathBuf) -> Self {
        Self { 0: value }
    }
}
impl From<PathBuf> for ObjectPath {
    fn from(value: PathBuf) -> Self {
        Self { 0: value }
    }
}
