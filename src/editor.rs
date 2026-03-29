use crate::command_info;
use nu_ansi_term::{Color, Style};
use reedline::{
    ColumnarMenu, Completer, Emacs, KeyCode, KeyModifiers, MenuBuilder, Reedline, ReedlineEvent,
    ReedlineMenu, Span, Suggestion, default_emacs_keybindings,
};

struct TodoCompleter {
    list_exists: bool,
    aliases: Vec<String>,
    commands: Vec<&'static str>,
    allowed_empty: Vec<&'static str>,
}

impl TodoCompleter {
    fn new(list_exists: bool, aliases: Vec<String>) -> Self {
        Self {
            list_exists,
            aliases,
            commands: command_info::all_commands().map(|c| c.name).collect(),
            allowed_empty: command_info::allowed_empty().map(|c| c.name).collect(),
        }
    }

    fn calculate_span(line: &str, word: &str) -> Span {
        let start = line.rfind(word).unwrap_or(line.len());
        Span::new(start, start + word.len())
    }

    fn needs_prefix(line: &str, prefix: &str) -> bool {
        !line.is_empty() && !line.ends_with(' ') && prefix.is_empty()
    }

    fn complete_from<T: AsRef<str>>(items: &[T], line: &str, prefix: &str) -> Vec<Suggestion> {
        let add_prefix = Self::needs_prefix(line, prefix);

        items
            .iter()
            .filter(|item| item.as_ref().starts_with(prefix))
            .map(|item| {
                let item_str = item.as_ref();
                let value = if add_prefix {
                    format!(" {item_str} ")
                } else {
                    format!("{item_str} ")
                };

                Suggestion {
                    value,
                    display_override: Some(item_str.to_string()),
                    span: Self::calculate_span(line, prefix),
                    ..Default::default()
                }
            })
            .collect()
    }

    fn is_command(word: &str) -> bool {
        command_info::get(word).is_some()
    }

    fn is_already_used(cmd: &str, word: &str) -> bool {
        let subcommands = command_info::get_subcommand_names(cmd);
        let flags = command_info::get_flags(cmd);

        subcommands.contains(&word) || flags.contains(&word)
    }

    fn complete_command(&self, cmd: &str, line: &str, prefix: &str) -> Vec<Suggestion> {
        let subcommands = command_info::get_subcommand_names(cmd);
        let flags = command_info::get_flags(cmd);

        if !subcommands.is_empty() {
            Self::complete_from(&subcommands, line, prefix)
        } else if !flags.is_empty() {
            Self::complete_from(flags, line, prefix)
        } else if matches!(cmd, "save" | "load") {
            Self::complete_from(&self.aliases, line, prefix)
        } else if cmd == "help" {
            Self::complete_from(&self.commands, line, prefix)
        } else {
            vec![]
        }
    }
}

impl Completer for TodoCompleter {
    fn complete(&mut self, line: &str, _pos: usize) -> Vec<Suggestion> {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.is_empty() {
            if self.list_exists {
                return Self::complete_from(&self.commands, line, "");
            } else {
                return Self::complete_from(&self.allowed_empty, line, "");
            }
        }

        if !Self::is_command(parts[0]) {
            return Self::complete_from(&self.commands, line, parts[0]);
        }

        match parts.len() {
            1 => self.complete_command(parts[0], line, ""),
            _ => {
                if Self::is_already_used(parts[0], parts[1]) {
                    return vec![];
                }

                if matches!(parts[0], "save" | "load")
                    && self.aliases.contains(&parts[1].to_string())
                {
                    return vec![];
                }

                if Self::is_command(parts[1]) {
                    if parts.len() > 2 && Self::is_already_used(parts[1], parts[2]) {
                        return vec![];
                    }

                    let prefix = if parts.len() > 2 {
                        parts.last().copied().unwrap_or("")
                    } else {
                        ""
                    };

                    return self.complete_command(parts[1], line, prefix);
                }

                let last_word = parts.last().copied().unwrap_or("");
                self.complete_command(parts[0], line, last_word)
            }
        }
    }
}

pub fn create_editor(list_exists: bool, aliases: Vec<String>) -> Reedline {
    let completion_menu = Box::new(
        ColumnarMenu::default()
            .with_name("completion_menu")
            .with_marker("")
            .with_selected_text_style(Style::new().fg(Color::Cyan))
            .with_selected_match_text_style(Style::new().fg(Color::Cyan).underline()),
    );

    let mut keybindings = default_emacs_keybindings();
    keybindings.add_binding(
        KeyModifiers::NONE,
        KeyCode::Tab,
        ReedlineEvent::UntilFound(vec![
            ReedlineEvent::Menu("completion_menu".to_string()),
            ReedlineEvent::Enter,
        ]),
    );

    Reedline::create()
        .with_completer(Box::new(TodoCompleter::new(list_exists, aliases)))
        .with_menu(ReedlineMenu::EngineCompleter(completion_menu))
        .with_edit_mode(Box::new(Emacs::new(keybindings)))
        .with_quick_completions(true)
}
