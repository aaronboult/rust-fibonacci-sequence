extern crate criterion;
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("FibonacciBenchmark", |b| b.iter(|| fibonacci()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);



// Fibonacci code

extern crate num_bigint;
extern crate num_traits;

use std::time::Instant;
use num_bigint::BigUint;
use num_traits::{Zero, One};

use std::io::{self, Write, BufWriter};


fn fibonacci() {

    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    let beginning = Instant::now();

    let number_of_iterations = 100_000;

    let mut current: BigUint = One::one();
    let mut previous: BigUint = Zero::zero();
    
    let mut term = 1;

    while term < number_of_iterations{

        let next = &current + previous;

        previous = current;

        current = next;

        term += 1;

        if term == number_of_iterations{

            writer.write_fmt(format_args!("Term {}: {}\n", term, previous)).unwrap();

        }

    }
        
    writer.write_fmt(format_args!("Took {} milliseconds to print {} fibonacci values\n", beginning.elapsed().as_millis(), number_of_iterations)).unwrap();

}