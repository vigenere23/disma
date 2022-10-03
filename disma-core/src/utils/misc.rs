pub trait IfThen {
    fn if_then<FCompare, FExecute>(self, _if: FCompare, then: FExecute)
    where
        Self: Sized,
        FCompare: Fn(&Self) -> bool,
        FExecute: FnOnce(Self),
    {
        if _if(&self) {
            then(self);
        }
    }
}

impl<T> IfThen for T {}
