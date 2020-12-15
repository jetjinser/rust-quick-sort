use my_ext::kt_ext::*;

fn main() {
    vec![1,2,3,4].on_each(|n| if n < &mut 3 { *n = 0 } else { *n = 9 }).echo();
    vec![7,65,3,6,78,3,56,76,8,3,4,3,4,6,8,4,23,8,5,86,34].quick_sort(true).echo();
}
