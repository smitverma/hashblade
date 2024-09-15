use std::io::Write;
use crypto::{
    md5::Md5,
    sha1::Sha1,
    sha2::Sha224,
    sha2::Sha256,
    sha2::Sha384,
    sha2::Sha512,
    sha3::Sha3Mode::Sha3_224,
    sha3::Sha3Mode::Sha3_256,
    sha3::Sha3Mode::Sha3_384,
    sha3::Sha3Mode::Sha3_512,
    ripemd160::Ripemd160,
    whirlpool::Whirlpool,
    sha3::Sha3,
    digest::Digest,
};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;


pub fn all_crack(words: Vec<String>, orig_hash: String) {
        let num_threads = 8;
    let chunk_size = words.len() / num_threads + 1;
    let words_arc = Arc::new(words);
    let orig_hash_arc = Arc::new(orig_hash);
    let result = Arc::new(Mutex::new(None));
    let progress = Arc::new(AtomicUsize::new(0));
    let solution_found = Arc::new(AtomicBool::new(false));
    let total_words = words_arc.len();

    let hash_functions_names: Vec<&str> = vec![
        "Md5", "Sha1", "Sha224", "Sha256", "Sha384", "Sha512",
        "Sha3_224", "Sha3_256", "Sha3_384", "Sha3_512",
        "Ripemd160", "Whirlpool",
    ];

    // Spawn a thread to update progress
    let progress_clone = Arc::clone(&progress);
    let solution_found_clone = Arc::clone(&solution_found);
    let progress_handle = thread::spawn(move || {
        while !solution_found_clone.load(Ordering::Relaxed) && progress_clone.load(Ordering::Relaxed) < total_words {
            let current_progress = progress_clone.load(Ordering::Relaxed);
            print!("\r    Progress: {}/{}", current_progress, total_words);
            std::io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(100)); // Update every 100ms
        }
        print!("\rProgress: {}/{}\n", progress_clone.load(Ordering::Relaxed), total_words);
    });

    let mut handles = vec![];

    for t in 0..num_threads {
        let words = Arc::clone(&words_arc);
        let orig_hash = Arc::clone(&orig_hash_arc);
        let result = Arc::clone(&result);
        let progress = Arc::clone(&progress);
        let solution_found = Arc::clone(&solution_found);
        let hash_functions_names = hash_functions_names.clone();

        let handle = thread::spawn(move || {
            let start = t * chunk_size;
            let end = std::cmp::min((t + 1) * chunk_size, words.len());

            let mut hash_functions: Vec<Box<dyn Digest>> = vec![
                Box::new(Md5::new()), Box::new(Sha1::new()),
                Box::new(Sha224::new()), Box::new(Sha256::new()),
                Box::new(Sha384::new()), Box::new(Sha512::new()),
                Box::new(Sha3::new(Sha3_224)), Box::new(Sha3::new(Sha3_256)),
                Box::new(Sha3::new(Sha3_384)), Box::new(Sha3::new(Sha3_512)),
                Box::new(Ripemd160::new()), Box::new(Whirlpool::new()),
            ];

            for word in &words[start..end] {
                if solution_found.load(Ordering::Relaxed) {
                    return;
                }
                progress.fetch_add(1, Ordering::Relaxed);
                for (i, hash_function) in hash_functions.iter_mut().enumerate() {
                    hash_function.input_str(word);
                    let result_str = hash_function.result_str();
                    if result_str == *orig_hash {
                        let mut result_guard = result.lock().unwrap();
                        *result_guard = Some((hash_functions_names[i].to_string(), word.clone()));
                        solution_found.store(true, Ordering::Relaxed);
                        return;
                    }
                    hash_function.reset();
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    progress_handle.join().unwrap();

    let result_guard = result.lock().unwrap();
    match &*result_guard {
        Some((hash_type, password)) => {
            println!("Hash cracked! Hash used: {}\nPassword = {}", hash_type, password);
        }
        None => {
            println!("Hash not cracked. No password matched.");
        }
    }
}