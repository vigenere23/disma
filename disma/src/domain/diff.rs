#[derive(Clone, Debug, PartialEq)]
pub enum Diff {
    Add(String),
    Remove(String),
    Update(String, Vec<Diff>),
}

pub trait Differ<T> {
    fn diffs_with(&self, target: &T) -> Vec<Diff>;
}

impl Differ<bool> for bool {
    fn diffs_with(&self, target: &Self) -> Vec<Diff> {
        diffs_between(self, target)
    }
}

impl Differ<String> for String {
    fn diffs_with(&self, target: &Self) -> Vec<Diff> {
        diffs_between(self, target)
    }
}

impl<'a, 'b> Differ<&'b str> for &'a str {
    fn diffs_with(&self, target: &&'b str) -> Vec<Diff> {
        diffs_between(self, target)
    }
}

impl<T, U> Differ<Option<U>> for Option<T>
where
    T: PartialEq<T> + ToString + Differ<U>,
    U: PartialEq<U> + ToString,
{
    fn diffs_with(&self, target: &Option<U>) -> Vec<Diff> {
        match (self, target) {
            (None, None) => vec![],
            (Some(origin), None) => vec![Diff::Remove(origin.to_string())],
            (None, Some(target)) => vec![Diff::Add(target.to_string())],
            (Some(origin), Some(target)) => origin.diffs_with(target),
        }
    }
}

impl<T> Differ<Vec<T>> for Vec<T>
where
    T: PartialEq<T> + ToString,
{
    fn diffs_with(&self, target: &Self) -> Vec<Diff> {
        let mut diffs = vec![];

        for item in self.iter() {
            if !target.contains(item) {
                diffs.push(Diff::Remove(item.to_string()))
            }
        }

        for item in target.iter() {
            if !self.contains(item) {
                diffs.push(Diff::Add(item.to_string()))
            }
        }

        diffs
    }
}

fn diffs_between<T>(origin: T, target: T) -> Vec<Diff>
where
    T: PartialEq<T> + ToString,
{
    let mut diffs = vec![];

    if origin != target {
        diffs.push(Diff::Remove(origin.to_string()));
        diffs.push(Diff::Add(target.to_string()));
    }

    diffs
}
