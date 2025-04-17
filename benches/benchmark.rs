use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashMap;

// 定义测试用的操作函数
fn operation1() -> i32 {
    let mut sum = 0;
    for i in 0..100 {
        sum += i;
    }
    sum
}

fn operation2() -> i32 {
    let mut product = 1;
    for i in 1..10 {
        product *= i;
    }
    product
}

fn operation3() -> i32 {
    let x = 42;
    x * x
}

// 使用 HashMap<&str, Box<dyn Fn>> 的实现
fn hashmap_dispatch(key: &str) -> Option<i32> {
    let mut map: HashMap<&str, Box<dyn Fn() -> i32>> = HashMap::new();
    map.insert("op1", Box::new(operation1));
    map.insert("op2", Box::new(operation2));
    map.insert("op3", Box::new(operation3));
    
    map.get(key).map(|f| f())
}

// 使用 match &str 的实现
fn match_dispatch(key: &str) -> Option<i32> {
    match key {
        "op1" => Some(operation1()),
        "op2" => Some(operation2()),
        "op3" => Some(operation3()),
        _ => None,
    }
}

// 基准测试
fn bench_dispatch(c: &mut Criterion) {
    let mut group = c.benchmark_group("Dispatch Comparison");
    
    // 测试 HashMap 实现
    group.bench_function("HashMap Dispatch", |b| {
        b.iter(|| {
            hashmap_dispatch(black_box("op1"));
            hashmap_dispatch(black_box("op2"));
            hashmap_dispatch(black_box("op3"));
        })
    });
    
    // 测试 Match 实现
    group.bench_function("Match Dispatch", |b| {
        b.iter(|| {
            match_dispatch(black_box("op1"));
            match_dispatch(black_box("op2"));
            match_dispatch(black_box("op3"));
        })
    });
    
    group.finish();
}

criterion_group!(benches, bench_dispatch);
criterion_main!(benches);