use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct History {
    state: String,
}

impl History {
    pub fn new(state: String) -> Self {
        Self { state }
    }

    pub fn pop(self) -> String {
        self.state
    }
}
