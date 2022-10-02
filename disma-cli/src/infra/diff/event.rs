use disma::diff::{base::Diff, event::DiffEventListener};

pub struct CliDiffEventListener {}

impl DiffEventListener for CliDiffEventListener {
    fn after_diff_executed(&self, _diff: Diff) {
        todo!()
    }
}
