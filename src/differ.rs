use text_diff::diff;

pub enum DiffMode {
    Line,
    Word,
    Char,
}

impl DiffMode {
    fn to_string(&self) -> String {
        match self {
            DiffMode::Line => "\n".to_string(),
            DiffMode::Word => " ".to_string(),
            DiffMode::Char => "".to_string(),
        }
    }
}

pub struct Differ {
    diff_mode: DiffMode,

    in_diff: bool,
    output: String,
    line_count: i32,
}

impl Differ {
    pub fn new(diff_mode: DiffMode) -> Self {
        Differ {
            diff_mode,
            in_diff: false,
            output: String::new(),
            line_count: 0,
        }
    }

    fn diff_prompt(&mut self) {
        if !self.in_diff {
            self.line_count += 1;
            self.output
                .push_str(format!(">>> a (line: {}): ", self.line_count).as_str());
            self.in_diff = true;
        } else {
            self.output
                .push_str(format!(">>> b (line: {}): ", self.line_count).as_str());
            self.in_diff = false;
        }
    }

    pub fn generate(&mut self, a: String, b: String) -> String {
        let diff_mode_string = self.diff_mode.to_string();
        let diff_mode_str = diff_mode_string.as_str();
        let (dist, changeset) = diff(&a, &b, diff_mode_str);
        if dist == 0 {
            return "no difference".to_string();
        }

        for seq in changeset {
            match seq {
                text_diff::Difference::Same(s) => {
                    self.line_count += 1;
                    self.output.push_str(&s);
                    self.output.push_str(diff_mode_str);
                }
                text_diff::Difference::Add(s) => {
                    self.diff_prompt();

                    self.output.push_str("+");
                    self.output.push_str(&s);
                    self.output.push_str("+");
                    self.output.push_str(diff_mode_str);
                }
                text_diff::Difference::Rem(s) => {
                    self.diff_prompt();

                    self.output.push_str("-");
                    self.output.push_str(&s);
                    self.output.push_str("-");
                    self.output.push_str(diff_mode_str);
                }
            }
        }

        self.output.clone()
    }
}
