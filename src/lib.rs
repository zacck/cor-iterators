pub fn flatten<I>(iter: I) -> Flatten<I>
where
    I: Iterator,
    I::Item: IntoIterator,
{
    Flatten::new(iter)
}

pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    //Cast inner item into an iterator
    inner: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    fn new(iter: O) -> Self {
        Flatten {
            outer: iter,
            inner: None,
        }
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
        // if there is an inner iterator and it is yeilding
        loop {
            if let Some(ref mut inner_iter) = self.inner {
                if let Some(i) = inner_iter.next() {
                    return Some(i);
                }
                self.inner = None;
            }

            //Pick the next item from the outer things if there is one
            let next_inner_iter = self.outer.next()?.into_iter();
            self.inner = Some(next_inner_iter);
        }
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
    #[test]
    fn empty_two_vec() {
        assert_eq!(
            flatten(vec![Vec::<()>::new(), vec![]].into_iter()).count(),
            0
        )
    }
}
