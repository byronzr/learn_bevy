use std::thread;

#[test]
fn test_memory_order() {
    // arm m3 test failed.
    use std::sync::atomic::AtomicUsize;
    let x: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));
    let y: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));
    let t1 = thread::spawn(move || {
        let r1 = y.load(std::sync::atomic::Ordering::Relaxed);
        //x.store(r1, std::sync::atomic::Ordering::Relaxed);
        x.store(42, std::sync::atomic::Ordering::Relaxed);
        r1
    });
    let t2 = thread::spawn(move || {
        let r2 = y.load(std::sync::atomic::Ordering::Relaxed);
        //x.store(42, std::sync::atomic::Ordering::Relaxed);
        x.store(r2, std::sync::atomic::Ordering::Relaxed);
        r2
    });
    let r1 = t1.join().unwrap();
    let r2 = t2.join().unwrap();
    println!("r1: {} / r2: {}", r1, r2);
}
