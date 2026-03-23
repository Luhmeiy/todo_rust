use reedline::{PromptEditMode, PromptHistorySearch};
use std::borrow::Cow;

pub struct TodoPrompt {
    left: String,
}

impl TodoPrompt {
    pub fn new(left: String) -> Self {
        Self { left }
    }
}

impl reedline::Prompt for TodoPrompt {
    fn render_prompt_left(&self) -> Cow<'_, str> {
        Cow::Owned(self.left.clone())
    }

    fn render_prompt_right(&self) -> Cow<'_, str> {
        Cow::Borrowed("")
    }

    fn render_prompt_indicator(&self, _: PromptEditMode) -> Cow<'_, str> {
        Cow::Borrowed("")
    }

    fn render_prompt_multiline_indicator(&self) -> Cow<'_, str> {
        Cow::Borrowed("... ")
    }

    fn render_prompt_history_search_indicator(&self, _: PromptHistorySearch) -> Cow<'_, str> {
        Cow::Borrowed("")
    }
}
