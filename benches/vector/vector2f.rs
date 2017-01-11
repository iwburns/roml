extern crate test;

use roml::vector::vector2f::Vector2f;
use roml::vector::*;
use self::test::*;

#[bench]
fn bench_add_vector(b: &mut Bencher) {
    let mut a = Vector2f::default();
    let c = Vector2f::new(1f32, 2f32);
    let a = test::black_box(&mut a);

    b.iter(|| {
        a.add(&c);
    });
}

#[bench]
fn bench_add_tuple(b: &mut Bencher) {
    let mut a = Vector2f::default();
    let c = (1f32, 2f32);
    let a = test::black_box(&mut a);

    b.iter(|| {
        a.add(&c);
    });
}

#[bench]
fn bench_add_float(b: &mut Bencher) {
    let mut a = Vector2f::default();
    let c = 1f32;
    let a = test::black_box(&mut a);

    b.iter(|| {
        a.add(&c);
    });
}

#[bench]
fn bench_add_into_vector(b: &mut Bencher) {
    let a = Vector2f::default();
    let c = Vector2f::new(1f32, 2f32);
    let mut d = Vector2f::default();
    let d = test::black_box(&mut d);

    b.iter(|| {
        a.add_into(&c, d);
    });
}

#[bench]
fn bench_add_into_tuple(b: &mut Bencher) {
    let a = Vector2f::default();
    let c = (1f32, 2f32);
    let mut d = Vector2f::default();
    let d = test::black_box(&mut d);

    b.iter(|| {
        a.add_into(&c, d);
    });
}

#[bench]
fn bench_add_into_float(b: &mut Bencher) {
    let a = Vector2f::default();
    let c = 1f32;
    let mut d = Vector2f::default();
    let d = test::black_box(&mut d);

    b.iter(|| {
        a.add_into(&c, d);
    });
}

#[bench]
fn bench_angle_vector(b: &mut Bencher) {
    let a = Vector2f::new(1f32, 0f32);
    let c = Vector2f::new(7f32, 7f32);

    b.iter(|| a.angle(&c));
}

#[bench]
fn bench_angle_tuple(b: &mut Bencher) {
    let a = Vector2f::new(1f32, 0f32);
    let c = (7f32, 7f32);

    b.iter(|| a.angle(&c));
}

#[bench]
fn bench_angle_float(b: &mut Bencher) {
    let a = Vector2f::new(1f32, 0f32);
    let c = 7f32;

    b.iter(|| a.angle(&c));
}

#[bench]
fn bench_angle_cos_vector(b: &mut Bencher) {
    let a = Vector2f::new(1f32, 0f32);
    let c = Vector2f::new(7f32, 7f32);

    b.iter(|| a.angle_cos(&c));
}

#[bench]
fn bench_angle_cos_tuple(b: &mut Bencher) {
    let a = Vector2f::new(1f32, 0f32);
    let c = (7f32, 7f32);

    b.iter(|| a.angle_cos(&c));
}

#[bench]
fn bench_angle_cos_float(b: &mut Bencher) {
    let a = Vector2f::new(1f32, 0f32);
    let c = 7f32;

    b.iter(|| a.angle_cos(&c));
}

#[bench]
fn bench_distance_vector(b: &mut Bencher) {
    let a = Vector2f::new(1f32, 0f32);
    let c = Vector2f::new(7f32, 7f32);

    b.iter(|| a.distance(&c));
}

#[bench]
fn bench_distance_tuple(b: &mut Bencher) {
    let a = Vector2f::new(1f32, 0f32);
    let c = (7f32, 7f32);

    b.iter(|| a.distance(&c));
}

#[bench]
fn bench_distance_float(b: &mut Bencher) {
    let a = Vector2f::new(1f32, 0f32);
    let c = 7f32;

    b.iter(|| a.distance(&c));
}

#[bench]
fn bench_distance_sq_vector(b: &mut Bencher) {
    let a = Vector2f::new(1f32, 0f32);
    let c = Vector2f::new(7f32, 7f32);

    b.iter(|| a.distance_sq(&c));
}

#[bench]
fn bench_distance_sq_tuple(b: &mut Bencher) {
    let a = Vector2f::new(1f32, 0f32);
    let c = (7f32, 7f32);

    b.iter(|| a.distance_sq(&c));
}

#[bench]
fn bench_distance_sq_float(b: &mut Bencher) {
    let a = Vector2f::new(1f32, 0f32);
    let c = 7f32;

    b.iter(|| a.distance_sq(&c));
}

#[bench]
fn bench_dot_vector(b: &mut Bencher) {
    let a = Vector2f::new(1f32, 0f32);
    let c = Vector2f::new(7f32, 7f32);

    b.iter(|| a.dot(&c));
}

#[bench]
fn bench_dot_tuple(b: &mut Bencher) {
    let a = Vector2f::new(1f32, 0f32);
    let c = (7f32, 7f32);

    b.iter(|| a.dot(&c));
}

#[bench]
fn bench_dot_float(b: &mut Bencher) {
    let a = Vector2f::new(1f32, 0f32);
    let c = 7f32;

    b.iter(|| a.dot(&c));
}

#[bench]
fn bench_fma_vector(b: &mut Bencher) {
    let mut a = Vector2f::new(1f32, 0f32);
    let c = Vector2f::new(7f32, 7f32);
    let d = Vector2f::new(7f32, 7f32);
    let a = test::black_box(&mut a);

    b.iter(|| {
        a.fma(&c, &d);
    });
}

#[bench]
fn bench_fma_tuple(b: &mut Bencher) {
    let mut a = Vector2f::new(1f32, 0f32);
    let c = (7f32, 7f32);
    let d = (7f32, 7f32);
    let a = test::black_box(&mut a);

    b.iter(|| {
        a.fma(&c, &d);
    });
}

#[bench]
fn bench_fma_float(b: &mut Bencher) {
    let mut a = Vector2f::new(1f32, 0f32);
    let c = 7f32;
    let d = 7f32;
    let a = test::black_box(&mut a);

    b.iter(|| {
        a.fma(&c, &d);
    });
}

#[bench]
fn bench_fma_into_vector(b: &mut Bencher) {
    let a = Vector2f::new(1f32, 0f32);
    let c = Vector2f::new(7f32, 7f32);
    let d = Vector2f::new(7f32, 7f32);
    let mut e = Vector2f::default();
    let e = test::black_box(&mut e);

    b.iter(|| {
        a.fma_into(&c, &d, e);
    });
}

#[bench]
fn bench_fma_into_tuple(b: &mut Bencher) {
    let a = Vector2f::new(1f32, 0f32);
    let c = (7f32, 7f32);
    let d = (7f32, 7f32);
    let mut e = Vector2f::default();
    let e = test::black_box(&mut e);

    b.iter(|| {
        a.fma_into(&c, &d, e);
    });
}

#[bench]
fn bench_fma_into_float(b: &mut Bencher) {
    let a = Vector2f::new(1f32, 0f32);
    let c = 7f32;
    let d = 7f32;
    let mut e = Vector2f::default();
    let e = test::black_box(&mut e);

    b.iter(|| {
        a.fma_into(&c, &d, e);
    });
}

#[bench]
fn bench_length(b: &mut Bencher) {
    let a = Vector2f::new(7f32, 7f32);

    b.iter(|| a.length());
}

#[bench]
fn bench_length_sq(b: &mut Bencher) {
    let a = Vector2f::new(7f32, 7f32);

    b.iter(|| a.length_sq());
}

#[bench]
fn bench_lerp_vector(b: &mut Bencher) {
    let mut a = Vector2f::new(1f32, 0f32);
    let c = Vector2f::new(1f32, 1f32);
    let d = 0.5f32;
    let a = test::black_box(&mut a);

    b.iter(|| {
        a.lerp(&c, d);
    });
}

#[bench]
fn bench_lerp_tuple(b: &mut Bencher) {
    let mut a = Vector2f::new(1f32, 0f32);
    let c = (1f32, 1f32);
    let d = 0.5f32;
    let a = test::black_box(&mut a);

    b.iter(|| {
        a.lerp(&c, d);
    });
}

#[bench]
fn bench_lerp_float(b: &mut Bencher) {
    let mut a = Vector2f::new(1f32, 0f32);
    let c = 1f32;
    let d = 0.5f32;
    let a = test::black_box(&mut a);

    b.iter(|| {
        a.lerp(&c, d);
    });
}

#[bench]
fn bench_lerp_into_vector(b: &mut Bencher) {
    let a = Vector2f::new(1f32, 0f32);
    let c = Vector2f::new(1f32, 1f32);
    let d = 0.5f32;
    let mut e = Vector2f::default();
    let e = test::black_box(&mut e);

    b.iter(|| {
        a.lerp_into(&c, d, e);
    });
}

#[bench]
fn bench_lerp_into_tuple(b: &mut Bencher) {
    let a = Vector2f::new(1f32, 0f32);
    let c = (1f32, 1f32);
    let d = 0.5f32;
    let mut e = Vector2f::default();
    let e = test::black_box(&mut e);

    b.iter(|| {
        a.lerp_into(&c, d, e);
    });
}

#[bench]
fn bench_lerp_into_float(b: &mut Bencher) {
    let a = Vector2f::new(1f32, 0f32);
    let c = 1f32;
    let d = 0.5f32;
    let mut e = Vector2f::default();
    let e = test::black_box(&mut e);

    b.iter(|| {
        a.lerp_into(&c, d, e);
    });
}

#[bench]
fn bench_mul_vector(b: &mut Bencher) {
    let mut a = Vector2f::new(1f32, 0f32);
    let c = Vector2f::new(2f32, 2f32);
    let a = test::black_box(&mut a);

    b.iter(|| {
        a.mul(&c);
    });
}

#[bench]
fn bench_mul_tuple(b: &mut Bencher) {
    let mut a = Vector2f::new(1f32, 0f32);
    let c = (2f32, 2f32);
    let a = test::black_box(&mut a);

    b.iter(|| {
        a.mul(&c);
    });
}

#[bench]
fn bench_mul_float(b: &mut Bencher) {
    let mut a = Vector2f::new(1f32, 0f32);
    let c = 2f32;
    let a = test::black_box(&mut a);

    b.iter(|| {
        a.mul(&c);
    });
}

#[bench]
fn bench_mul_into_vector(b: &mut Bencher) {
    let a = Vector2f::new(1f32, 0f32);
    let c = Vector2f::new(2f32, 2f32);
    let mut d = Vector2f::default();
    let d = test::black_box(&mut d);

    b.iter(|| {
        a.mul_into(&c, d);
    });
}

#[bench]
fn bench_mul_into_tuple(b: &mut Bencher) {
    let a = Vector2f::new(1f32, 0f32);
    let c = (2f32, 2f32);
    let mut d = Vector2f::default();
    let d = test::black_box(&mut d);

    b.iter(|| {
        a.mul_into(&c, d);
    });
}

#[bench]
fn bench_mul_into_float(b: &mut Bencher) {
    let a = Vector2f::new(1f32, 0f32);
    let c = 2f32;
    let mut d = Vector2f::default();
    let d = test::black_box(&mut d);

    b.iter(|| {
        a.mul_into(&c, d);
    });
}

#[bench]
fn bench_negate(b: &mut Bencher) {
    let mut a = Vector2f::new(1f32, 2f32);
    let a = test::black_box(&mut a);

    b.iter(|| {
        a.negate();
    });
}

#[bench]
fn bench_negate_into(b: &mut Bencher) {
    let a = Vector2f::new(1f32, 2f32);
    let mut c = Vector2f::default();
    let c = test::black_box(&mut c);

    b.iter(|| {
        a.negate_into(c);
    });
}

#[bench]
fn bench_normalize(b: &mut Bencher) {
    let mut a = Vector2f::new(1f32, 2f32);
    let a = test::black_box(&mut a);

    b.iter(|| {
        a.normalize();
    });
}

#[bench]
fn bench_normalize_into(b: &mut Bencher) {
    let a = Vector2f::new(1f32, 2f32);
    let mut c = Vector2f::default();
    let c = test::black_box(&mut c);

    b.iter(|| {
        a.normalize_into(c);
    });
}

#[bench]
fn bench_set_vector(b: &mut Bencher) {
    let mut a = Vector2f::default();
    let c = Vector2f::new(2f32, 3f32);
    let a = test::black_box(&mut a);

    b.iter(|| {
        a.set(&c);
    });
}

#[bench]
fn bench_set_tuple(b: &mut Bencher) {
    let mut a = Vector2f::default();
    let c = (2f32, 3f32);
    let a = test::black_box(&mut a);

    b.iter(|| {
        a.set(&c);
    });
}

#[bench]
fn bench_set_float(b: &mut Bencher) {
    let mut a = Vector2f::default();
    let c = 2f32;
    let a = test::black_box(&mut a);

    b.iter(|| {
        a.set(&c);
    });
}

#[bench]
fn bench_sub_vector(b: &mut Bencher) {
    let mut a = Vector2f::default();
    let c = Vector2f::new(2f32, 3f32);
    let a = test::black_box(&mut a);

    b.iter(|| {
        a.sub(&c);
    });
}

#[bench]
fn bench_sub_tuple(b: &mut Bencher) {
    let mut a = Vector2f::default();
    let c = (2f32, 3f32);
    let a = test::black_box(&mut a);

    b.iter(|| {
        a.sub(&c);
    });
}

#[bench]
fn bench_sub_float(b: &mut Bencher) {
    let mut a = Vector2f::default();
    let c = 2f32;
    let a = test::black_box(&mut a);

    b.iter(|| {
        a.sub(&c);
    });
}

#[bench]
fn bench_sub_into_vector(b: &mut Bencher) {
    let a = Vector2f::default();
    let c = Vector2f::new(2f32, 3f32);
    let mut d = Vector2f::default();
    let d = test::black_box(&mut d);

    b.iter(|| {
        a.sub_into(&c, d);
    });
}

#[bench]
fn bench_sub_into_tuple(b: &mut Bencher) {
    let a = Vector2f::default();
    let c = (2f32, 3f32);
    let mut d = Vector2f::default();
    let d = test::black_box(&mut d);

    b.iter(|| {
        a.sub_into(&c, d);
    });
}

#[bench]
fn bench_sub_into_float(b: &mut Bencher) {
    let a = Vector2f::default();
    let c = 2f32;
    let mut d = Vector2f::default();
    let d = test::black_box(&mut d);

    b.iter(|| {
        a.sub_into(&c, d);
    });
}

#[bench]
fn bench_zero(b: &mut Bencher) {
    let mut a = Vector2f::new(1f32, 2f32);
    let a = test::black_box(&mut a);

    b.iter(|| {
        a.zero();
    });
}
// [test]
// fn test_zero() {
//    let mut a = Vector2f::new(1f32, 2f32);
//
//    a.zero();
//
//    assert_eq!(a.x, 0f32);
//    assert_eq!(a.y, 0f32);
//
