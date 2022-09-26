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
}

impl Differ {
    pub fn new(diff_mode: DiffMode) -> Self {
        Differ { diff_mode }
    }

    pub fn generate(&self, a: String, b: String) -> (bool, String) {
        let diff_mode_string = self.diff_mode.to_string();
        let diff_mode_str = diff_mode_string.as_str();
        let (dist, changeset) = diff(&a, &b, diff_mode_str);
        if dist == 0 {
            return (false, "".to_string());
        }

        let mut in_diff = false;
        let mut output = String::new();
        let mut line_count = 1;

        for seq in changeset {
            match seq {
                text_diff::Difference::Same(s) => {
                    line_count += 1;
                    output.push_str(&s);
                    output.push_str(diff_mode_str);
                }
                text_diff::Difference::Add(s) => {
                    (in_diff, line_count) = diff_prompt(in_diff, &mut output, line_count);

                    output.push_str("+");
                    output.push_str(&s);
                    output.push_str("+");
                    output.push_str(diff_mode_str);
                }
                text_diff::Difference::Rem(s) => {
                    (in_diff, line_count) = diff_prompt(in_diff, &mut output, line_count);

                    output.push_str("-");
                    output.push_str(&s);
                    output.push_str("-");
                    output.push_str(diff_mode_str);
                }
            }
        }

        (true, output)
    }
}

fn diff_prompt(mut in_diff: bool, output: &mut String, mut line_count: i32) -> (bool, i32) {
    if !in_diff {
        line_count += 1;
        output.push_str(format!(">>> a (line: {}): ", line_count).as_str());
        in_diff = true;
    } else {
        output.push_str(format!(">>> b (line: {}): ", line_count).as_str());
        in_diff = false;
    }
    (in_diff, line_count)
}
