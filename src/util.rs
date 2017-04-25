pub enum Iterators<A,B>
    where A: Iterator, B: Iterator<Item = A::Item>,
{
    A(A),
    B(B),
}

impl<A, B> Iterator for Iterators<A, B>
    where A: Iterator, B: Iterator<Item = A::Item>,
{
    type Item = A::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match *self {
            Iterators::A(ref mut a) => a.next(),
            Iterators::B(ref mut b) => b.next(),
        }
    }
}

pub struct IteratorObject<'a, T: 'a> { obj: Box<Iterator<Item=T> + 'a> }

impl<'a, T> IteratorObject<'a, T> {
    pub fn new<U: 'a>(iter: U) -> IteratorObject<'a, T>
        where U: Iterator<Item = T>
    {
        IteratorObject { obj: Box::new(iter) }
    }
}

impl<'a, T> Iterator for IteratorObject<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.obj.next()
    }
}
