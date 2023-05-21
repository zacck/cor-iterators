pub fn flatten<I>(iter: I) -> Flatten<I> {
    Flatten::new(iter)
}

pub struct Flatten<O> {
    outer: O,
}

impl<O> Flatten<O> {
    fn new(iter: O) -> Self {
        Flatten { outer: iter }
    }
}

impl<O> Iterator for Flatten<O>
where
    //outer thing has to be an iterator
    O: Iterator,
    //Items of that outer type need to implement IntoIterator
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.outer.next().and_then(|inner| inner.into_iter().next())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty() {
        assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0)
    }
    #[test]
    fn one() {
        assert_eq!(flatten(std::iter::once(vec!["a"])).count(), 1)
    }
    #[test]
    fn two() {
        assert_eq!(flatten(std::iter::once(vec!["a", "b"])).count(), 2)
    }
    #[test]
    fn two_vec() {
        assert_eq!(flatten(vec![vec!["a"], vec!["b"]].into_iter()).count(), 2)
    }
}
