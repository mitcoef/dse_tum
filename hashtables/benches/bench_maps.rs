use rand::prelude::*;
use std::collections::HashMap;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use hashtables::{
    chaining_hashtable::ChainingHashtable, hashtable::SimpleHashtable,
    open_hashtable::OpenHashtable, std_reference_fxhash::FxWrapper,
};

fn gen_relation(size: usize) -> Vec<(u64, u64)> {
    let mut result: Vec<(u64, u64)> = Vec::new();

    let mut rng = rand::rngs::OsRng;

    for _ in 0..size {
        let key = rng.gen::<u64>();
        let value = rng.gen::<u64>();
        result.push((key, value));
    }

    result
}

pub fn bench_hashmap<T: SimpleHashtable<Key = u64, Value = u64>>(
    size: usize,
    relation: &[(u64, u64)],
) {
    let mut map = T::new(size);

    // insert the relation
    for (key, value) in relation.iter() {
        map.insert(*key, *value);
    }

    // lookup every third key
    for (key, _) in relation.iter().step_by(3) {
        map.lookup(key);
    }

    // erase every second key
    for (key, _) in relation.iter().step_by(2) {
        map.erase(key);
    }
}

fn bench_maps(c: &mut Criterion) {
    let mut group = c.benchmark_group("Maps");

    for size in [4096, 131072] {
        let relation = gen_relation(size);

        group.bench_with_input(BenchmarkId::new("Chaining", size), &size, |b, i| {
            b.iter(|| bench_hashmap::<ChainingHashtable>(*i, black_box(&relation)))
        });
        group.bench_with_input(BenchmarkId::new("Open", size), &size, |b, i| {
            b.iter(|| bench_hashmap::<OpenHashtable>(*i, black_box(&relation)))
        });
        group.bench_with_input(BenchmarkId::new("Reference", size), &size, |b, i| {
            b.iter(|| bench_hashmap::<HashMap<u64, u64>>(*i, black_box(&relation)))
        });
        group.bench_with_input(BenchmarkId::new("ReferenceFx", size), &size, |b, i| {
            b.iter(|| bench_hashmap::<FxWrapper<u64, u64>>(*i, black_box(&relation)))
        });
    }

    group.finish();
}

criterion_group!(benches, bench_maps);
criterion_main!(benches);
