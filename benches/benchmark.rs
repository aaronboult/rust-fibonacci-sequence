extern crate criterion;
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("FibonacciBenchmark", |b| b.iter(|| fibonacci(
        vec![
            String::from("fibonacci.exe"),
            String::from("10000"),
            String::from("-v"),
        ] // This mimics passing command line arguments
    )));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);


extern crate num_bigint;
extern crate num_traits;

use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::time::Instant;

use num_bigint::BigUint;
use num_traits::{One, Zero};

fn fibonacci(args: Vec<String>) {

    if !check_args(&args) {
        println!("\
                    A simple program providing different options in terms of retrieving large scale fibonacci numbers.\
                    \nUsage:\
                    \n\tfibonacci.exe max_value [options]\
                    \nOptions:\
                    \n\t-h (--help): Outputs this help message\
                    \n\t-v (--verbose): Outputs all values and their associated index (Significantly slows down execution time)\
                    \n\t-f (--file): The file name to write the output to, instead of the terminal");

        return;
    }

    let number_of_iterations = args[1].parse::<u64>().unwrap();

    let verbose = if args.contains(&"-v".to_string()) || args.contains(&"--verbose".to_string()) {

        true

    }
    else {

        false

    };

    let (write_file, file_name) = if args.contains(&"-f".to_string()) {

        let index = args.binary_search(&"-f".to_string()).unwrap();

        (true, args[index + 1].to_string())

    }
    else if args.contains(&"--file".to_string()) {

        let index = args.binary_search(&"--file".to_string()).unwrap();

        (true, args[index + 1].to_string())

    }
    else {

        (false, "".to_string())

    };

    let stdout = io::stdout();

    let mut writer = match write_file {

        true => Box::new(BufWriter::new(File::create(file_name).expect("Unable to create file"))) as Box<dyn Write>,

        false => Box::new(BufWriter::new(stdout.lock())) as Box<dyn Write>,

    };

    let beginning = Instant::now();

    let mut current: BigUint = One::one();
    let mut previous: BigUint = Zero::zero();
    let mut term = 1;

    while term <= number_of_iterations {

        if term == number_of_iterations || verbose {

            writer
                .write_fmt(format_args!("Term {}: {}\n", term, previous))
                .unwrap();

        }

        let next = &current + previous;

        previous = current;

        current = next;

        term += 1;

    }

    writer
        .write_fmt(format_args!(
            "Took {} milliseconds to generate {} fibonacci values\n",
            beginning.elapsed().as_millis(),
            number_of_iterations
        ))
        .unwrap();

}

fn check_args(args: &Vec<String>) -> bool {

    if args.len() == 1 {
        
        return false;

    }

    let max = args[1].parse::<u64>();

    if max.is_err() {
        
        return false;

    }

    for (i, arg) in args.iter().enumerate() {

        if i == 0 || i == 1 {

            continue;

        }

        if !["-h", "--help", "-v", "--verbose", "-f", "--file"].contains(&&arg[..]) && args[i - 1] != "-f" && args[i - 1] != "--file" {
            
            return false;

        }
        else {

            if ["-f", "--file"].contains(&&arg[..]) {
                
                if i >= args.len() - 1 {
                    
                    return false;

                }
                else {

                    if ["-h", "--help", "-v", "--verbose"].contains(&&args[i + 1][..]) {
                        
                        return false;

                    }

                }

            }

        }

    }

    true
}
