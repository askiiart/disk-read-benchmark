use curl::easy::Easy as easy_curl;
use rand::{self, RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;
use std::{
    env::current_dir,
    fs::{create_dir_all, exists, remove_dir_all, remove_file, File},
    io::{Error, Write},
    os::unix::fs::FileExt,
    process::Command,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

/*
===================
    ====                                                         ====
    ====                 ↓ DATASET GATHERING ↓                   ====
    ====                                                         ====
    =================================================================
*/
pub fn large_random_file_generation(path: String) {
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

pub fn _large_random_file_generation_helper(i: &u64, out: Arc<Mutex<Result<File, Error>>>) {
    let mut rng = XorShiftRng::seed_from_u64(2484345508);
    // NOTE: update this both here and in `large_random_file_generation()`
    let num_threads = 12;
    let mut data = [0u8; 1310720];
    let block_size = 1310720;

    // enter desired size in bytes, must be a multiple of 655360
    // this is not a typo, the extra zero after 65536is for the threads
    // 26843545600 = 25 GiB
    let blocks_per_thread: u64 = 26843545600 / (block_size * num_threads);
    for u in (i * blocks_per_thread)..((i + 1) * blocks_per_thread) {
        rng.fill_bytes(&mut data);

        //let offset: u64 = (i * blocks_per_thread * 1310720) + (1310720 * u);
        let offset: u64 = u * block_size;
        let mut out = out.lock().unwrap();
        out.as_mut().unwrap().write_all_at(&data, offset).unwrap();
    }
}

/*
pub fn single_threaded_large_random_file_generation(path: String) {
    let mut out = File::create(path).unwrap();
    let mut rng = XorShiftRng::seed_from_u64(2484345508);
    let mut data = [0u8; 65536];
    for _ in 0..409600 {
        rng.fill_bytes(&mut data);
        out.write_all(&data).unwrap();
    }
}
*/

pub fn small_random_files_generation(folder: String) {
    let mut rng = XorShiftRng::seed_from_u64(2484345508);
    let mut data: [u8; 1024] = [0u8; 1024];
    for i in 1..1025 {
        let mut out = File::create(format!("{folder}/{i}")).unwrap();
        rng.fill_bytes(&mut data);
        out.write_all(&data).unwrap();
    }
}

pub fn random_file_generator(path: String, size_mib: u64) {
    let mut out = File::create(path).unwrap();
    let mut rng = XorShiftRng::seed_from_u64(2484345508);

    let mut data = [0u8; 1310720];
    let block_size = 1310720;
    let blocks: u64 = (size_mib * 1024 * 1024) / block_size;

    for _ in 0..blocks {
        rng.fill_bytes(&mut data);
        out.write_all(&data).unwrap();
    }
}

pub fn create_null_file(path: String, size: u64) {
    let out = File::create(path).unwrap();
    out.write_all_at(&[0], size - 1).unwrap();
}

// no reason for it not to be multithreaded, but there's not much point either, it hardly takes any time... if anything, the overhead from multithreading might be worse?
pub fn small_null_files_generation(folder: String) {
    for i in 1..1025 {
        create_null_file(format!("{folder}/{i}"), 1024);
    }
}

pub fn grab_kernel(folder: String, kernel_version: String) -> Result<bool, String> {
    // maybe i should've just used reqwest, but that's no fun (also much more dependencies and stuff i'm sure)
    // NOTE: requires openssl-devel to be installed for compilation (presumably requires openssl-libs for execution)
    if !(exists(format!("{folder}/linux-{kernel_version}.tar.xz")).unwrap()) {
        let mut curl = easy_curl::new();
        curl.url(&format!(
            "https://cdn.kernel.org/pub/linux/kernel/v6.x/linux-{kernel_version}.tar.xz"
        ))
        .unwrap();
        curl.follow_location(true).unwrap();
        let mut out = File::create(format!("{folder}/linux-{kernel_version}.tar.xz")).unwrap();
        match curl.write_function(move |data| {
            out.write_all(data).unwrap();
            Ok(data.len())
        }) {
            Ok(_) => (),
            Err(e) => return Err(e.to_string()),
        }
        curl.perform().unwrap();
    }

    // i'm too lazy to do this in rust
    if !(exists(format!("{folder}/linux-{kernel_version}")).unwrap()) {
        let mut dir = current_dir().unwrap();
        dir.push(folder);
        match Command::new("tar")
            .current_dir(dir)
            .arg("-xf")
            .arg(&format!("linux-{kernel_version}.tar.xz"))
            .arg("")
            .output()
        {
            Ok(_) => (),
            Err(e) => return Err(e.to_string()),
        }
    }

    return Ok(true);
}

pub fn grab_datasets() -> Result<bool, String> {
    let kernel_version = "6.6.58";

    if !exists(format!("data/datasets/kernel/linux-{kernel_version}")).unwrap() {
        println!("Downloading kernel...");
        create_dir_all("data/datasets/kernel").unwrap();
        match grab_kernel(
            "data/datasets/kernel".to_string(),
            kernel_version.to_string(),
        ) {
            Ok(_) => (),
            Err(e) => {
                remove_dir_all(format!("data/datasets/kernel/linux-{kernel_version}")).unwrap();
                remove_file(format!(
                    "data/datasets/kernel/linux-{kernel_version}.tar.xz"
                ))
                .unwrap();
                panic!("{}", e.to_string());
            }
        }
        println!("Kernel downloaded");
    }

    if !exists(format!("data/datasets/25G-random.bin")).unwrap() {
        println!("Generating random 25 GiB file...");
        large_random_file_generation("data/datasets/25G-random.bin".to_string());
        println!("Random 25 GiB file generated");
    }

    if !exists(format!("data/datasets/small-files/random")).unwrap() {
        println!("Generating random 1 KiB files...");
        create_dir_all("data/datasets/small-files/random").unwrap();
        small_random_files_generation("data/datasets/small-files/random".to_string());
        println!("Random 1 KiB files generated...");
    }

    if !exists(format!("data/datasets/25G-null.bin")).unwrap() {
        println!("Generating null 25 GiB file...");
        create_null_file("data/datasets/25G-null.bin".to_string(), 26843545600);
        println!("Null 25 GiB file generated...");
    }

    if !exists("data/datasets/small-files/null").unwrap() {
        println!("Generating null 1 KiB files...");
        create_dir_all("data/datasets/small-files/null").unwrap();
        small_null_files_generation("data/datasets/small-files/null".to_string());
        println!("Null 1 KiB files generated...");
    }

    if !exists("data/datasets/100M-polygon.txt").unwrap() {
        return Err("*** MANUAL: Get 100M-sided regular polygon data and put it at `./data/datasets/100M-polygon.txt` ***".to_string());
    };

    return Ok(true);
}

pub fn prep_other_dirs() -> bool {
    if !exists("data/mountpoints").unwrap() {
        create_dir_all("data/mountpoints").unwrap();
    };

    return true;
}
