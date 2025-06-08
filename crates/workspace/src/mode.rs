use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Mode {
    Single,
    Tree,
}

impl Mode {
    pub fn from(root: &PathBuf) -> Option<Mode> {
        let path = root.join(global::workspace::FILE);
        if path.exists() {
            return Some(Mode::Single);
        }
        let path = root.join(global::workspace::DIRECTORY);
        if path.exists() {
            return Some(Mode::Tree);
        }
        None
    }
}
