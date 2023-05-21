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
        None
    }
}
