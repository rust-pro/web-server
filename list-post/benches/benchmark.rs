use criterion::{black_box, Criterion, criterion_group, criterion_main};

fn find_name(arr: &mut Vec<&str>) {
  let mut a = 0; //O(1)
  for i in arr.iter() {
    a += 1; //O(n)
    println!("{}", a); // //O(n)
    let mut b = 0;
    for j in arr.iter() {
      b += 1;
      println!("Count: {} -> {}", a, b);
      match j {
        &"Kukun" => println!("Find name 2: {}", j),
        _ => {}
      }
    }
  }
}

fn criterion_benchmark(c: &mut Criterion) {
  let mut arr = black_box(vec![
    "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1",
    "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1",
    "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1",
    "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1",
    "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Ok1", "Hainghia", "Kukun",
  ]);
  c.bench_function("is sorted", |b| b.iter(|| find_name(&mut arr)));
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);