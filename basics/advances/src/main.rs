// unsafe fn dangerous() {}

// use std::slice;

// fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = values.len();
//     let ptr = values.as_mut_ptr();

//     assert!(mid <= len);

//     unsafe {
//         (
//             slice::from_raw_parts_mut(ptr, mid),
//             slice::from_raw_parts_mut(ptr.add(mid), len - mid),
//         )
//     }
// }

// unsafe extern "C" {
//     fn abs(input: i32) -> i32;
// }

// use std::fmt;

// struct Wrapper(Vec<String>);

// impl fmt::Display for Wrapper {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "[{}]", self.0.join(", "))
//     }
// }

// fn generic<T: fmt::Display>(t: T) {
//     println!("{}", t);
// }

type Add = fn(i32) -> i32;

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: Add, i: i32) -> i32 {
    f(i) + f(i)
}

fn main() {
    // let mut num = 5;

    // let r1 = &raw const num;
    // let r2 = &raw mut num;
    // r2 = 5;

    // unsafe {
    //     println!("r1 is: {}", *r1);
    //     println!("r2 is: {}", *r2);
    // }

    // unsafe {
    //     dangerous();
    // }

    // let mut v = vec![1, 2, 3, 4, 5, 6];
    // let r = &mut v[..];
    // // let (a, b) = r.split_at_mut(3);
    // let (a, b) = split_at_mut(r, 3);
    // assert_eq!(a, &mut [1, 2, 3]);
    // assert_eq!(b, &mut [4, 5, 6]);

    // unsafe { println!("Absolute value of - 3 according to C: {}", abs(-3)) }

    // let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    // println!("w = {w}");

    // type Kilometers = i32;

    // let x: i32 = 5;
    // let y: Kilometers = 5;

    // println!("x + y = {}", x + y);

    // generic(String::from("value"));
    // generic(2);

    let answer = do_twice(add_one, 1);
    println!("{}", answer);
}
