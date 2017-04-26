use std::collections::HashMap;
use std::hash::Hash;

//pub enum Iterators<A,B>
//    where A: Iterator, B: Iterator<Item = A::Item>,
//{
//    A(A),
//    B(B),
//}
//
//impl<A, B> Iterator for Iterators<A, B>
//    where A: Iterator, B: Iterator<Item = A::Item>,
//{
//    type Item = A::Item;
//
//    fn next(&mut self) -> Option<Self::Item> {
//        match *self {
//            Iterators::A(ref mut a) => a.next(),
//            Iterators::B(ref mut b) => b.next(),
//        }
//    }
//}

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

pub struct Substitution<N, F>
    where N: Hash + Eq + Clone, F: Clone,
{
    map: HashMap<N, F>
}

impl<N, F> Substitution<N, F>
    where N: Hash + Eq + Clone, F: Clone,
{
    pub fn new<'a, Ns, Fs>(keys: Ns, values: Fs) -> Self
        where N: 'a,
              F: 'a,
              Ns: IntoIterator<Item = &'a N>,
              Fs: IntoIterator<Item = &'a F>,
    {
        Substitution {
            map: keys.into_iter().cloned()
                     .zip(values.into_iter().cloned())
                     .collect()
        }
    }
}

impl<'s, 'n, N, F> FnOnce<(&'n N,)> for &'s Substitution<N, F>
    where N: Hash + Eq + Clone, F: Clone,
{
    type Output = Option<&'s F>;

    extern "rust-call" fn call_once(self, (arg,): (&'n N,)) -> Self::Output {
        self.map.get(arg)
    }
}
