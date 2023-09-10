use std::sync::{Arc, mpsc};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;

use sha256::digest;

use crate::types::Args;


pub fn find_all_hash(cpus: usize, params: Args) -> Vec<(usize, String)> {
    let numbers = params.numbers;
    let mut results: Vec<(usize, String)> = Vec::new();

    let finished = Arc::new(AtomicBool::new(false));
    let current_next = Arc::new(AtomicUsize::new(1));
    let (tx, rx) = mpsc::channel();

    for _ in 0..cpus {
        let finished = finished.clone();
        let current_next = current_next.clone();
        let tx = tx.clone();
        thread::spawn(move || {
            while !finished.load(Ordering::Relaxed) {
                let current = current_next.fetch_add(1, Ordering::Relaxed);
                test_max_value(current, &finished);
                let (count, hash) = calculate_hash(current);
                if count == numbers {
                    tx.send((current, hash)).unwrap();
                }
            }
        });
    }

    drop(tx);

    while let Ok((val, hash)) = rx.recv() {
        results.push((val, hash));
        if results.len() == params.find {
            finished.store(true, Ordering::Relaxed);
        }
    }

    prepare_results(results, params.find)
}

fn test_max_value(current: usize, finished: &Arc<AtomicBool>) {
    if current == usize::MAX {
        finished.store(true, Ordering::Relaxed);
    }
}

fn prepare_results(results: Vec<(usize, String)>, max: usize) -> Vec<(usize, String)> {
    let mut results = results;
    results.sort_by(|(a, _), (b, _)| a.cmp(&b));
    if results.len() > max {
        results[..max].iter().map(|(a, b)| (*a, b.to_string())).collect::<Vec<_>>()
    } else {
        results
    }
}

fn calculate_hash(val: usize) -> (usize, String) {
    let res = digest(val.to_string());
    let mut count = 0;
    let mut list = res.chars().rev();
    while list.next() == Some('0') {
        count += 1;
    }
    (count, res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_hash() {
        assert_eq!(calculate_hash(0).0, 0);
        assert_eq!(calculate_hash(23).0, 1);
    }

    #[test]
    fn test_find_all_hash() {
        let result = vec![
            (23, "535fa30d7e25dd8a49f1536779734ec8286108d115da5045d77f3b4185d8f790".to_string()),
            (38, "aea92132c4cbeb263e6ac2bf6c183b5d81737f179f21efdc5863739672f0f470".to_string()),
        ];
        assert_eq!(find_all_hash(1, Args { find: 2, numbers: 1 }), result);
    }

    #[test]
    fn test_test_max_value() {
        let finished = Arc::new(AtomicBool::new(false));
        test_max_value(usize::MAX - 1, &finished);
        assert_eq!(finished.load(Ordering::Relaxed), false);
        test_max_value(usize::MAX, &finished);
        assert_eq!(finished.load(Ordering::Relaxed), true);
    }

    #[test]
    fn test_prepare_results() {
        let r = vec![
            (1, "1".to_string()),
            (2, "2".to_string())
        ];
        assert_eq!(prepare_results(r.clone(), 2), r.clone());
        assert_eq!(prepare_results(r.clone(), 1), vec![r[0].clone()]);
    }
}