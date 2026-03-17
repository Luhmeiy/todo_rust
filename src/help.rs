pub const GENERAL: &str = r#"
List Management:
  mklist <title>         Create a new list
  lists                  Display all lists
  switch <id|title>      Switch to another list
  rmlist <id|title>      Remove a list
  rename [id] <title>    Rename a list by ID (current if no id)

Current List Tasks:
  add <task>             Add a task
  list                   Display all tasks
  update <id> <desc>     Update a task by ID
  check <id|desc>        Mark done
    --all                Mark all tasks as done
  uncheck <id|desc>      Mark not done
    --all                Mark all tasks as not done
  delete <id|desc>       Delete a task
    --all                Delete all tasks
    --checked            Delete checked tasks
    --unchecked          Delete unchecked tasks

Other:
  save <path>           Save to a custom location
  help [command]         Display all commands or details for a specific command
  exit                   Exit the program
"#;

pub const MKLIST: &str = r#"
mklist <title>
    Create a new list with the given title
    Example: mklist Work
"#;

pub const LISTS: &str = r#"
lists
    Display all existing lists
    Example: lists
"#;

pub const SWITCH: &str = r#"
switch <id|title>
    Switch to a different list by ID or title
    Examples:
      switch 2
      switch Work
"#;

pub const RMLIST: &str = r#"
rmlist <id|title>
    Remove a list by ID or title
    Examples:
      rmlist 2
      rmlist Work
"#;

pub const RENAME: &str = r#"
rename [id] <new title>
    Rename a list by ID
    If only one argument is given, rename the current list
    Examples:
      rename 3 Shopping  # rename list with ID 3 to "Shopping"
      rename Projects    # rename current list to "Projects"
"#;

pub const ADD: &str = r#"
add <task description>
    Add a new task to the current list
    Example: add Buy milk
"#;

pub const LIST: &str = r#"
list
    Display all tasks in the current list with their completion status
    Example: list
"#;

pub const UPDATE: &str = r#"
update <id> <new description>
    Update an existing task description by ID
    Example: update 2 Buy organic milk
"#;

pub const CHECK: &str = r#"
check [--all | <id|description>]
    Mark task(s) as done
    • --all              : check all tasks in the current list
    • <id>               : check task by ID
    • <description>      : check task by description; if multiple match, choose from list
    Examples:
      check --all
      check 3
      check Buy milk
"#;

pub const UNCHECK: &str = r#"
uncheck [--all | <id|description>]
    Mark task(s) as not done
    • --all              : uncheck all tasks in the current list
    • <id>               : uncheck task by ID
    • <description>      : uncheck task by description; if multiple match, choose from list
    Examples:
      uncheck --all
      uncheck 5
      uncheck Finish report
"#;

pub const DELETE: &str = r#"
delete [--all | --checked | --unchecked | <id|description>]
    Delete task(s)
    • --all              : delete all tasks in the current list
    • --checked          : delete completed tasks
    • --unchecked        : delete incomplete tasks
    • <id>               : delete task by ID
    • <description>      : delete task by description; if multiple match, choose from list
    Examples:
      delete --checked
      delete 4
      delete Buy milk
"#;

pub const SAVE: &str = r#"
save <path>
    Save data to a custom file path
    Example: save ./my_backup.json
"#;

pub const HELP: &str = r#"
help [command]
    Display general help with all commands or detailed help for a specific command
    Examples:
      help               # Display general help
      help add           # Display detailed help for the 'add' command
"#;

pub const EXIT: &str = r#"
exit
    Exit the application
"#;

pub fn for_command(cmd: &str) -> Option<&'static str> {
    match cmd {
        "mklist" => Some(MKLIST),
        "lists" => Some(LISTS),
        "switch" => Some(SWITCH),
        "rmlist" => Some(RMLIST),
        "rename" => Some(RENAME),
        "add" => Some(ADD),
        "list" => Some(LIST),
        "update" => Some(UPDATE),
        "check" => Some(CHECK),
        "uncheck" => Some(UNCHECK),
        "delete" => Some(DELETE),
        "save" => Some(SAVE),
        "help" => Some(HELP),
        "exit" => Some(EXIT),
        _ => None,
    }
}
