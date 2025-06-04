pub mod project {
    pub const NAME: &'static str = "unimake";
    pub const NAME_SHORT: &'static str = "umk";
    pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
}

pub mod workspace {
    pub const FILE: &'static str = "unimake.py";
    pub const DIRECTORY: &'static str = ".unimake";
    pub const CACHE: &'static str = ".unimake/cache";
}

pub mod script {
    pub const SINGLE_NAME: &'static str = "unimake";
    pub const SINGLE_FILE: &'static str = super::workspace::FILE;

    pub const PROJECT_NAME: &'static str = "project";
    pub const PROJECT_FILE: &'static str = ".unimake/project.py";

    pub const CONFIG_NAME: &'static str = "config";
    pub const CONFIG_FILE: &'static str = ".unimake/config.py";

    pub const CLI_NAME: &'static str = "cli";
    pub const CLI_FILE: &'static str = ".unimake/cli.py";

    pub const LAYOUT_NAME: &'static str = "layout";
    pub const LAYOUT_FILE: &'static str = ".unimake/layout.py";

    pub const LIST: [(&str, &str); 4] = [
        (CONFIG_NAME, CONFIG_FILE),
        (LAYOUT_NAME, LAYOUT_FILE),
        (PROJECT_NAME, PROJECT_FILE),
        (CLI_NAME, CLI_FILE),
    ];
}

pub mod state {
    pub const CONTAINER: &'static str = "__unimake__";
}
