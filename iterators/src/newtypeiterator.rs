pub struct NewType<T>(pub Vec<T>);

impl<T> NewType<T> {
    pub fn iter(&self) -> NewTypeIter<'_, T> {
        NewTypeIter {
            inner: self,
            pos: 0,
        }
    }

    pub fn iter_mut(&mut self) -> NewTypeIterMut<'_, T> {
        NewTypeIterMut {
            inner: self,
            pos: 0,
        }
    }
}

pub struct NewTypeIter<'a, T: 'a> {
    inner: &'a NewType<T>,
    pos: usize,
}

impl<'a, T> Iterator for NewTypeIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.inner.0.len() {
            return None;
        }

        self.pos += 1;
        self.inner.0.get(self.pos - 1)
    }
}

pub struct NewTypeIterMut<'a, T> {
    inner: &'a mut NewType<T>,
    pos: usize,
}

impl<'a, T> Iterator for NewTypeIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&'_ mut self) -> Option<Self::Item> {
        if self.pos >= self.inner.0.len() {
            return None;
        }

        self.pos += 1;
        let innerT = self.inner.0.get_mut(self.pos - 1).unwrap();

        let ptr = innerT as *mut T;

        unsafe{
            ptr.as_mut()
        }

        //todo!();
    }
}
