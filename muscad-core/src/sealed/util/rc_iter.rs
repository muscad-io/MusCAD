use std::rc::Weak;

pub struct LinkedWeakIter<T, V> {
    cur: Weak<T>,
    mark: Option<Weak<T>>,
    get_next: fn(&Weak<T>) -> Weak<T>,
    get_val: fn(&Weak<T>) -> V,
}

impl<T, V> Clone for LinkedWeakIter<T, V> {
    fn clone(&self) -> Self {
        let mark = match &self.mark {
            Some(v) => Some(Weak::clone(v)),
            None => None,
        };

        LinkedWeakIter {
            cur: Weak::clone(&self.cur),
            mark,
            get_next: self.get_next,
            get_val: self.get_val,
        }
    }
}

impl<T, V> LinkedWeakIter<T, V> {
    pub fn build(
        head: Weak<T>,
        get_next: fn(&Weak<T>) -> Weak<T>,
        get_val: fn(&Weak<T>) -> V,
    ) -> Self {
        Self {
            cur: head,
            mark: None,
            get_next,
            get_val,
        }
    }
}

impl<T, V> Iterator for LinkedWeakIter<T, V> {
    type Item = V;
    fn next(&mut self) -> Option<Self::Item> {
        match &self.mark {
            Some(start) => {
                if Weak::ptr_eq(start, &self.cur) {
                    return None;
                }
            }
            None => self.mark = Some(Weak::clone(&self.cur)),
        }

        let rtnval = (self.get_val)(&self.cur);
        self.cur = (self.get_next)(&self.cur);

        Some(rtnval)
    }
}
