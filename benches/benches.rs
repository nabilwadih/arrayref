extern crate arrayref;

use arrayref::array_ref;
use criterion::{criterion_group, criterion_main, Criterion, black_box};
use rand::{Rng, SeedableRng};

pub fn array_refs_random_data(c: &mut Criterion) {
    let x = [0,1,2,3,4,5,6,7,8,9];
    c.bench_function("Simple bench", |b| {
        b.iter(|| {
            let a = black_box(array_ref!(x, 0, 5));
            let a = black_box(array_ref!(x, 5, 5));
        })
    });

    c.bench_function("Simple bench unsafe", |b| {
        b.iter(|| {
            let a = black_box(array_ref_unsafe!(x, 0, 5));
            let a = black_box(array_ref_unsafe!(x, 5, 5));
        })
    });

    let x = [0u8; (1usize << 20) - 1];
    c.bench_function("large array bench", |b| {
        b.iter(|| {
            let a = black_box(array_ref!(x, 0, 10001));
            let a = black_box(array_ref!(x, 10001, 999999));
        })
    });

    c.bench_function("large array bench unsafe", |b| {
        b.iter(|| {
            let a = black_box(array_ref_unsafe!(x, 0, 10001));
            let a = black_box(array_ref_unsafe!(x, 10001, 999999));
        })
    });
}

criterion_group!(benches, array_refs_random_data);
criterion_main!(benches);

#[macro_export]
macro_rules! array_ref_unsafe {
    ($arr:expr, $offset:expr, $len:expr) => {{
        {
            #[inline]
            unsafe fn as_array<T>(slice: &[T]) -> &[T; $len] {
                &*(slice.as_ptr() as *const [_; $len])
            }
            let offset = $offset;
            let slice = & $arr[offset..offset + $len];
            #[allow(unused_unsafe)]
            unsafe {
                as_array(slice)
            }
        }
    }}
}
