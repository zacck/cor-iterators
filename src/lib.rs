pub fn flatten<I>(iter: I) -> Flatten<I::IntoIter>
where
    I: IntoIterator,
    I::Item: IntoIterator,
{
    Flatten::new(iter.into_iter())
}

pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    //Cast inner item into an iterator
    //two inner iterators for implementing double endedness
    next_iter: Option<<O::Item as IntoIterator>::IntoIter>,
    back_iter: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    fn new(iter: O) -> Self {
        Flatten {
            outer: iter,
            next_iter: None,
            back_iter: None,
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
            if let Some(ref mut inner_iter) = self.next_iter {
                if let Some(i) = inner_iter.next() {
                    return Some(i);
                }
                self.next_iter = None;
            }

            //Pick the next item from the outer things if there is one
            //consider the case that we have arrived at the outer iterator
            //that was consumed by next_back and we now need to walk it forward
            if let Some(next_inner) = self.outer.next() {
                //walking forward and the forward iterator gives us another element
                self.next_iter = Some(next_inner.into_iter());
            } else {
                //use ? to return None if that's what we get from
                // the back back iter if we dont return what is there and start walking it
                return self.back_iter.as_mut()?.next();
            }
        }
    }
}

impl<O> DoubleEndedIterator for Flatten<O>
where
    O: DoubleEndedIterator,
    O::Item: IntoIterator,
    //when the inner item is turned into an iterator that iterator needs
    //to implement doubleendediterator
    <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut back_iter) = self.back_iter {
                if let Some(i) = back_iter.next_back() {
                    return Some(i);
                }
                self.back_iter = None;
            }

            //Pick the next back item from the outer things if there is one
            //consider the case that we have arrived at the outer iterator
            //that was consumed by next and we now need to walk it backward
            if let Some(next_back_inner) = self.outer.next_back() {
                //walking backward and the backward  iterator gives us another element
                self.back_iter = Some(next_back_inner.into_iter());
            } else {
                //use ? to return None if that's what we get from
                // the  next iter if we dont return what is there and start walking it
                return self.next_iter.as_mut()?.next_back();
            }
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
        assert_eq!(flatten(vec![vec!["a"], vec!["b"]]).count(), 2)
    }
    #[test]
    fn empty_two_vec() {
        assert_eq!(flatten(vec![Vec::<()>::new(), vec![]]).count(), 0)
    }
    #[test]
    fn reverse_two() {
        assert_eq!(
            flatten(std::iter::once(vec!["a", "b"]))
                .rev()
                .collect::<Vec<_>>(),
            vec!["b", "a"]
        )
    }
    #[test]
    fn reverse_two_vec() {
        assert_eq!(
            flatten(vec![vec!["a"], vec!["b"]])
                .rev()
                .collect::<Vec<_>>(),
            vec!["b", "a"]
        )
    }
    #[test]
    fn both_ends() {
        let mut iter = flatten(vec![vec!["a", "b"], vec!["c", "d"]]);
        assert_eq!(iter.next(), Some("a"));
        assert_eq!(iter.next_back(), Some("d"));
        assert_eq!(iter.next(), Some("b"));
        assert_eq!(iter.next_back(), Some("c"));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }
}
