pub struct Draft___ { content_: String }
pub struct P_Review { content_: String }
pub struct P_lished { content_: String }

impl Draft___ { // l-ord
    pub fn new_() -> Draft___ {
        Draft___ { content_: String::new() }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content_.push_str(text);
    }

    pub fn r_review(self) -> P_Review {
        P_Review { content_: self.content_ }
    }
}

impl P_Review {
    pub fn approve_(self) -> P_lished {
        P_lished { content_: self.content_ }
    }
}

impl P_lished { // l-ord
    pub fn content_(&self) -> &str {
        &self.content_
    }
}
