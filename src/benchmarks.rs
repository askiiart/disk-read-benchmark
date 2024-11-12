use rand::{self, Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use std::io::Read;
use std::time::{Duration, Instant};
use std::{fs::File, os::unix::fs::FileExt};

/*
    =================================================================
    ====                                                         ====
    ====                     ↓ BENCHMARKS ↓                      ====
    ====                                                         ====
    =================================================================
*/

pub fn sequential_read(path: String) -> Duration {
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
pub fn sequential_read_latency(path: String) -> Duration {
    let mut f: File = File::open(path).unwrap();
    let mut data: [u8; 1] = [0u8; 1];
    let now = Instant::now();
    f.read(&mut data).unwrap();
    let elapsed = now.elapsed();
    return elapsed;
}

/// Reads 1 GiB from the file at `path` in random 1 MiB chunks
pub fn random_read(path: String) -> Duration {
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
pub fn random_read_latency(path: String) -> Duration {
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

pub fn bulk_sequential_read(path: String) -> Vec<Duration> {
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

pub fn bulk_sequential_read_latency(path: String) -> Vec<Duration> {
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

pub fn bulk_random_read_latency(path: String) -> Vec<Duration> {
    let mut rng = XorShiftRng::seed_from_u64(9198675309);
    let mut data: [u8; 1] = [0u8; 1];
    let mut times: Vec<Duration> = Vec::new();
    for i in 1..1025 {
        let f: File = File::open(format!("{path}/{i}")).unwrap();
        let offset = rng.gen_range(0..1023);
        let now = Instant::now();
        f.read_at(&mut data, offset).unwrap();
        let elapsed = now.elapsed();
        times.push(elapsed);
    }

    return times;
}

pub fn benchmark() {
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
        let bulk_files: Vec<String> = vec![
            "small-files/null".to_string(),
            "small-files/random".to_string(),
        ];

        for filename in single_files {
            let path = format!("{fs}/{filename}");
            println!("=== {} ===", path.clone());

            let seq_read = format!("{:.5?}", sequential_read(path.clone()));
            println!("Sequential read (complete file read): {}", seq_read.clone());

            let seq_latency = format!("{:.5?}", sequential_read_latency(path.clone()));
            println!("Sequential latency (1 byte read): {}", seq_latency);

            let rand_read = format!("{:.5?}", random_read(path.clone()));
            println!("Random read (1024x 1 MiB): {}", rand_read);

            let mut rand_latency: String = "0s".to_string();
            if fs != "data/mountpoints/fuse-archive-tar" {
                rand_latency = format!("{:.5?}", random_read_latency(path.clone()));
            }

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

        // bulk files
        for folder in bulk_files {
            let cloned = fs.clone();
            let path = format!("{cloned}/{folder}");
            println!("[bulk] Testing {}", path);
            let dataset_info: Vec<String> = vec![fs.clone(), folder];

            let mut times = _vec_duration_to_string(bulk_sequential_read(path.clone()));
            let mut tmp = Vec::new();
            dataset_info.clone_into(&mut tmp);
            tmp.push("bulk_sequential_read".to_string());
            tmp.append(&mut times);
            bulk_recorder.write_record(tmp).unwrap();

            times = _vec_duration_to_string(bulk_sequential_read_latency(path.clone()));
            tmp = Vec::new();
            dataset_info.clone_into(&mut tmp);
            tmp.push("bulk_sequential_read_latency".to_string());
            tmp.append(&mut times);
            bulk_recorder.write_record(tmp).unwrap();

            // not enough data in these files to warrant bulk_random_read()
            //bulk_recorder.write_record(_vec_duration_to_string(bulk_random_read(path.clone()))).unwrap();
            times = _vec_duration_to_string(bulk_random_read_latency(path.clone()));
            tmp = Vec::new();
            dataset_info.clone_into(&mut tmp);
            tmp.push("bulk_random_read_latency".to_string());
            tmp.append(&mut times);
            bulk_recorder.write_record(tmp).unwrap();
        }
        println!("\n=== === === === === === === === === === ===\n")
    }
}

pub fn _vec_duration_to_string(
    vector_committing_crimes_with_both_direction_and_magnitude: Vec<Duration>,
) -> Vec<String> {
    return vector_committing_crimes_with_both_direction_and_magnitude
        .iter()
        .map(|item| format!("{:.5?}", item))
        .collect::<Vec<String>>();
}
