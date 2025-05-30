pub const PROJECT_NAME: &'static str = "unimake";
pub const PROJECT_NAME_SHORT: &'static str = "umk";
pub const PROJECT_VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub const WORKSPACE_FILE: &'static str = "unimake.py";
pub const WORKSPACE_DIRECTORY: &'static str = ".unimake";
pub const WORKSPACE_CACHE: &'static str = ".unimake/cache";

pub const SINGLE_SCRIPT_NAME: &'static str = "unimake";
pub const SINGLE_SCRIPT_FILE: &'static str = WORKSPACE_FILE;

pub const SCRIPT_PROJECT_NAME: &'static str = "project";
pub const SCRIPT_PROJECT_FILE: &'static str = ".unimake/project.py";

pub const SCRIPT_CONFIG_NAME: &'static str = "config";
pub const SCRIPT_CONFIG_FILE: &'static str = ".unimake/config.py";

pub const SCRIPT_CLI_NAME: &'static str = "cli";
pub const SCRIPT_CLI_FILE: &'static str = ".unimake/cli.py";

pub const SCRIPT_LAYOUT_NAME: &'static str = "layout";
pub const SCRIPT_LAYOUT_FILE: &'static str = ".unimake/layout.py";

pub const SCRIPTS: [(&str, &str); 4] = [
    (SCRIPT_CONFIG_NAME, SCRIPT_CONFIG_FILE),
    (SCRIPT_LAYOUT_NAME, SCRIPT_LAYOUT_FILE),
    (SCRIPT_PROJECT_NAME, SCRIPT_PROJECT_FILE),
    (SCRIPT_CLI_NAME, SCRIPT_CLI_FILE),
];

pub const STATES_OBJECT_NAME: &'static str = "__unimake__";
