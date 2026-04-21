use crate::help;

pub struct SubcommandInfo {
    pub name: &'static str,
    pub help: &'static str,
}

pub struct CommandInfo {
    pub name: &'static str,
    pub subcommands: &'static [SubcommandInfo],
    pub flags: &'static [&'static str],
    pub requires_list: bool,
    pub help: &'static str,
}

pub const COMMANDS: &[CommandInfo] = &[
    CommandInfo {
        name: "mklist",
        subcommands: &[],
        flags: &[],
        requires_list: false,
        help: help::MKLIST,
    },
    CommandInfo {
        name: "lists",
        subcommands: &[],
        flags: &[],
        requires_list: false,
        help: help::LISTS,
    },
    CommandInfo {
        name: "switch",
        subcommands: &[],
        flags: &[],
        requires_list: true,
        help: help::SWITCH,
    },
    CommandInfo {
        name: "rmlist",
        subcommands: &[],
        flags: &[],
        requires_list: true,
        help: help::RMLIST,
    },
    CommandInfo {
        name: "rename",
        subcommands: &[],
        flags: &[],
        requires_list: true,
        help: help::RENAME,
    },
    CommandInfo {
        name: "add",
        subcommands: &[],
        flags: &["--priority", "-p", "--due", "-d"],
        requires_list: true,
        help: help::ADD,
    },
    CommandInfo {
        name: "list",
        subcommands: &[],
        flags: &[],
        requires_list: true,
        help: help::LIST,
    },
    CommandInfo {
        name: "dues",
        subcommands: &[],
        flags: &[],
        requires_list: true,
        help: help::DUES,
    },
    CommandInfo {
        name: "update",
        subcommands: &[],
        flags: &[],
        requires_list: true,
        help: help::UPDATE,
    },
    CommandInfo {
        name: "due",
        subcommands: &[],
        flags: &["--remove"],
        requires_list: true,
        help: help::DUE,
    },
    CommandInfo {
        name: "priority",
        subcommands: &[],
        flags: &["--remove"],
        requires_list: true,
        help: help::PRIORITY,
    },
    CommandInfo {
        name: "check",
        subcommands: &[],
        flags: &["--all"],
        requires_list: true,
        help: help::CHECK,
    },
    CommandInfo {
        name: "uncheck",
        subcommands: &[],
        flags: &["--all"],
        requires_list: true,
        help: help::UNCHECK,
    },
    CommandInfo {
        name: "delete",
        subcommands: &[],
        flags: &["--all", "--checked", "--unchecked"],
        requires_list: true,
        help: help::DELETE,
    },
    CommandInfo {
        name: "save",
        subcommands: &[],
        flags: &[],
        requires_list: false,
        help: help::SAVE,
    },
    CommandInfo {
        name: "load",
        subcommands: &[],
        flags: &[],
        requires_list: false,
        help: help::LOAD,
    },
    CommandInfo {
        name: "alias",
        subcommands: &[
            SubcommandInfo {
                name: "add",
                help: help::ALIAS_ADD,
            },
            SubcommandInfo {
                name: "list",
                help: help::ALIAS_LIST,
            },
            SubcommandInfo {
                name: "remove",
                help: help::ALIAS_REMOVE,
            },
            SubcommandInfo {
                name: "rename",
                help: help::ALIAS_RENAME,
            },
            SubcommandInfo {
                name: "path",
                help: help::ALIAS_PATH,
            },
        ],
        flags: &[],
        requires_list: false,
        help: help::ALIAS,
    },
    CommandInfo {
        name: "help",
        subcommands: &[],
        flags: &[],
        requires_list: false,
        help: help::HELP,
    },
    CommandInfo {
        name: "exit",
        subcommands: &[],
        flags: &[],
        requires_list: false,
        help: help::EXIT,
    },
];

pub fn get(name: &str) -> Option<&'static CommandInfo> {
    COMMANDS.iter().find(|c| c.name == name)
}

pub fn get_subcommand_names(name: &str) -> Vec<&str> {
    get(name).map_or(vec![], |c| c.subcommands.iter().map(|s| s.name).collect())
}

pub fn get_subcommand_info(cmd: &str, subcmd: &str) -> Option<&'static SubcommandInfo> {
    get(cmd)?.subcommands.iter().find(|s| s.name == subcmd)
}

pub fn get_subcommand_help(cmd: &str, subcmd: &str) -> Option<&'static str> {
    get_subcommand_info(cmd, subcmd).map(|s| s.help)
}

pub fn get_flags(name: &str) -> &'static [&'static str] {
    get(name).map_or(&[], |c| c.flags)
}

pub fn get_help(name: &str) -> Option<&'static str> {
    get(name).map(|c| c.help)
}

pub fn requires_list(name: &str) -> bool {
    get(name).map_or(false, |c| c.requires_list)
}

pub fn all_commands() -> impl Iterator<Item = &'static CommandInfo> {
    COMMANDS.iter()
}

pub fn allowed_empty() -> impl Iterator<Item = &'static CommandInfo> {
    COMMANDS.iter().filter(|c| !c.requires_list)
}
