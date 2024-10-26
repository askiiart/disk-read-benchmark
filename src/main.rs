use curl::easy::Easy as easy_curl;
use rand::{self, RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;
use std::{
    env::current_dir,
    fs::{create_dir_all, exists, File},
    io::{Error, Write},
    os::unix::fs::FileExt,
    process::Command,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};
use clap::Parser;
use clap::command;

fn large_random_file_generation(path: String) {
    // https://stackoverflow.com/a/65235966
    let out = Arc::new(Mutex::new(File::create(path)));
    // NOTE: update this both here and in the helper (_large_random_file_generation_helper())
    let num_threads: u64 = 12;
    let mut threads: Vec<JoinHandle<()>> = Vec::new();
    for i in 0..num_threads {
        let out = Arc::clone(&out);

        let thread = thread::spawn(move || {
            _large_random_file_generation_helper(&i, out);
        });

        threads.push(thread);
    }

    for t in threads {
        t.join().unwrap();
    }
}

fn _large_random_file_generation_helper(i: &u64, out: Arc<Mutex<Result<File, Error>>>) {
    let mut rng = XorShiftRng::seed_from_u64(2484345508);
    // NOTE: update this both here and in `large_random_file_generation()`
    let num_threads = 12;
    let mut data = [0u8; 1310720];
    let block_size = 1310720;

    // enter desired size in bytes, must be a multiple of 655360
    // this is not a typo, the extra zero after 65536is for the threads
    // 26843545600 = 25 GiB
    let blocks_per_thread: u64 = 26843545600 / (block_size * num_threads);
    println!("{}", i);
    for u in (i * blocks_per_thread)..((i + 1) * blocks_per_thread) {
        rng.fill_bytes(&mut data);

        //let offset: u64 = (i * blocks_per_thread * 1310720) + (1310720 * u);
        let offset: u64 = u * block_size;
        let mut out = out.lock().unwrap();
        out.as_mut().unwrap().write_all_at(&data, offset).unwrap();
    }
}

/*
fn single_threaded_large_random_file_generation(path: String) {
    let mut out = File::create(path).unwrap();
    let mut rng = XorShiftRng::seed_from_u64(2484345508);
    let mut data = [0u8; 65536];
    for _ in 0..409600 {
        rng.fill_bytes(&mut data);
        out.write_all(&data).unwrap();
    }
}
*/

fn small_random_files_generation(folder: String) {
    let mut rng = XorShiftRng::seed_from_u64(2484345508);
    let mut data: [u8; 1024] = [0u8; 1024];
    for i in 1..1001 {
        let mut out = File::create(format!("{folder}/{i}")).unwrap();
        rng.fill_bytes(&mut data);
        out.write_all(&data).unwrap();
    }
}

fn create_empty_file(path: String, size: u64) {
    let out = File::create(path).unwrap();
    out.write_all_at(&[0], size - 1).unwrap();
}

fn small_empty_files_generation(folder: String) {
    for i in 1..1001 {
        let out = File::create(format!("{folder}/{i}")).unwrap();
        out.write_all_at(&[0], 1023).unwrap();
    }
}

fn grab_kernel(folder: String, kernel_version: String) {
    // https://cdn.kernel.org/pub/linux/kernel/v6.x/linux-6.6.58.tar.xz
    if !(exists(format!("{folder}/linux-{kernel_version}.tar.xz")).unwrap()) {
        let mut curl = easy_curl::new();
        curl.url(&format!(
            "https://cdn.kernel.org/pub/linux/kernel/v6.x/linux-{kernel_version}.tar.xz"
        ))
        .unwrap();
        curl.follow_location(true).unwrap();
        let mut out = File::create(format!("{folder}/linux-{kernel_version}.tar.xz")).unwrap();
        curl.write_function(move |data| {
            out.write_all(data).unwrap();
            Ok(data.len())
        })
        .unwrap();
        curl.perform().unwrap();
    }

    // i'm too lazy to do this in rust
    let mut dir = current_dir().unwrap();
    dir.push(folder);
    Command::new("tar")
        .current_dir(dir)
        .arg("-xf")
        .arg(&format!("linux-{kernel_version}.tar.xz"))
        .arg("");
}

fn grab_datasets() {
    let kernel_version = "6.6.58";

    create_dir_all("data/kernel").unwrap();

    if !(exists(format!("data/kernel/linux-{kernel_version}")).unwrap()) {
        println!("Downloading kernel...");
        grab_kernel("data/kernel".to_string(), kernel_version.to_string());
        println!("Kernel downloaded");
    } else {
        println!("Kernel already downloaded");
    }

    if !(exists(format!("data/25G-random.bin")).unwrap()) {
        println!("Generating random 25 GiB file...");
        large_random_file_generation("data/25G-random.bin".to_string());
        println!("Random 25 GiB file generated");
    } else {
        println!("Random 25 GiB file already generated");
    }

    if !(exists(format!("data/small-files/random")).unwrap()) {
        println!("Generating random 1 KiB files...");
        create_dir_all("data/small-files/random").unwrap();
        small_random_files_generation("data/small-files/random".to_string());
        println!("Random 1 KiB files generated...");
    } else {
        println!("Random 1 KiB files already generated")
    }

    if !(exists(format!("data/25G-null.bin")).unwrap()) {
        println!("Generating empty 25 GiB file...");
        create_empty_file("data/25G-null.bin".to_string(), 26843545600);
        println!("Empty 25 GiB file generated...");
    } else {
        println!("Empty 25 GiB file already generated");
    }

    if !(exists("data/small-files/null").unwrap()) {
        println!("Generating empty 1 KiB files...");
        create_dir_all("data/small-files/null").unwrap();
        small_empty_files_generation("data/small-files/null".to_string());
        println!("Empty 1 KiB files generated...");
    } else {
        println!("Empty 1 KiB files already generated")
    }

    if !(exists("data/small-files/100M-polygon.txt").unwrap()) {
        println!("*** Get 100M-sided regular polygon data and put it at `./data/small-files/100M-polygon.txt` ***");
    }
}

/// A simple read-only benchmark testing latency, sequential reads, and random reads.
#[derive(Parser, Debug)]
struct Args {
    /// A test thing
    #[arg(short, long, default_value = "hellooooo")]
    this_is_a_testtttt: String
}

fn main() {
    let args = Args::parse();
}
