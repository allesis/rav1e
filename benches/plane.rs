use criterion::*;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;
use rav1e::bench::frame::*;

fn init_plane_u8(width: usize, height: usize) -> Plane<u8> {
  let mut ra = ChaChaRng::from_seed([0; 32]);
  let data: Vec<u8> = (0..(width * height)).map(|_| ra.random()).collect();
  let out = Plane::from_slice(&data, width);
  if out.cfg.width % 2 == 0 && out.cfg.height % 2 == 0 {
    out
  } else {
    let xpad = out.cfg.width % 2;
    let ypad = out.cfg.height % 2;
    let mut padded =
      Plane::new(out.cfg.width, out.cfg.height, 0, 0, xpad, ypad);
    let mut padded_slice = padded.mut_slice(PlaneOffset { x: 0, y: 0 });
    for (dst_row, src_row) in padded_slice.rows_iter_mut().zip(out.rows_iter())
    {
      dst_row[..out.cfg.width].copy_from_slice(&src_row[..out.cfg.width]);
    }
    padded
  }
}

fn init_plane_u16(width: usize, height: usize) -> Plane<u16> {
  let mut ra = ChaChaRng::from_seed([0; 32]);
  let data: Vec<u16> = (0..(width * height)).map(|_| ra.random()).collect();
  Plane::from_slice(&data, width)
}

pub fn downsample_8bit(c: &mut Criterion) {
  let input = init_plane_u8(1920, 1080);
  c.bench_function("downsample_8bit", move |b| {
    b.iter(|| {
      let _ = input.downsampled(input.cfg.width, input.cfg.height);
    })
  });
}

pub fn downsample_odd(c: &mut Criterion) {
  let input = init_plane_u8(1919, 1079);
  c.bench_function("downsample_odd", move |b| {
    b.iter(|| {
      let _ = input.downsampled(input.cfg.width, input.cfg.height);
    })
  });
}

pub fn downsample_10bit(c: &mut Criterion) {
  let input = init_plane_u16(1920, 1080);
  c.bench_function("downsample_10bit", move |b| {
    b.iter(|| {
      let _ = input.downsampled(input.cfg.width, input.cfg.height);
    })
  });
}

fn init_raw_plane_data(width: usize, height: usize) -> Vec<u8> {
  let mut ra = ChaChaRng::from_seed([0; 32]);
  (0..(width * height)).map(|_| ra.random()).collect()
}

pub fn copy_from_raw_u8_8bit(c: &mut Criterion) {
  let input = init_raw_plane_data(1920, 1080);
  let dest = Plane::<u8>::new(1920, 1080, 0, 0, 0, 0);
  c.bench_function("copy_from_raw_u8", move |b| {
    b.iter(|| {
      let mut dest = dest.clone();
      let _ = dest.copy_from_raw_u8(&input, dest.cfg.stride, 1);
    })
  });
}

pub fn copy_from_raw_u8_10bit(c: &mut Criterion) {
  let input = init_raw_plane_data(1920, 1080);
  let dest = Plane::<u16>::new(1920, 1080, 0, 0, 0, 0);
  c.bench_function("copy_from_raw_u8", move |b| {
    b.iter(|| {
      let mut dest = dest.clone();
      let _ = dest.copy_from_raw_u8(&input, dest.cfg.stride, 2);
    })
  });
}

criterion_group!(
  plane,
  downsample_8bit,
  downsample_odd,
  downsample_10bit,
  copy_from_raw_u8_8bit,
  copy_from_raw_u8_10bit
);
