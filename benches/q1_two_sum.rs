use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use leetcode::q1_two_sum::{two_sum, two_sum2};

fn make_worst_case(n: usize) -> (Vec<i32>, i32) {
    let nums: Vec<i32> = (0..n as i32).collect();
    let target = nums[0] + nums[n - 1] + 1;
    (nums, target)
}

fn bench_compare(c: &mut Criterion) {
    let mut group = c.benchmark_group("two_sum_compare");
    for &n in &[100usize, 1000, 5000] {
        let (nums, target) = make_worst_case(n);
        group.bench_with_input(BenchmarkId::new("brute_force", n), &(nums.clone(), target), |b, (nums, target)| {
            b.iter(|| two_sum(black_box(nums.clone()), black_box(*target)))
        });
        group.bench_with_input(BenchmarkId::new("hashmap", n), &(nums, target), |b, (nums, target)| {
            b.iter(|| two_sum2(black_box(nums.clone()), black_box(*target)))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_compare);
criterion_main!(benches);
