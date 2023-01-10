use std::sync::Arc;

use colored::Colorize;
use disma::diff::Diff;

pub struct DiffFormater {}
pub type DiffFormaterRef = Arc<DiffFormater>;

impl DiffFormater {
    pub fn new() -> Self {
        Self {}
    }

    pub fn format(&self, diff: &Diff) -> String {
        self.format_with_indent(0, diff)
    }

    fn format_with_indent(&self, indent: usize, diff: &Diff) -> String {
        let mut text = String::new();

        match diff {
            Diff::Add(desc) => {
                let string = self.indent_lines(" + ", indent, desc).green().to_string();
                text.push_str(&string);
            }
            Diff::Remove(desc) => {
                let string = self.indent_lines(" - ", indent, desc).red().to_string();
                text.push_str(&string);
            }
            Diff::Update(desc, diffs) => {
                text.push_str(&self.indent_lines("   ", indent, &format!("{desc}:")));
                for diff in diffs {
                    text.push_str(&self.format_with_indent(indent + 2, diff));
                }
            }
        }

        text
    }

    fn indent_lines(&self, prefix: &str, indent: usize, text: &str) -> String {
        let indent_text = " ".repeat(indent);

        text.split('\n')
            .map(|line| format!("{prefix}{indent_text}{line}\n"))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use colored::Colorize;
    use disma::diff::Diff;

    use super::DiffFormater;

    #[test]
    fn can_format_additions() {
        let formatter = DiffFormater::new();
        let diff = Diff::Add("Something".into());

        let formatted = formatter.format(&diff);

        assert_eq!(formatted, " + Something\n".green().to_string());
    }

    #[test]
    fn can_format_additions_multiline() {
        let formatter = DiffFormater::new();
        let diff = Diff::Add("Something\nnew".into());

        let formatted = formatter.format(&diff);

        assert_eq!(formatted, " + Something\n + new\n".green().to_string());
    }

    #[test]
    fn can_format_removals() {
        let formatter = DiffFormater::new();
        let diff = Diff::Remove("Something".into());

        let formatted = formatter.format(&diff);

        assert_eq!(formatted, " - Something\n".red().to_string());
    }

    #[test]
    fn can_format_removals_multiline() {
        let formatter = DiffFormater::new();
        let diff = Diff::Remove("Something\nnew".into());

        let formatted = formatter.format(&diff);

        assert_eq!(formatted, " - Something\n - new\n".red().to_string());
    }

    #[test]
    fn can_format_updates() {
        let formatter = DiffFormater::new();
        let diff = Diff::Update("Something".into(), vec![]);

        let formatted = formatter.format(&diff);

        assert_eq!(formatted, "   Something:\n");
    }

    #[test]
    fn can_format_updates_multiline() {
        let formatter = DiffFormater::new();
        let diff = Diff::Update("Something\nnew".into(), vec![]);

        let formatted = formatter.format(&diff);

        assert_eq!(formatted, "   Something\n   new:\n");
    }

    #[test]
    fn can_format_recursive_updates() {
        let formatter = DiffFormater::new();
        let diff = Diff::Update(
            "Something".into(),
            vec![
                Diff::Add("Another".into()),
                Diff::Update("Yet".into(), vec![Diff::Remove("Wow".into())]),
            ],
        );

        let formatted = formatter.format(&diff);

        let expected_text = format!(
            "{}{}{}{}",
            "   Something:\n",
            " +   Another\n".green(),
            "     Yet:\n",
            " -     Wow\n".red()
        );
        assert_eq!(formatted, expected_text);
    }

    #[test]
    fn can_format_recursive_updates_multiline() {
        let formatter = DiffFormater::new();
        let diff = Diff::Update(
            "Something\nnew".into(),
            vec![
                Diff::Add("Another\nthing".into()),
                Diff::Update(
                    "Yet\npossible".into(),
                    vec![Diff::Remove("Wow\nnice".into())],
                ),
            ],
        );

        let formatted = formatter.format(&diff);

        let expected_text = format!(
            "{}{}{}{}",
            concat!("   Something\n", "   new:\n"),
            concat!(" +   Another\n", " +   thing\n").green(),
            concat!("     Yet\n", "     possible:\n"),
            concat!(" -     Wow\n", " -     nice\n").red()
        );
        assert_eq!(formatted, expected_text);
    }
}
