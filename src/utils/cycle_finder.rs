use std::{collections::HashSet, hash::Hash};

/// An iterator adapter that finds cycles in the underlying iterator and stops afterwards
pub struct CycleFinder<T, I>
where
    I: Iterator<Item = T>,
{
    inner: I,
    seen: HashSet<T>,
}

impl<T, I> Iterator for CycleFinder<T, I>
where
    T: PartialEq + Clone + Eq + Hash,
    I: Iterator<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.inner.next();
        if let Some(ref n) = next {
            if self.seen.contains(n) {
                return None;
            }
            self.seen.insert(n.clone());
        }

        next
    }
}

pub trait CycleFinderExt: Iterator {
    fn find_cycle(self) -> CycleFinder<Self::Item, Self>
    where
        Self: Sized,
        Self::Item: PartialEq + Clone,
    {
        CycleFinder {
            inner: self,
            seen: HashSet::new(),
        }
    }
}

impl<T, I> CycleFinderExt for I where I: Iterator<Item = T> {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cycle_finder() {
        let input = vec![1, 2, 3, 4, 5, 1, 2, 3, 4, 5];
        let cycle = input.iter().cloned().find_cycle().collect::<Vec<_>>();
        assert_eq!(cycle, vec![1, 2, 3, 4, 5]);
    }
}
