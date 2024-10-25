use curl::easy::{self, Easy as easy_curl};
use rand::{self, RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;
use std::{
    fs::{exists, File},
    io::{Error, Write},
    os::unix::fs::FileExt,
    process::Command,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

fn large_random_file_generation(path: String) {
    // https://stackoverflow.com/a/65235966
    let mut out = Arc::new(Mutex::new(File::create(path)));
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
        out.as_mut().unwrap().write_all_at(&data, offset);
    }
}

fn single_threaded_large_random_file_generation(path: String) {
    let mut out = File::create(path).unwrap();
    let mut rng = XorShiftRng::seed_from_u64(2484345508);
    let mut data = [0u8; 65536];
    for i in 0..409600 {
        rng.fill_bytes(&mut data);
        out.write_all(&data);
    }
}

fn small_random_files_generation(folder: String) {
    let mut rng = XorShiftRng::seed_from_u64(2484345508);
    let mut data = [0u8; 1024];
    for i in 1..1001 {
        let mut out = File::create(format!("{folder}/{i}")).unwrap();
        rng.fill_bytes(&mut data);
        out.write(&data);
    }
}

fn grab_kernel(folder: String) {
    // https://cdn.kernel.org/pub/linux/kernel/v6.x/linux-6.6.58.tar.xz
    let mut curl = easy_curl::new();
    curl.url("https://cdn.kernel.org/pub/linux/kernel/v6.x/linux-6.6.58.tar.xz");
    curl.follow_location(true);
    if !(exists(format!("{folder}/kernel.tar.xz")).unwrap()) {
        let mut out = File::create(format!("{folder}/kernel.tar.xz")).unwrap();
        curl.write_function(move |data| {
            out.write_all(data).unwrap();
            Ok(data.len())
        });
        curl.perform().unwrap();
    }

    // i'm too lazy to do this in rust
    println!(
        "{:?}",
        Command::new("tar")
            .arg("-xvf")
            .arg("data/kernel/kernel.tar.xz")
            .arg("--one-top-level")
            .current_dir("data/kernel/")
            .output()
            .unwrap()
    );
}

fn main() {
    large_random_file_generation("data/25G-random.bin".to_string());
    //single_threaded_large_random_file_generation("data/output".to_string())

    //small_random_files_generation("data/small-files/random".to_string());
    //grab_kernel("data/kernel".to_string());
}
