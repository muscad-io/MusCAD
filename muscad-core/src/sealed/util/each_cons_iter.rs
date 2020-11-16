/// Helper methods for iterating points and vectors

pub struct EachCons3Iter<'container, V> {
    i1: Box<dyn Iterator<Item = V> + 'container>,
    i2: Box<dyn Iterator<Item = V> + 'container>,
    i3: Box<dyn Iterator<Item = V> + 'container>,
    _next: fn(&mut Self) -> Option<(V, V, V)>,
}

impl<'container, V> EachCons3Iter<'container, V> {
    pub fn build(
        i1: Box<dyn Iterator<Item = V> + 'container>,
        i2: Box<dyn Iterator<Item = V> + 'container>,
        i3: Box<dyn Iterator<Item = V> + 'container>,
        f: fn(&mut Self) -> Option<(V, V, V)>,
    ) -> Self {
        Self {
            i1,
            i2,
            i3,
            _next: f,
        }
    }

    pub fn _next_cycle(this: &mut Self) -> Option<(V, V, V)> {
        match this.i1.next() {
            Some(v) => Some((v, this.i2.next().unwrap(), this.i3.next().unwrap())),
            None => None,
        }
    }

    pub fn _next(this: &mut Self) -> Option<(V, V, V)> {
        match this.i3.next() {
            Some(v) => Some((this.i1.next().unwrap(), this.i2.next().unwrap(), v)),
            None => None,
        }
    }
}

impl<'container, V> Iterator for EachCons3Iter<'container, V> {
    type Item = (V, V, V);

    fn next(&mut self) -> Option<Self::Item> {
        (self._next)(self)
    }
}

pub fn to_each_cons_3_cycle_iter<'container, I, V>(it: I) -> EachCons3Iter<'container, V>
where
    I: Iterator<Item = V> + Clone + 'container,
{
    let mut i2 = it.clone().cycle();
    i2.next();

    let mut i3 = it.clone().cycle();
    i3.next();
    i3.next();

    EachCons3Iter::build(
        Box::new(it),
        Box::new(i2),
        Box::new(i3),
        EachCons3Iter::_next_cycle,
    )
}

pub fn to_each_cons_3_iter<'container, I, V>(it: I) -> EachCons3Iter<'container, V>
where
    I: Iterator<Item = V> + Clone + 'container,
{
    let mut i2 = it.clone();
    i2.next();

    let mut i3 = it.clone();
    i3.next();
    i3.next();

    EachCons3Iter::build(
        Box::new(it),
        Box::new(i2),
        Box::new(i3),
        EachCons3Iter::_next,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geom_iter_each_cons_3_cycle() {
        let v = vec![1, 2, 3, 4];
        let r = to_each_cons_3_cycle_iter(v.iter()).collect::<Vec<_>>();

        assert_eq!(
            r,
            vec![(&1, &2, &3), (&2, &3, &4), (&3, &4, &1), (&4, &1, &2)]
        );
    }

    #[test]
    fn test_geom_iter_each_cons_3() {
        let v = vec![1, 2, 3, 4];
        let r = to_each_cons_3_iter(v.iter()).collect::<Vec<_>>();

        assert_eq!(r, vec![(&1, &2, &3), (&2, &3, &4)]);
    }
}
