pub struct ListComparison<T, U> {
    pub extra_self: Vec<T>,
    pub extra_other: Vec<U>,
    pub same: Vec<(T, U)>,
}
