use rayon::join;
use std::{fmt::Debug, marker::Send};

pub trait KtStd {
    fn let_ref<R>(&self, block: impl FnOnce(&Self) -> R) -> R {
        block(self)
    }
    
    fn let_mut<R>(&mut self, mut block: impl FnMut(&mut Self) -> R) -> R {
        block(self)
    }

    fn drop(self) where Self: Sized {}

    fn let_owned<R>(self, block: impl FnOnce(Self) -> R) -> R where Self: Sized {
        block(self)
    }

    fn let_mut_d<R>(mut self, mut block: impl FnMut(&mut Self) -> R) -> R where Self: Sized {
        block(&mut self)
    }

    fn also_ref(self, block: impl FnOnce(&Self)) -> Self where Self: Sized {
        block(&self);
        self
    }
    
    fn also_mut(mut self, mut block: impl FnMut(&mut Self)) -> Self where Self: Sized {
        block(&mut self);
        self
    }

    fn also_mut_once(mut self, block: impl FnOnce(&mut Self)) -> Self where Self: Sized {
        block(&mut self);
        self
    }

    fn sout(self) -> Self where Self: Debug + Sized {
        dbg!(self)
    }

    fn print(self) -> Self where Self: Debug + Sized {
        self.also_ref(|s| println!("{:#?}", s))
    }

    fn echo(self) -> Self where Self: Debug + Sized {
        self.also_ref(|s| println!("{:?}", s))
    }
}

impl<T> KtStd for T {}

pub trait IterExt<T> {
    fn on_each(self, f: impl Fn(&mut T)) -> Self;
    fn thertation(self, predicate1: impl Fn(&T) -> bool, predicate2: impl Fn(&T) -> bool) -> (Vec<T>, Vec<T>, Vec<T>);
}

impl<T> IterExt<T> for Vec<T> {
    fn on_each(self, f: impl Fn(&mut T)) -> Self {
        self.also_mut(|v| v.iter_mut().for_each(|e| f(e)))
    }

    fn thertation(self, predicate1: impl Fn(&T) -> bool, predicate2: impl Fn(&T) -> bool) -> (Vec<T>, Vec<T>, Vec<T>) {
        let mut first = vec![];
        let mut second = vec![];
        let mut third = vec![];
        for e in self {
            if predicate1(&e) { first.push(e) }
            else if predicate2(&e) { second.push(e) }
            else { third.push(e) }
        }
        (first, second, third)
    }
}

pub trait QuickSort<T> {
    fn quick_sort(self) -> Vec<T>;
    fn quick_sort_rev(self) -> Vec<T>;
}

impl<T: Ord + Copy> QuickSort<T> for Vec<T> {
    fn quick_sort(self) -> Vec<T> {
        match self.len() {
            0 | 1 => self,
            _ => self[0].let_owned(|x| self.thertation(|e| e < &x, |e| e == &x)).let_owned(|(less, mut equal, greater): (Vec<T>, Vec<T>, Vec<T>)|
                    less.quick_sort().also_mut(|v| v.append(&mut equal)).also_mut_once(|v| v.append(&mut greater.quick_sort()))
                )
        }
    }

    fn quick_sort_rev(self) -> Vec<T> {
        match self.len() {
            0 | 1 => self,
            _ => self[0].let_owned(|x| self.thertation(|e| e < &x, |e| e == &x)).let_owned(|(less, mut equal, greater): (Vec<T>, Vec<T>, Vec<T>)|
                    greater.quick_sort_rev().also_mut(|v| v.append(&mut equal)).also_mut_once(|v| v.append(&mut less.quick_sort_rev()))
                )
        }
    }
}

pub trait QSortP<T> {
    fn qsort_p(self) -> Vec<T>;
    fn qsort_p_rev(self) -> Vec<T>;
}

impl<T> QSortP<T> for Vec<T> where T: Ord + Copy + Send {
    fn qsort_p(self) -> Vec<T> {
        match self.len() {
            0 | 1 => self,
            _ => self[0].let_owned(|x| self.thertation(|e| e < &x, |e| e == &x)).let_owned(|(less, mut equal, greater): (Vec<T>, Vec<T>, Vec<T>)|
                    join(|| less.qsort_p(), || greater.qsort_p())
                        .let_owned(|(small, mut big)| small.also_mut(|v| v.append(&mut equal)).also_mut(|v| v.append(&mut big)))
                )
        }
    }

    fn qsort_p_rev(self) -> Vec<T> {
        match self.len() {
            0 | 1 => self,
            _ => self[0].let_owned(|x| self.thertation(|e| e < &x, |e| e == &x)).let_owned(|(less, mut equal, greater): (Vec<T>, Vec<T>, Vec<T>)|
                    join(|| less.qsort_p_rev(), || greater.qsort_p_rev())
                        .let_owned(|(mut small, big)| big.also_mut(|v| v.append(&mut equal)).also_mut(|v| v.append(&mut small)))
                    )
        }
    }
}