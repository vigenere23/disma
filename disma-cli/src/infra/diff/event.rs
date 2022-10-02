use disma::diff::{base::Diff, event::DiffEventListener};

pub struct CliDiffEventListener {}

impl DiffEventListener for CliDiffEventListener {
    fn after_diff_executed(&self, diff: Diff) {
        match diff {
            Diff::Add(desc) => println!("● 🆕 Added {}", desc),
            Diff::Remove(desc) => println!("● 🗑️  Removed {}", desc),
            Diff::Update(desc, _) => println!("● 🔄 Updated {}", desc),
        }
    }
}
