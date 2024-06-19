use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    mutex();
    multiples();
}

fn mutex() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {m:?}");
}

fn multiples() {
    // This doesn't work by itself, gives an ownership error when trying to access the value at the end
    //let counter = Mutex::new(0);

    // Also doesn't work because Rc cannot be sent safely between threads
    //let counter = Rc::new(Mutex::new(0));

    // Instead, let's use an *atomic* reference count
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // This is also necessary so we can use the local counter
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}