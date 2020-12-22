#[macro_use]
extern crate bencher;
#[macro_use]
extern crate lazy_static;
use rayon::prelude::*;
use my_ext::kt_ext::*;
use rand::{thread_rng, Rng};

fn gen_rand_vec(n: u16) -> Vec<i128> {
    let mut vec = vec![];
    (0..n).for_each(|_| vec.push(thread_rng().gen_range(i128::MIN, i128::MAX)));
    vec
}

lazy_static! {
    static ref v: Vec<i128> = gen_rand_vec(u16::MAX);
}

fn std_sort() {
    let mut t = v.clone(); t.sort();
}

fn qsort() {
    v.clone().quick_sort();
}

fn qsort_p() {
    v.clone().qsort_p();
}

fn rayon_sort() {
    let mut t = v.clone(); t.par_sort();
}

fn main() {
    benchmark_group!(benches, std_sort, qsort, qsort_p, rayon_sort);
    benchmark_main!(benches);
}
