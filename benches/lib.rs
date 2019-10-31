use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

const MAX_LABEL_LENGTH: usize = 63;

fn iterator(peer_id: String) -> String {
    peer_id
        .as_bytes()
        .chunks(MAX_LABEL_LENGTH)
        .map(std::str::from_utf8)
        .map(std::result::Result::unwrap)
        .collect::<Vec<&str>>()
        .join(".")
}

fn iterator_v2(peer_id: String) -> String {
    let b = peer_id
        .as_bytes()
        .chunks(MAX_LABEL_LENGTH)
        .collect::<Vec<_>>()
        .join(&b'.');
    String::from_utf8(b).expect("we copy from a UTF8 string; qed")
}

fn for_loop(peer_id: String) -> String {
    // Guard for the most common case
    if peer_id.len() <= MAX_LABEL_LENGTH {
        return peer_id;
    }

    // This will only perform one allocation except in extreme circumstances.
    let mut out = String::with_capacity(peer_id.len() + 8);

    for (idx, chr) in peer_id.chars().enumerate() {
        if idx > 0 && idx % MAX_LABEL_LENGTH == 0 {
            out.push('.');
        }
        out.push(chr);
    }
    out
}

fn for_loop_v2(peer_id: String) -> String {
    // Guard for the most common case
    if peer_id.len() <= MAX_LABEL_LENGTH {
        return peer_id;
    }

    let len = ( peer_id.len()  - 1 ) / MAX_LABEL_LENGTH;

    let mut out = vec![0u8; peer_id.len() + len];

    let mut i = 0;

    for (peer_id_i, b) in peer_id.as_bytes().into_iter().enumerate() {
        if i > 0 && peer_id_i % MAX_LABEL_LENGTH == 0 {
            out[i] = b'.';
            i += 1;
        }
        out[i] = *b;
        i += 1;
    }

    String::from_utf8(out).unwrap()
}

fn bench_fibs(c: &mut Criterion) {
    let strings = vec![
        String::from_utf8(vec![b'x'; 32]).unwrap(),
        String::from_utf8(vec![b'x'; 63]).unwrap(),
        String::from_utf8(vec![b'x'; 64]).unwrap(),
        String::from_utf8(vec![b'x'; 126]).unwrap(),
        String::from_utf8(vec![b'x'; 127]).unwrap(),
    ];

    // Correctness testing.
    {
        let strings = strings.clone();
        for s in strings.into_iter() {
            assert_eq!(iterator(s.clone()), iterator_v2(s.clone()));
            assert_eq!(iterator_v2(s.clone()), for_loop(s.clone()));
            assert_eq!(for_loop_v2(s.clone()), for_loop(s));
        }
    };

    let mut group = c.benchmark_group("string segmentation");

    group.bench_function("Iterator", |b| {
        b.iter(|| {
            let strings = strings.clone();

            for s in strings.into_iter() {
                criterion::black_box(iterator(s));
            }
        })
    });

    group.bench_function("Iterator v2", |b| {
        b.iter(|| {
            let strings = strings.clone();

            for s in strings.into_iter() {
                criterion::black_box(iterator_v2(s));
            }
        })
    });

    group.bench_function("For Loop", |b| {
        b.iter(|| {
            let strings = strings.clone();

            for s in strings.into_iter() {
                criterion::black_box(for_loop(s));
            }
        })
    });

    group.bench_function("For Loop v2", |b| {
        b.iter(|| {
            let strings = strings.clone();

            for s in strings.into_iter() {
                criterion::black_box(for_loop_v2(s));
            }
        })
    });

    group.finish();
}

criterion_group!(benches, bench_fibs);
criterion_main!(benches);
