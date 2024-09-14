use arrayvec::ArrayVec;
use bench_match::*;
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    ch::find_alphabets_lazy_static('\0');
    ch::find_alphabets_boommap_lazy_static('\0');
    c.bench_function("ch find match", |b| {
        b.iter(|| ch::find_alphabets(black_box('褐')))
    });
    c.bench_function("ch find lazy_static ahash", |b| {
        b.iter(|| ch::find_alphabets_lazy_static(black_box('褐')))
    });
    c.bench_function("ch find lazy_static boom", |b| {
        b.iter(|| ch::find_alphabets_boommap_lazy_static(black_box('褐')))
    });
    c.bench_function("ch find phf", |b| {
        b.iter(|| ch::find_alphabets_phf(black_box('褐')))
    });
    c.bench_function("ch find bs", |b| b.iter(|| ch::find_bs(black_box('褐'))));

    str::find_alphabets_lazy_static("");
    str::find_alphabets_boommap_lazy_static("");
    c.bench_function("str find match", |b| {
        b.iter(|| str::find_alphabets(black_box("勅asd勅")))
    });
    c.bench_function("str find lazy_static ahash", |b| {
        b.iter(|| str::find_alphabets_lazy_static(black_box("勅asd勅")))
    });
    c.bench_function("str find lazy_static boom", |b| {
        b.iter(|| str::find_alphabets_boommap_lazy_static(black_box("勅asd勅")))
    });
    c.bench_function("str find phf", |b| {
        b.iter(|| str::find_alphabets_phf(black_box("勅asd勅")))
    });
    c.bench_function("str find bs", |b| {
        b.iter(|| str::find_bs(black_box("勅asd勅")))
    });

    ch_arr::find_alphabets_lazy_static(['\0', '\0', '\0', '\0', '\0']);
    ch_arr::find_alphabets_boommap_lazy_static(['\0', '\0', '\0', '\0', '\0']);

    c.bench_function("ch_arr find match", |b| {
        b.iter(|| ch_arr::find_alphabets(black_box(&['勅', 'a', 's', 'd', '勅'])))
    });
    let inp = "z勅asd勅zzzzzzzzzzzzzz";
    let data: ArrayVec<char, 5> = inp.chars().skip(1).take(5).collect();
    c.bench_function("ch_arr find match transmute", |b| {
        b.iter(|| {
            ch_arr::find_alphabets(black_box(unsafe {
                std::mem::transmute::<*const char, &[char; 5]>(data.as_ptr())
            }))
        })
    });
    c.bench_function("ch_arr find lazy_static ahash", |b| {
        b.iter(|| ch_arr::find_alphabets_lazy_static(black_box(['勅', 'a', 's', 'd', '勅'])))
    });
    c.bench_function("ch_arr find lazy_static boom", |b| {
        b.iter(|| {
            ch_arr::find_alphabets_boommap_lazy_static(black_box(['勅', 'a', 's', 'd', '勅']))
        })
    });
    // c.bench_function("ch_arr find phf", |b| b.iter(|| ch_arr::find_alphabets_phf(black_box(['勅','a','s','d','勅']))));
    c.bench_function("ch_arr find bs", |b| {
        b.iter(|| ch_arr::find_bs(black_box(['勅', 'a', 's', 'd', '勅'])))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
