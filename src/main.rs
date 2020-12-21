use my_ext::kt_ext::*;

fn main() {
    // Std自带排序
    let now = std::time::Instant::now();
    let mut temp = vec![7,65,3,6,78,3,56,76,8,3,4,3,4,6,8,4,23,8,5,86,34];
    temp.sort(); temp.echo(); now.elapsed().sout(); println!();

    // 快速排序
    let now = std::time::Instant::now();
    vec![7,65,3,6,78,3,56,76,8,3,4,3,4,6,8,4,23,8,5,86,34].quick_sort().echo();
    now.elapsed().sout(); println!();

    // 并行快排
    let now = std::time::Instant::now();
    vec![7,65,3,6,78,3,56,76,8,3,4,3,4,6,8,4,23,8,5,86,34].qsort_p().echo();
    now.elapsed().sout(); println!();
}
