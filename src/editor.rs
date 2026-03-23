use crate::command::{ALL_COMMANDS, ALLOWED_EMPTY};
use nu_ansi_term::{Color, Style};
use reedline::{
    ColumnarMenu, Completer, Emacs, KeyCode, KeyModifiers, MenuBuilder, Reedline, ReedlineEvent,
    ReedlineMenu, Span, Suggestion, default_emacs_keybindings,
};

struct TodoCompleter {
    commands: Vec<String>,
    allowed_empty: Vec<String>,
    alias_subcommands: Vec<String>,
    list_exists: bool,
}

impl TodoCompleter {
    fn new(list_exists: bool) -> Self {
        Self {
            commands: ALL_COMMANDS.iter().map(|&s| s.to_string()).collect(),
            allowed_empty: ALLOWED_EMPTY.iter().map(|&s| s.to_string()).collect(),
            alias_subcommands: vec![
                "add".to_string(),
                "list".to_string(),
                "remove".to_string(),
                "rename".to_string(),
                "path".to_string(),
            ],
            list_exists,
        }
    }

    fn calculate_span(line: &str, word: &str) -> Span {
        let start = line.rfind(word).unwrap_or(line.len());
        Span::new(start, start + word.len())
    }

    fn needs_prefix(line: &str, prefix: &str) -> bool {
        !line.is_empty() && !line.ends_with(' ') && prefix.is_empty()
    }

    fn complete_from(commands: &Vec<String>, line: &str, prefix: &str) -> Vec<Suggestion> {
        let add_prefix = Self::needs_prefix(line, prefix);

        commands
            .iter()
            .filter(|cmd| cmd.starts_with(prefix))
            .map(|cmd| {
                let value = if add_prefix {
                    format!(" {cmd} ")
                } else {
                    format!("{cmd} ")
                };

                Suggestion {
                    value,
                    display_override: Some(cmd.clone()),
                    span: Self::calculate_span(line, prefix),
                    ..Default::default()
                }
            })
            .collect()
    }
}

impl Completer for TodoCompleter {
    fn complete(&mut self, line: &str, _pos: usize) -> Vec<Suggestion> {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.is_empty() {
            if self.list_exists {
                return TodoCompleter::complete_from(&self.commands, line, "");
            } else {
                return TodoCompleter::complete_from(&self.allowed_empty, line, "");
            }
        }

        if !self.commands.contains(&parts[0].to_string()) {
            return TodoCompleter::complete_from(&self.commands, line, parts[0]);
        }

        match parts.len() {
            1 => match parts[0] {
                "alias" => TodoCompleter::complete_from(&self.alias_subcommands, line, ""),
                "help" => TodoCompleter::complete_from(&self.commands, line, ""),
                _ => vec![],
            },
            _ => {
                if parts[0] == "alias" && self.alias_subcommands.contains(&parts[1].to_string()) {
                    return vec![];
                }

                if parts[0] == "help" && parts[1] == "help" {
                    return vec![];
                }

                if !self.commands.contains(&parts[1].to_string()) {
                    let last_word = parts.last().copied().unwrap_or("");
                    match parts[0] {
                        "alias" => {
                            return TodoCompleter::complete_from(
                                &self.alias_subcommands,
                                line,
                                last_word,
                            );
                        }
                        "help" => {
                            return TodoCompleter::complete_from(&self.commands, line, last_word);
                        }
                        _ => return vec![],
                    }
                }

                if parts.len() == 2 {
                    match parts[1] {
                        "alias" => {
                            return TodoCompleter::complete_from(&self.alias_subcommands, line, "");
                        }
                        _ => return vec![],
                    }
                } else if parts.len() > 2 {
                    if self.alias_subcommands.contains(&parts[2].to_string()) {
                        return vec![];
                    }

                    let last_word = parts.last().copied().unwrap_or("");

                    match parts[1] {
                        "alias" => {
                            return TodoCompleter::complete_from(
                                &self.alias_subcommands,
                                line,
                                last_word,
                            );
                        }
                        _ => return vec![],
                    }
                } else {
                    vec![]
                }
            }
        }
    }
}

pub fn create_editor(list_exists: bool) -> Reedline {
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
        .with_completer(Box::new(TodoCompleter::new(list_exists)))
        .with_menu(ReedlineMenu::EngineCompleter(completion_menu))
        .with_edit_mode(Box::new(Emacs::new(keybindings)))
        .with_quick_completions(true)
}
