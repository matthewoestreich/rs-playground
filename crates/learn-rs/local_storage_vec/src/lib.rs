#![allow(clippy::new_without_default)]
#![allow(clippy::len_without_is_empty)]

//
//
//
// Exeercise  : https://teach-rs.trifectatech.org/exercises/2-foundations-of-rust/4-traits-and-generics/index.html
// GitHub     : https://github.com/trifectatechfoundation/teach-rs
// Run tests  : [from project root] : `cargo test -p local_storage_vec`
//
//
//

use std::{
    fmt::Debug,
    ops::{Deref, DerefMut, Index, Range, RangeFrom, RangeTo},
};

pub enum LocalStorageVec<T, const N: usize> {
    Heap(Vec<T>),
    Stack { buf: [T; N], len: usize },
}

/********************** LocalStorageVec Impl *************************************/

impl<T, const N: usize> LocalStorageVec<T, N>
where
    T: Copy + Default,
{
    pub fn new() -> Self {
        Self::Stack {
            buf: [T::default(); N],
            len: 0,
        }
    }

    // len, push, pop, insert, remove and clear:
    pub fn len(&self) -> usize {
        match self {
            LocalStorageVec::Heap(items) => items.len(),
            LocalStorageVec::Stack { len, .. } => *len,
        }
    }

    pub fn push(&mut self, item: T) {
        match self {
            LocalStorageVec::Heap(items) => items.push(item),
            LocalStorageVec::Stack { buf, len } => {
                if *len >= N {
                    *self = Self::from(Vec::from(*buf));
                    self.push(item);
                } else {
                    buf[*len] = item;
                    *len += 1;
                }
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match self {
            LocalStorageVec::Heap(items) => items.pop(),
            LocalStorageVec::Stack { buf, len } => {
                if *len == 0 {
                    return None;
                }
                *len -= 1;
                let p = buf[*len];
                buf[*len] = T::default();
                Some(p)
            }
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        match self {
            LocalStorageVec::Heap(items) => items.remove(index),
            LocalStorageVec::Stack { buf, len } => {
                if index > *len {
                    panic!("[remove] index > *len");
                }
                if index == *len {
                    return self.pop().expect("remove : element to exist");
                }
                let removed = buf[index];
                buf.copy_within(index + 1..*len, index);
                *len -= 1;
                removed
            }
        }
    }

    pub fn insert(&mut self, index: usize, item: T)
    where
        T: Debug,
    {
        match self {
            LocalStorageVec::Heap(items) => items.insert(index, item),
            LocalStorageVec::Stack { buf, len } => {
                if index >= N {
                    panic!("index out of bounds! max index is {} got : {index}", N - 1);
                }
                if *len >= N {
                    let mut v = Vec::from(*buf);
                    v.insert(index, item);
                    *self = Self::from(v);
                } else {
                    buf.copy_within(index..*len, index + 1);
                    buf[index] = item;
                    *len += 1;
                }
            }
        }
    }

    pub fn clear(&mut self) {
        match self {
            LocalStorageVec::Heap(items) => items.clear(),
            LocalStorageVec::Stack { buf, len } => {
                *buf = [T::default(); N];
                *len = 0;
            }
        }
    }
}

/********************** LocalStorageVec Borrow Iter Impl *************************/

impl<'a, T, const N: usize> LocalStorageVec<T, N> {
    pub fn iter(&'a self) -> LocalStorageVecBorrowIter<'a, T, N> {
        match self {
            LocalStorageVec::Heap(items) => LocalStorageVecBorrowIter {
                slice: items.as_slice(),
                counter: 0,
            },
            LocalStorageVec::Stack { buf, len } => LocalStorageVecBorrowIter {
                slice: &buf[..*len],
                counter: 0,
            },
        }
    }
}

/********************** LocalStorageVec From<[T; N]> Impl ************************/

impl<T, const N: usize, const M: usize> From<[T; N]> for LocalStorageVec<T, M>
where
    T: Default,
{
    fn from(array: [T; N]) -> Self {
        if N <= M {
            let mut it = array.into_iter();
            Self::Stack {
                buf: [(); M].map(|_| it.next().unwrap_or_default()),
                len: N,
            }
        } else {
            Self::Heap(Vec::from(array))
        }
    }
}

/********************** LocalStorageVec From<Vec<T>> Impl ************************/

impl<T, const N: usize> From<Vec<T>> for LocalStorageVec<T, N> {
    fn from(value: Vec<T>) -> Self {
        Self::Heap(value)
    }
}

/********************** LocalStorageVec AsRef Impl ********************************/

impl<T, const N: usize> AsRef<[T]> for LocalStorageVec<T, N> {
    fn as_ref(&self) -> &[T] {
        match self {
            LocalStorageVec::Heap(vec) => vec.as_slice(),
            LocalStorageVec::Stack { buf, len } => &buf[..*len],
        }
    }
}

/********************** LocalStorageVec AsMut Impl *******************************/

impl<T, const N: usize> AsMut<[T]> for LocalStorageVec<T, N> {
    fn as_mut(&mut self) -> &mut [T] {
        match self {
            LocalStorageVec::Heap(vec) => vec.as_mut_slice(),
            LocalStorageVec::Stack { buf, len } => &mut buf[..*len],
        }
    }
}

/********************** LocalStorageVec Index<usize> Impl ************************/

impl<T, const N: usize> Index<usize> for LocalStorageVec<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            LocalStorageVec::Heap(items) => &items[index],
            LocalStorageVec::Stack { buf, .. } => &buf[index],
        }
    }
}

/********************** LocalStorageVec Index<RangeTo<>> Impl ********************/

impl<T, const N: usize> Index<RangeTo<usize>> for LocalStorageVec<T, N> {
    type Output = [T];

    fn index(&self, index: RangeTo<usize>) -> &Self::Output {
        match self {
            LocalStorageVec::Heap(items) => &items[index],
            LocalStorageVec::Stack { buf, len } => {
                let e = if index.end > *len { *len } else { index.end };
                &buf[..e]
            }
        }
    }
}

/********************** LocalStorageVec Index<RangeFrom<> Impl *******************/

impl<T, const N: usize> Index<RangeFrom<usize>> for LocalStorageVec<T, N> {
    type Output = [T];

    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        match self {
            LocalStorageVec::Heap(items) => &items[index],
            LocalStorageVec::Stack { buf, len } => &buf[index.start..*len],
        }
    }
}

/********************** LocalStorageVec Index<Range> Impl ************************/

impl<T, const N: usize> Index<Range<usize>> for LocalStorageVec<T, N> {
    type Output = [T];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        match self {
            LocalStorageVec::Heap(items) => &items[index],
            LocalStorageVec::Stack { buf, .. } => &buf[index],
        }
    }
}

/********************** LocalStorageVec IntoIterator Impl ************************/

impl<T, const N: usize> IntoIterator for LocalStorageVec<T, N>
where
    T: Default,
{
    type Item = T;
    type IntoIter = LocalStorageVecIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        LocalStorageVecIter {
            vec: self,
            counter: 0,
        }
    }
}

/********************** LocalStorageVec Deref Impl *******************************/

impl<T, const N: usize> Deref for LocalStorageVec<T, N> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        match self {
            LocalStorageVec::Heap(vec) => vec.as_slice(),
            LocalStorageVec::Stack { buf, len } => &buf[..*len],
        }
    }
}

/********************** LocalStorageVec DerefMut Impl ****************************/

impl<T, const N: usize> DerefMut for LocalStorageVec<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            LocalStorageVec::Heap(vec) => vec.as_mut_slice(),
            LocalStorageVec::Stack { buf, len } => &mut buf[..*len],
        }
    }
}

/*-------------------------------------------------------------------------------*/
/*                                 Structs                                       */
/*-------------------------------------------------------------------------------*/

/********************** LocalStorageVecBorrowIter ********************************/

pub struct LocalStorageVecBorrowIter<'a, T, const N: usize> {
    slice: &'a [T],
    counter: usize,
}

impl<'a, T, const N: usize> Iterator for LocalStorageVecBorrowIter<'a, T, N> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter < self.slice.len() {
            let item = &self.slice[self.counter];
            self.counter += 1;
            Some(item)
        } else {
            None
        }
    }
}

/********************** LocalStorageVecIter **************************************/

pub struct LocalStorageVecIter<T, const N: usize> {
    vec: LocalStorageVec<T, N>,
    counter: usize,
}

impl<T, const N: usize> Iterator for LocalStorageVecIter<T, N>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.vec {
            LocalStorageVec::Heap(items) => {
                if self.counter < items.len() {
                    let r = Some(std::mem::take(&mut items[self.counter]));
                    self.counter += 1;
                    return r;
                }
                None
            }
            LocalStorageVec::Stack { buf, len } => {
                if self.counter < *len {
                    self.counter += 1;
                    let r = Some(std::mem::take(&mut buf[self.counter]));
                    self.counter += 1;
                    return r;
                }
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::LocalStorageVec;

    #[test]
    // Don't remove the #[ignore] attribute or your tests will take forever!
    #[ignore = "This test is just to validate the definition of `LocalStorageVec`. If it compiles, all is OK"]
    #[allow(unreachable_code, unused_variables)]
    fn it_compiles() {
        // Here's a trick to 'initialize' a type while not actually
        // creating a value: an infinite `loop` expression diverges
        // and evaluates to the 'never type' `!`, which, as is can never
        // actually be instantiated, coerces to any other type.
        // Some other ways of diverging are by calling the `panic!` or the `todo!`
        // macros.
        // More info:
        // - https://doc.rust-lang.org/rust-by-example/fn/diverging.html
        // - https://doc.rust-lang.org/reference/expressions/loop-expr.html#infinite-loops
        let vec: LocalStorageVec<u32, 10> = LocalStorageVec::from([3]);
        match vec {
            LocalStorageVec::Stack { buf, len } => {
                let _buf: [u32; 10] = buf;
                let _len: usize = len;
            }
            LocalStorageVec::Heap(v) => {
                let _v: Vec<u32> = v;
            }
        }
    }

    // Uncomment me for part B
    #[test]
    fn it_from_vecs() {
        // The `vec!` macro creates a `Vec<T>` in a way that resembles
        // array-initialization syntax.
        let vec: LocalStorageVec<usize, 10> = LocalStorageVec::from(vec![1, 2, 3]);
        // Assert that the call to `from` indeed yields a `Heap` variant
        assert!(matches!(vec, LocalStorageVec::Heap(_)));
        let vec: LocalStorageVec<usize, 2> = LocalStorageVec::from(vec![1, 2, 3]);
        assert!(matches!(vec, LocalStorageVec::Heap(_)));
    }

    // Uncomment me for part C
    #[test]
    fn it_as_refs() {
        let vec: LocalStorageVec<i32, 256> = LocalStorageVec::from([0; 128]);
        let slice: &[i32] = vec.as_ref();
        assert!(slice.len() == 128);
        let vec: LocalStorageVec<i32, 32> = LocalStorageVec::from([0; 128]);
        let slice: &[i32] = vec.as_ref();
        assert!(slice.len() == 128);
        let mut vec: LocalStorageVec<i32, 256> = LocalStorageVec::from([0; 128]);
        let slice_mut: &[i32] = vec.as_mut();
        assert!(slice_mut.len() == 128);
        let mut vec: LocalStorageVec<i32, 32> = LocalStorageVec::from([0; 128]);
        let slice_mut: &[i32] = vec.as_mut();
        assert!(slice_mut.len() == 128);
    }

    // Uncomment me for part D
    #[test]
    fn it_constructs() {
        let vec: LocalStorageVec<usize, 10> = LocalStorageVec::new();
        // Assert that the call to `new` indeed yields a `Stack` variant with zero length
        assert!(matches!(vec, LocalStorageVec::Stack { buf: _, len: 0 }));
    }

    // Uncomment me for part D
    #[test]
    fn it_lens() {
        let vec: LocalStorageVec<_, 3> = LocalStorageVec::from([0, 1, 2]);
        assert_eq!(vec.len(), 3);
        let vec: LocalStorageVec<_, 2> = LocalStorageVec::from([0, 1, 2]);
        assert_eq!(vec.len(), 3);
    }

    // Uncomment me for part D
    #[test]
    fn it_pushes() {
        let mut vec: LocalStorageVec<_, 128> = LocalStorageVec::new();
        for value in 0..128 {
            vec.push(value);
        }
        assert!(matches!(vec, LocalStorageVec::Stack { len: 128, .. }));
        for value in 128..256 {
            vec.push(value);
        }
        let vec_len = vec.len();
        assert!(
            matches!(vec, LocalStorageVec::Heap(v) if v.len() == 256),
            "v.len() = {}",
            vec_len
        )
    }

    // Uncomment me for part D
    #[test]
    fn it_pops() {
        let mut vec: LocalStorageVec<_, 128> = LocalStorageVec::from([0; 128]);
        for _ in 0..128 {
            assert_eq!(vec.pop(), Some(0))
        }
        assert_eq!(vec.pop(), None);

        let mut vec: LocalStorageVec<_, 128> = LocalStorageVec::from([0; 256]);
        for _ in 0..256 {
            assert_eq!(vec.pop(), Some(0))
        }
        assert_eq!(vec.pop(), None);

        let mut vec: LocalStorageVec<_, 128> = LocalStorageVec::from(vec![0; 256]);
        for _ in 0..256 {
            assert_eq!(vec.pop(), Some(0))
        }
        assert_eq!(vec.pop(), None);
    }

    // Uncomment me for part D
    #[test]
    fn it_inserts() {
        let mut vec: LocalStorageVec<_, 4> = LocalStorageVec::from([0, 1, 2]);
        vec.insert(1, 3);
        let len = vec.len();
        assert!(
            matches!(
                vec,
                LocalStorageVec::Stack {
                    buf: [0, 3, 1, 2],
                    len: 4
                }
            ),
            "len={len}"
        );
        let mut vec: LocalStorageVec<_, 4> = LocalStorageVec::from([0, 1, 2, 3]);
        vec.insert(1, 3);
        assert!(matches!(vec, LocalStorageVec::Heap { .. }));
        assert_eq!(vec.as_ref(), &[0, 3, 1, 2, 3]);

        let mut vec: LocalStorageVec<_, 4> = LocalStorageVec::from([0, 1, 2, 3, 4]);
        vec.insert(1, 3);
        assert!(matches!(vec, LocalStorageVec::Heap { .. }));
        assert_eq!(vec.as_ref(), &[0, 3, 1, 2, 3, 4])
    }

    // Uncomment me for part D
    #[test]
    fn it_removes() {
        let mut vec: LocalStorageVec<_, 4> = LocalStorageVec::from([0, 1, 2]);
        let elem = vec.remove(1);
        //dbg!(&vec);
        let vec_len = vec.len();
        assert!(
            matches!(
                vec,
                LocalStorageVec::Stack {
                    buf: [0, 2, _, _],
                    len: 2
                }
            ),
            "len={vec_len}"
        );
        assert_eq!(elem, 1);
        let mut vec: LocalStorageVec<_, 2> = LocalStorageVec::from([0, 1, 2]);
        let elem = vec.remove(1);
        assert!(matches!(vec, LocalStorageVec::Heap(..)));
        assert_eq!(vec.as_ref(), &[0, 2]);
        assert_eq!(elem, 1);
    }

    // Uncomment me for part D
    #[test]
    fn it_clears() {
        let mut vec: LocalStorageVec<_, 10> = LocalStorageVec::from([0, 1, 2, 3]);
        assert!(matches!(vec, LocalStorageVec::Stack { buf: _, len: 4 }));
        vec.clear();
        assert_eq!(vec.len(), 0);

        let mut vec: LocalStorageVec<_, 3> = LocalStorageVec::from([0, 1, 2, 3]);
        assert!(matches!(vec, LocalStorageVec::Heap(_)));
        vec.clear();
        assert_eq!(vec.len(), 0);
    }

    // Uncomment me for part E
    #[test]
    fn it_iters() {
        let vec: LocalStorageVec<_, 128> = LocalStorageVec::from([0; 32]);
        let mut iter = vec.into_iter();
        for item in &mut iter {
            assert_eq!(item, 0);
        }
        assert_eq!(iter.next(), None);

        let vec: LocalStorageVec<_, 128> = LocalStorageVec::from(vec![0; 128]);
        let mut iter = vec.into_iter();
        for item in &mut iter {
            assert_eq!(item, 0);
        }
        assert_eq!(iter.next(), None);
    }

    // Uncomment me for part F
    #[test]
    fn it_indexes() {
        let vec: LocalStorageVec<i32, 10> = LocalStorageVec::from([0, 1, 2, 3, 4, 5]);
        assert_eq!(vec[1], 1);
        assert_eq!(vec[4..], [4, 5]);
        assert_eq!(vec[..10], [0, 1, 2, 3, 4, 5]);
        assert_eq!(vec[4..], [4, 5]);
        assert_eq!(vec[1..3], [1, 2]);
    }

    // Uncomment me for part H
    #[test]
    fn it_borrowing_iters() {
        let vec: LocalStorageVec<String, 10> = LocalStorageVec::from([
            "0".to_owned(),
            "1".to_owned(),
            "2".to_owned(),
            "3".to_owned(),
            "4".to_owned(),
            "5".to_owned(),
        ]);
        let iter = vec.iter();
        for _ in iter {}
        // This requires the `vec` not to be consumed by the call to `iter()`
        drop(vec);
    }

    // Uncomment me for part J
    #[test]
    fn it_derefs() {
        use std::ops::{Deref, DerefMut};
        let vec: LocalStorageVec<_, 128> = LocalStorageVec::from([0; 128]);
        // `chunks` is a method that's defined for slices `[T]`, that we can use thanks to `Deref`
        let _chunks = vec.chunks(4);
        let _slice: &[_] = vec.deref();

        let mut vec: LocalStorageVec<_, 128> = LocalStorageVec::from([0; 128]);
        let _chunks = vec.chunks_mut(4);
        let _slice: &mut [_] = vec.deref_mut();
    }
}
