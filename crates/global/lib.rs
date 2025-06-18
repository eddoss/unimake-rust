pub mod project {
    pub const NAME: &'static str = "unimake";
    pub const NAME_SHORT: &'static str = "umk";
    pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
}

pub mod workspace {
    pub const FILE: &'static str = "unimake.py";
    pub const SCRIPT: &'static str = "unimake";
    pub const DIRECTORY: &'static str = ".unimake";
    pub const CACHE: &'static str = ".unimake/.cache";
}

pub mod kit {
    pub const NAME: &'static str = "umk";
    pub const CONTAINER: &'static str = "__unimake__";
}
