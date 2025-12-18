//! Simple benchmarks for SetX operations
use std::time::Instant;
use verus_proof_time_testing::set_x::SetX;

fn main() {
    println!("SetX Benchmarks");
    println!("═══════════════");

    // Insert
    let start = Instant::now();
    for _ in 0..1000 {
        let mut s: SetX<i32> = SetX::empty();
        for i in 0..100 {
            s.insert(i);
        }
    }
    println!("insert 100 x 1000:  {:?}", start.elapsed());

    // Mem lookup
    let mut s: SetX<i32> = SetX::empty();
    for i in 0..100 { s.insert(i); }
    let start = Instant::now();
    for _ in 0..10000 {
        for i in 0..100 {
            std::hint::black_box(s.mem(&i));
        }
    }
    println!("mem 100 x 10000:    {:?}", start.elapsed());

    // Iterate
    let start = Instant::now();
    for _ in 0..10000 {
        let _sum: i32 = s.iter().sum();
    }
    println!("iter sum x 10000:   {:?}", start.elapsed());
}
