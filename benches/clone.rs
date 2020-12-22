use bencher::*;
#[macro_use]
extern crate lazy_static;
use my_ext::kt_ext::*;
use rand::{thread_rng, Rng};
use rayon::prelude::*;

fn gen_rand_vec(n: u16) -> Vec<i128> {
    let mut vec = vec![];
    (0..n).for_each(|_| vec.push(thread_rng().gen_range(i128::MIN, i128::MAX)));
    vec
}

lazy_static! {
    static ref V: Vec<i128> = gen_rand_vec(u16::MAX);
}

fn std_sort(bench: &mut Bencher) {
    bench.iter(|| {
        let mut t = V.clone();
        t.sort();
    })
}

fn qsort(bench: &mut Bencher) {
    bench.iter(|| {
        let t = V.clone();
        t.quick_sort();
    });
}

fn qsort_p(bench: &mut Bencher) {
    bench.iter(|| {
        let t = V.clone();
        t.qsort_p();
    })
}

fn rayon_sort(bench: &mut Bencher) {
    bench.iter(|| {
        let mut t = V.clone();
        t.par_sort();
    })
}

benchmark_group!(benches, std_sort, qsort, qsort_p, rayon_sort);
benchmark_main!(benches);
