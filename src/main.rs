use curl::easy::Easy as easy_curl;
use rand::{self, Rng, RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;
use std::io::Read;
use std::time::{Duration, Instant};
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
    =================================================================
    ====                                                         ====
    ====                 ↓ DATASET GATHERING ↓                   ====
    ====                                                         ====
    =================================================================
*/

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
    for i in 1..1025 {
        let mut out = File::create(format!("{folder}/{i}")).unwrap();
        rng.fill_bytes(&mut data);
        out.write_all(&data).unwrap();
    }
}

fn random_file_generator(path: String, size_mib: u64) {
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

fn create_null_file(path: String, size: u64) {
    let out = File::create(path).unwrap();
    out.write_all_at(&[0], size - 1).unwrap();
}

// no reason for it not to be multithreaded, but there's not much point either, it hardly takes any time... if anything, the overhead from multithreading might be worse?
fn small_null_files_generation(folder: String) {
    for i in 1..1025 {
        create_null_file(format!("{folder}/{i}"), 1024);
    }
}

fn grab_kernel(folder: String, kernel_version: String) -> Result<bool, String> {
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

fn grab_datasets() -> Result<bool, String> {
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

fn prep_other_dirs() -> bool {
    if !exists("data/ext-workdir").unwrap() {
        create_dir_all("data/ext-workdir").unwrap();
    };

    if !exists("data/benchmark-workdir").unwrap() {
        create_dir_all("data/benchmark-workdir").unwrap();
    }

    if !exists("data/mountpoints").unwrap() {
        create_dir_all("data/mountpoints").unwrap();
    };

    return true;
}

/*
    =================================================================
    ====                                                         ====
    ====                     ↓ BENCHMARKS ↓                      ====
    ====                                                         ====
    =================================================================
*/

fn sequential_read(path: String) -> Duration {
    let mut f: File = File::open(path).unwrap();
    let size = f.metadata().unwrap().len();

    let mut data: [u8; 1310720] = [0u8; 1310720];
    // benchmarking/elapsed: https://stackoverflow.com/a/40953863
    let now = Instant::now();
    for _ in 0..(size / 1310720) {
        f.read(&mut data).unwrap();
    }
    let elapsed = now.elapsed();
    return elapsed;
}

/// Reads 1 byte from the start of file
fn sequential_read_latency(path: String) -> Duration {
    let mut f: File = File::open(path).unwrap();
    let mut data: [u8; 1] = [0u8; 1];
    let now = Instant::now();
    f.read(&mut data).unwrap();
    let elapsed = now.elapsed();
    return elapsed;
}

/// Reads 1 GiB from the file at `path` in random 1 MiB chunks
fn random_read(path: String) -> Duration {
    let mut rng = XorShiftRng::seed_from_u64(9198675309);
    let f: File = File::open(path).unwrap();
    let size = f.metadata().unwrap().len();

    let mut data: [u8; 1048576] = [0u8; 1048576];
    let now = Instant::now();
    for _ in 0..1024 {
        let offset = rng.gen_range(0..((size - 1048576) / 1048576));
        f.read_at(&mut data, offset).unwrap();
    }
    let elapsed = now.elapsed();
    return elapsed;
}

/// Reads 1 random byte from the file at `path` 1024 times
fn random_read_latency(path: String) -> Duration {
    let mut rng = XorShiftRng::seed_from_u64(9198675309);
    let f: File = File::open(path).unwrap();
    let size = f.metadata().unwrap().len();
    let mut data: [u8; 1] = [0u8; 1];
    let now = Instant::now();
    for _ in 0..1024 {
        let offset = rng.gen_range(0..(size - 1));
        f.read_at(&mut data, offset).unwrap();
    }
    let elapsed = now.elapsed();
    return elapsed;
}

fn bulk_sequential_read(path: String) -> Vec<Duration> {
    let mut data: [u8; 1024] = [0u8; 1024];
    let mut times: Vec<Duration> = Vec::new();
    for i in 1..1025 {
        let mut f: File = File::open(format!("{path}/{i}")).unwrap();
        let now = Instant::now();
        f.read(&mut data).unwrap();
        let elapsed = now.elapsed();
        times.push(elapsed);
    }

    return times;
}

fn bulk_sequential_read_latency(path: String) -> Vec<Duration> {
    let mut data: [u8; 1] = [0u8; 1];
    let mut times: Vec<Duration> = Vec::new();
    for i in 1..1025 {
        let now = Instant::now();
        let mut f: File = File::open(format!("{path}/{i}")).unwrap();
        f.read(&mut data).unwrap();
        let elapsed = now.elapsed();
        times.push(elapsed);
    }

    return times;
}


fn benchmark() {
    let mut recorder = csv::Writer::from_path("data/benchmark-data.csv").unwrap();
    let mut bulk_recorder = csv::Writer::from_path("data/bulk.csv").unwrap();
    let mountpoint_dir = "data/mountpoints";
    let mut filesystems = std::fs::read_dir(mountpoint_dir)
        .unwrap()
        .map(|item| {
            let tmp = item.unwrap().file_name().into_string().unwrap();
            format!("{mountpoint_dir}/{tmp}")
        })
        .collect::<Vec<String>>();

    filesystems.push("data/datasets".to_string());

    for fs in filesystems {
        let single_files = vec![
            "25G-null.bin".to_string(),
            "25G-random.bin".to_string(),
            "100M-polygon.txt".to_string(),
            "kernel/linux-6.6.58.tar.xz".to_string(),
        ];

        let bulk_files = vec!["small-files/null", "small-files/random"];

        for filename in single_files {
            println!("=== {} ===", filename);

            let path = format!("{fs}/{filename}");
            println!("{}", path);
            //panic!("hi");

            let seq_read = format!("{:.5?}", sequential_read(path.clone()));
            println!("Sequential read (complete file read): {}", seq_read.clone());

            let seq_latency = format!("{:.5?}", sequential_read_latency(path.clone()));
            println!("Sequential latency (1 byte read): {}", seq_latency);

            let rand_read = format!("{:.5?}", random_read(path.clone()));
            println!("Random read (1024x 1 MiB): {}", rand_read);

            let rand_latency = format!("{:.5?}", random_read_latency(path.clone()));
            println!("Random latency (1024x 1 byte read): {}", rand_latency);

            let data: Vec<String> = vec![
                fs.clone(),
                filename,
                seq_read,
                seq_latency,
                rand_read,
                rand_latency,
            ];
            recorder.write_record(data).unwrap();

            println!();
        }

        for folder in bulk_files {
            bulk_recorder.write_record(_vec_duration_to_string(bulk_sequential_read(folder.to_string()))).unwrap();
            bulk_recorder.write_record(_vec_duration_to_string(bulk_sequential_read_latency(folder.to_string()))).unwrap();
            //bulk_recorder.write_record(_vec_duration_to_string(bulk_random_read(folder.to_string()))).unwrap();
            //bulk_recorder.write_record(_vec_duration_to_string(bulk_random_read_latency(folder.to_string()))).unwrap();
        }

        println!("=== === === === === === === === === === ===\n")
    }
}

fn main() {
    grab_datasets().unwrap();
    prep_other_dirs();
    benchmark();
}

fn _vec_duration_to_string(vector_committing_crimes_with_both_direction_and_magnitude: Vec<Duration>) -> Vec<String> {
    return vector_committing_crimes_with_both_direction_and_magnitude.iter()
    .map(|item| {
        format!("{:.5?}", item)
    })
    .collect::<Vec<String>>();

}
