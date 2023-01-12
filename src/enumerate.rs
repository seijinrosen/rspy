use std::str::Chars;

pub struct PyEnumerate<I> {
    iter: I,
    count: i32,
}
impl<I> PyEnumerate<I> {
    fn new(iter: I, start: i32) -> PyEnumerate<I> {
        PyEnumerate { iter, count: start }
    }
}

impl<I> Iterator for PyEnumerate<I>
where
    I: Iterator,
{
    type Item = (i32, <I as Iterator>::Item);

    fn next(&mut self) -> Option<(i32, <I as Iterator>::Item)> {
        let a = self.iter.next()?;
        let i = self.count;
        self.count += 1;
        Some((i, a))
    }
}

pub trait PyString {
    fn enumerate(&self, start: i32) -> PyEnumerate<Chars>;
}

impl PyString for str {
    fn enumerate(&self, start: i32) -> PyEnumerate<Chars> {
        PyEnumerate::new(self.chars(), start)
    }
}
