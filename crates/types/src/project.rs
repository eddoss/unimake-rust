use std::collections::HashMap;
use std::path::Path;
use crate::util;

pub struct Contributor {
    pub name: String,
    pub email: Vec<String>,
    pub socials: HashMap<String, String>
}

pub struct Info {
    pub id: String,
    pub version: String,
    pub name: String,
    pub description: String,
    pub contributors: Vec<Contributor>,
}

trait Layout {
    fn root(&self) -> &Path;
    fn unimake(&self) -> &Path;
    fn get(&self, name: &str) -> util::Res<&Path>;
}

trait Interface {
    fn info(&self) -> Info;
    fn layout(&self) -> impl Layout;
}

pub mod go {
    use std::path::Path;
    use crate::util::Res;

    struct Layout {
        root: Path,
        unimake: Path,
    }

    impl crate::project::Layout for Layout {
        fn root(&self) -> &Path {
            &self.root
        }

        fn unimake(&self) -> &Path {
            &self.unimake
        }

        fn get(&self, name: &str) -> Res<&Path> {
            Ok(&self.root.join("assets"))
        }
    }

    impl Layout {
        pub fn assets(&self) -> &Path {
            &self.root.join("assets")
        }

        pub fn build(&self) -> &Path {
            &self.root.join("build")
        }

        pub fn cmd(&self) -> &Path {
            &self.root.join("cmd")
        }

        pub fn configs(&self) -> &Path {
            &self.root.join("configs")
        }

        pub fn deployment(&self) -> &Path {
            &self.root.join("deployment")
        }

        pub fn docs(&self) -> &Path {
            &self.root.join("docs")
        }

        pub fn examples(&self) -> &Path {
            &self.root.join("examples")
        }

        pub fn githooks(&self) -> &Path {
            &self.root.join("githooks")
        }

        pub fn init(&self) -> &Path {
            &self.root.join("init")
        }

        pub fn internal(&self) -> &Path {
            &self.root.join("internal")
        }

        pub fn pkg(&self) -> &Path {
            &self.root.join("pkg")
        }

        pub fn scripts(&self) -> &Path {
            &self.root.join("scri")
        }

        pub fn test(&self) -> &Path {
            &self.root.join("test")
        }

        pub fn third_party(&self) -> &Path {
            &self.root.join("third_party")
        }

        pub fn tools(&self) -> &Path {
            &self.root.join("tools")
        }

        pub fn vendor(&self) -> &Path {
            &self.root.join("vendor")
        }

        pub fn web(&self) -> &Path {
            &self.root.join("web")
        }

        pub fn website(&self) -> &Path {
            &self.root.join("website")
        }

        pub fn output(&self) -> &Path {
            &self.root.join("output")
        }
    }
}