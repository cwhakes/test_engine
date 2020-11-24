use std::cmp::Ordering;

impl<I: Iterator> PartialMaxMin for I {}

pub trait PartialMaxMin: Iterator {
    fn partial_max(self) -> Option<Self::Item>
    where
        Self: Sized,
        Self::Item: PartialOrd,
    {
        self.partial_max_by(PartialOrd::partial_cmp)
    }

    fn partial_min(self) -> Option<Self::Item>
    where
        Self: Sized,
        Self::Item: PartialOrd,
    {
        self.partial_min_by(PartialOrd::partial_cmp)
    }

    #[inline]
    fn partial_max_by_key<B: PartialOrd, F>(self, f: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> B,
    {
        #[inline]
        fn key<T, B>(mut f: impl FnMut(&T) -> B) -> impl FnMut(T) -> (B, T) {
            move |x| (f(&x), x)
        }

        #[inline]
        fn compare<T, B: PartialOrd>((x_p, _): &(B, T), (y_p, _): &(B, T)) -> Option<Ordering> {
            x_p.partial_cmp(y_p)
        }

        let (_, x) = self.map(key(f)).partial_max_by(compare)?;
        Some(x)
    }

    #[inline]
    fn partial_max_by<F>(self, compare: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> Option<Ordering>,
    {
        #[inline]
        fn fold<T>(
            mut compare: impl FnMut(&T, &T) -> Option<Ordering>,
        ) -> impl FnMut(T, T) -> Option<T> {
            move |x, y| partial_max_by(x, y, &mut compare)
        }

        fold1(self, fold(compare))
    }

    #[inline]
    fn partial_min_by_key<B: PartialOrd, F>(self, f: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> B,
    {
        #[inline]
        fn key<T, B>(mut f: impl FnMut(&T) -> B) -> impl FnMut(T) -> (B, T) {
            move |x| (f(&x), x)
        }

        #[inline]
        fn compare<T, B: PartialOrd>((x_p, _): &(B, T), (y_p, _): &(B, T)) -> Option<Ordering> {
            x_p.partial_cmp(y_p)
        }

        let (_, x) = self.map(key(f)).partial_min_by(compare)?;
        Some(x)
    }

    #[inline]
    fn partial_min_by<F>(self, compare: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> Option<Ordering>,
    {
        #[inline]
        fn fold<T>(
            mut compare: impl FnMut(&T, &T) -> Option<Ordering>,
        ) -> impl FnMut(T, T) -> Option<T> {
            move |x, y| partial_min_by(x, y, &mut compare)
        }

        fold1(self, fold(compare))
    }
}

#[inline]
#[must_use]
pub fn partial_min_by<T, F: FnOnce(&T, &T) -> Option<Ordering>>(
    v1: T,
    v2: T,
    compare: F,
) -> Option<T> {
    compare(&v1, &v2).map(|res| match res {
        Ordering::Less | Ordering::Equal => v1,
        Ordering::Greater => v2,
    })
}

#[inline]
#[must_use]
pub fn partial_max_by<T, F: FnOnce(&T, &T) -> Option<Ordering>>(
    v1: T,
    v2: T,
    compare: F,
) -> Option<T> {
    compare(&v1, &v2).map(|res| match res {
        Ordering::Less | Ordering::Equal => v2,
        Ordering::Greater => v1,
    })
}

/// Fold an iterator without having to provide an initial value.
#[inline]
fn fold1<I, F>(mut it: I, f: F) -> Option<I::Item>
where
    I: Iterator,
    F: FnMut(I::Item, I::Item) -> Option<I::Item>,
{
    // start with the first element as our selection. This avoids
    // having to use `Option`s inside the loop, translating to a
    // sizeable performance gain (6x in one case).
    let first = it.next()?;
    it.try_fold(first, f)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn find_extremes() {
        let array = [55.1f32, -100.0, 72.1111111, 33.3, 100000000.0];
        let max = array.iter().partial_max().unwrap();
        let min = array.iter().partial_min_by_key(|f| f.powi(2)).unwrap();

        assert_eq!(*max, array[4]);
        assert_eq!(*min, array[3]);
    }

    #[test]
    fn find_extremes_fail() {
        let array = [55.1, -100.0, 72.1111111, 33.3, 100000000.0, f32::NAN];
        let max = array.iter().partial_max();
        let min = array.iter().partial_min_by_key(|f| f.powi(2));

        assert!(max.is_none());
        assert!(min.is_none());
    }
}
