use criterion::{black_box, criterion_group, criterion_main, Criterion};

use hlsrs::parser::parse;

use m3u8_rs::Playlist;

fn lib_parse(input: &[u8]) {
    match m3u8_rs::parse_playlist(input) {
        Ok((_i, Playlist::MasterPlaylist(_pl))) => (),
        Ok((_i, Playlist::MediaPlaylist(_pl))) => (),
        Err(e) => panic!("Parsing error: \n {}", e),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../fixtures/rendition.m3u8");

    c.bench_function("parse rendition", |b| {
        b.iter(|| lib_parse(black_box(input.as_bytes())))
    });

    c.bench_function("parse new rendition", |b| {
        b.iter(|| parse(black_box(input.as_bytes())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
