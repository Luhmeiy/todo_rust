pub const GENERAL: &str = r#"
List Management:
  mklist <title>         Create a new list
  lists                  Display all lists
  switch <id|title>      Switch to another list
  rmlist <id|title>      Remove a list
  rename [id] <title>    Rename a list by ID (current if no id)

Current List Tasks:
  add <task>             Add a task
  list [flags]           Display tasks (flags: --checked, --unchecked, --priority, --due)
    --all                Display all tasks from all lists (combine with flags)
    <id|title>           Display tasks from a specific list by ID or name
  dues                   Display tasks with due dates
  update <id> <desc>     Update a task by ID
  due <id> date          View or add due date
    --remove             Remove due date
  priority <id> level    View or add priority
    --remove             Remove priority
  info <id>              Display all task information
  check <id|desc>        Mark done
    --all                Mark all tasks as done
  uncheck <id|desc>      Mark not done
    --all                Mark all tasks as not done
  delete <id|desc>       Delete a task
    --all                Delete all tasks
    --checked            Delete checked tasks
    --unchecked          Delete unchecked tasks

Other:
  save <@alias|path>     Save to an alias or custom location
  load <@alias|path>     Load from an alias or custom location
  alias                  Manage aliases with subcommands
    add <@name> <path>   Create an alias
    list                 List all aliases
    remove <@name>       Remove an alias
    rename <@old> <@new> Rename an alias
    path <@name> <path>  Update alias path
  undo                   Undo last command
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
add <task description> [--priority <level> | -p <level>] [--due <date> | -d <date>]
  Add a new task to the current list with optional priority and due date
  • --priority <level>, -p <level>   Priority level (low, medium, high)
  • --due <date>, -d <date>          Due date (DD-MM-YYYY)
  Examples:
    add Buy milk
    add "Finish report" -p high
    add "Call mom" --due 25-12-2026
    add "Project deadline" -p high -d 25-12-2026
    add "Meeting" --due 01-01-2027 -p medium
"#;

pub const LIST: &str = r#"
list [--all | <id|title>] [flags]
  Display tasks in a list with their completion status
  • <id|title>           Display tasks from a specific list by ID or name
  • --all, -a            Display tasks from all lists (can combine with flags)
  • --checked, -c        Only show checked tasks (AND logic with other flags)
  • --unchecked, -u      Only show unchecked tasks (AND logic with other flags)
  • --priority, -p       Only show tasks with priority (AND logic with other flags)
  • --due, -d            Only show tasks with due date (AND logic with other flags)
  Examples:
    list                 # Display all tasks in current list
    list --checked       # Display only checked tasks in current list
    list --unchecked     # Display only unchecked tasks in current list
    list -u -p           # Unchecked and priority tasks
    list --all --due     # Tasks with due dates across all lists
    list 2               # Display tasks from list ID 2
    list work            # Display tasks from list named "work"
"#;

pub const DUES: &str = r#"
dues
  Display incomplete tasks with due dates, sorted by nearest date
"#;

pub const UPDATE: &str = r#"
update <id> <new description>
  Update an existing task description by ID
  Example: update 2 Buy organic milk
"#;

pub const DUE: &str = r#"
due <id> [<due date> | --remove]
  View, add, or remove a due date from a task by ID
  • <due date>           Add due date (format based on config date-format)
  • --remove             Remove due date
  Examples:
    due 5                # View due date for task 5
    due 5 21-04-2026     # Add due date to task 5
    due 5 --remove       # Remove due date from task 5
"#;

pub const PRIORITY: &str = r#"
priority <id> [<level> | --remove]
  View, add, or remove priority from a task by ID
  • <level>              Priority level (low, medium, high)
  • --remove             Remove priority
  Examples:
    priority 5           # View priority for task 5
    priority 5 high      # Set priority to high for task 5
    priority 5 --remove  # Remove priority from task 5
"#;

pub const INFO: &str = r#"
info <id>
  Display all information for a task by ID
  Example: info 5
"#;

pub const CHECK: &str = r#"
check [--all | <id|description>]
  Mark task(s) as done
  • --all                Check all tasks in the current list
  • <id>                 Check task by ID
  • <description>        Check task by description; if multiple match, choose from list
  Examples:
    check --all
    check 3
    check Buy milk
"#;

pub const UNCHECK: &str = r#"
uncheck [--all | <id|description>]
  Mark task(s) as not done
  • --all                Uncheck all tasks in the current list
  • <id>                 Uncheck task by ID
  • <description>        Uncheck task by description; if multiple match, choose from list
  Examples:
    uncheck --all
    uncheck 5
    uncheck Finish report
"#;

pub const DELETE: &str = r#"
delete [--all | --checked | --unchecked | <id|description>]
  Delete task(s)
  • --all                Delete all tasks in the current list
  • --checked            Delete completed tasks
  • --unchecked          Delete incomplete tasks
  • <id>                 Delete task by ID
  • <description>        Delete task by description; if multiple match, choose from list
  Examples:
    delete --checked
    delete 4
    delete Buy milk
"#;

pub const SAVE: &str = r#"
save <@alias|path>
  Save data to an alias or custom file path
  Examples:
    save @work
    save ./my_backup.json
"#;

pub const LOAD: &str = r#"
load <@alias|path>
  Load data from an alias or custom file path
  Examples:
    load @work
    load ./my_backup.json
"#;

pub const ALIAS: &str = r#"
alias <subcommand>
  Manage aliases for file paths

  Subcommands:
    add <@name> <path>   Create an alias
    list                 List all aliases
    remove <@name>       Remove an alias
    rename <@old> <@new> Rename an alias
    path <@name> <path>  Update alias path

  Examples:
    alias add @work ./work.json
    alias list
    alias remove @work
    alias rename @work @job
    alias path @work ./new_work.json
"#;

pub const ALIAS_ADD: &str = r#"
alias add <@name> <path>
  Create an alias for a file path
  Example: alias add @work ./work.json
"#;

pub const ALIAS_LIST: &str = r#"
alias list
  List all saved aliases
"#;

pub const ALIAS_REMOVE: &str = r#"
alias remove <@name>
  Remove an alias
  Example: alias remove @work
"#;

pub const ALIAS_RENAME: &str = r#"
alias rename <@old> <@new>
  Rename an alias
  Example: alias rename @work @job
"#;

pub const ALIAS_PATH: &str = r#"
alias path <@name> <path>
  Update the path an alias points to
  Example: alias path @work ./new_work.json
"#;

pub const CONFIG: &str = r#"
config <subcommand> [args]
  Manage application configuration
  Subcommands:
    date-format          Change the date format
    list                 List current settings
"#;

pub const CONFIG_DATE_FORMAT: &str = r#"
config date-format <format>
  Change the date format used throughout the app
  Example: config date-format %d/%m/%Y
"#;

pub const CONFIG_LIST: &str = r#"
config list
  List current configuration settings
"#;

pub const HELP: &str = r#"
help [command]
  Display general help with all commands or detailed help for a specific command
  Examples:
    help                 # Display general help
    help add             # Display detailed help for the 'add' command
"#;

pub const EXIT: &str = r#"
exit
  Exit the application
"#;

pub const UNDO: &str = r#"
undo
  Undo the last command
  Example: undo
"#;
