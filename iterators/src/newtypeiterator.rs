pub struct NewType<T>(pub Vec<T>);

impl<T> NewType<T> {
    pub fn iter(&self) -> NewTypeIter<'_, T>{
        NewTypeIter { inner: self, pos: 0 }
    }
}

pub struct NewTypeIter<'a, T> {
    inner: &'a NewType<T>,
    pos: usize,
}

impl<'a, T> Iterator for NewTypeIter<'a, T>{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        
        if self.pos >= self.inner.0.len() {
            return None;
        }

        self.pos +=1;
        self.inner.0.get(self.pos - 1)

    }
}