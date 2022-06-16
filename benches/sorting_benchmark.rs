use crust_of_rust::sorting;
use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion,
};
use rand;

criterion_group!(benches, sorting_benchmarks);
criterion_main!(benches);

fn sorting_benchmarks(c: &mut Criterion) {
    let mut vec = init_data(10000);

    c.bench_function(
        "bubble sort",
        |b| b.iter(|| sorting::bubble_sort(&mut vec))
    );
    c.bench_function(
        "insertion sort",
        |b| b.iter(|| sorting::insertion_sort(&mut vec))
    );
    c.bench_function(
        "selection sort",
        |b| b.iter(|| sorting::selection_sort(&mut vec))
    );
    c.bench_function(
        "quick sort",
        |b| b.iter(|| sorting::quick_sort(&mut vec))
    );
    c.bench_function(
        "default sort",
        |b| b.iter(|| vec.sort_unstable())
    );
}

fn init_data(data_size: usize) -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::with_capacity(data_size);
    for _ in 0..vec.capacity() {
        vec.push(rand::random())
    }
    black_box(vec)
}
