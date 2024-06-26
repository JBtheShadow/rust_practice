use core::slice;

fn main() {
    raw_pointers();
    functions_methods();
    safe_abstractions_over_unsafe_code();
    extern_functions();
    static_variables();
    unsafe_traits();
    unions();
}

fn raw_pointers() {
    let mut num = 5;

    // an immutable raw pointer
    let r1 = &num as *const i32;

    // a mutable raw pointer
    let r2 = &mut num as *mut i32;

    // Both pointers can be created outside of an unsafe block.
    // They just cannot be dereferenced unless inside one
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
    // The pointers above also have been created via casting references
    // which are guaranteed to be valid
    //
    // Also of note, Rust allowed us to create both mutable and immutable
    // pointers to the same memory address. Unsafe Rust won't prevent you
    // from creating any number of pointers this way, but how you access
    // or mutate their values could cause a data race, beware

    // The pointer below can't be guaranteed though
    let address = 0x012345usize;
    let _r = address as *const i32;
}

fn functions_methods() {
    unsafe {
        dangerous();
    }
}

unsafe fn dangerous() {}

fn safe_abstractions_over_unsafe_code() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    // Rust doesn't allow this in safe code because there are two borrows from the same reference
    // bus since what we want to do here is work with two non overlapping regions of memory
    // it should still be ok
    //(&mut values[..mid], &mut values[mid..])

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn extern_functions() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

// Reference a function from the "C" application binary interface (ABI)
extern "C" {
    fn abs(input: i32) -> i32;
}

// Expose a function call_from_c() to be usable within the "C" ABI
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

// Static immutable variable access is safe
static HELLO_WORLD: &str = "Hello, world!";
// Static mutable variable access is unsafe
static mut COUNTER: u32 = 0;

fn static_variables() {
    println!("name is: {HELLO_WORLD}");

    add_to_count(3);
    unsafe {
        println!("COUNTER: {COUNTER}");
    }
}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn unsafe_traits() {
    unsafe trait _Foo {

    }

    unsafe impl _Foo for i32 {

    }
}

// This is mostly when interfacing with C code where union types are common
fn unions() {
    #[repr(C)]
    union MyUnion {
        f1: u32,
        f2: f32,
    }

    let u = MyUnion { f1: 1 };

    let _f = unsafe { u.f1 };

    unsafe {
        match u {
            MyUnion { f1: 10 } => { println!("ten"); }
            MyUnion { f2 } => { println!("{}", f2); }
        }
    }

    #[repr(u32)]
    enum _Tag { I, F }

    #[repr(C)]
    union _U {
        i: i32,
        f: f32,
    }

    #[repr(C)]
    struct _Value {
        tag: _Tag,
        u: _U,
    }

    fn _is_zero(v: _Value) -> bool {
        unsafe {
            match v {
                _Value { tag: _Tag::I, u: _U { i: 0 } } => true,
                _Value { tag: _Tag::F, u: _U { f: num } } if num == 0.0 => true,
                _ => false,
            }
        }
    }
}
