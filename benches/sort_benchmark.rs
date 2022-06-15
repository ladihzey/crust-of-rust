use crust_of_rust::sorting;
use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion,
};

fn sort_arr_benchmark(c: &mut Criterion) {
    let mut array = black_box(
        [6, 2, 10, 123, -2, 10, -17, 22, 7, 8, 11, 124, -4, 12, -53, 24, 6, 2, 10, 123, -2, 10, -17, 22, 7, 8, 11, 124, -4, 12, -53, 24]
    );

    c.bench_function(
        "insertion_sort",
        |b| b.iter(|| sorting::insertion_sort(&mut array))
    );
    c.bench_function(
        "bubble_sort",
        |b| b.iter(|| sorting::bubble_sort(&mut array))
    );
    c.bench_function(
        "selection_sort",
        |b| b.iter(|| sorting::selection_sort(&mut array))
    );
    c.bench_function(
        "quick_sort",
        |b| b.iter(|| sorting::quick_sort(&mut array))
    );
    c.bench_function(
        "default_sort",
        |b| b.iter(|| array.sort_unstable())
    );
}

criterion_group!(benches, sort_arr_benchmark);
criterion_main!(benches);