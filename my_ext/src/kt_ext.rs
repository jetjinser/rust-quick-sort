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

    fn sout(self) -> Self where Self: Sized + std::fmt::Debug {
        dbg!(self)
    }

    fn print(self) -> Self where Self: Sized + std::fmt::Debug {
        self.also_ref(|s| println!("{:#?}", s))
    }

    fn echo(self) -> Self where Self: Sized + std::fmt::Debug {
        self.also_ref(|s| println!("{:?}", s))
    }
}

impl <T> KtStd for T {}

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
        self.into_iter().for_each(|e|
            if predicate1(&e) { first.push(e) }
            else if predicate2(&e) { second.push(e) }
            else { third.push(e) }
        );
        (first, second, third)
    }
}

pub trait QuickSort {
    fn quick_sort(self, ascending: bool) -> Vec<i32>;
}

impl QuickSort for Vec<i32> {
    fn quick_sort(self, ascending: bool) -> Vec<i32> {
        match self.len() {
            0 => Vec::new(),
            _ => self[0].let_owned(|x| self.thertation(|e| e < &x, |e| e == &x)).let_owned(|(less, mut equal, greater): (Vec<i32>, Vec<i32>, Vec<i32>)| {
                let mut small = less.quick_sort(ascending);
                let mut big = greater.quick_sort(ascending);
                match ascending {
                    true => small.also_mut(|v| v.append(&mut equal)).also_mut(|v| v.append(&mut big)),
                    false => big.also_mut(|v| v.append(&mut equal)).also_mut(|v| v.append(&mut small))
                }
            })
        }
    }
}
