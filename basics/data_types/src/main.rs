// fn main() {
//     let x: i32 = 5;
//     let x = plus(x);
//     println!("Number: {x}");

//     for i in (1..4).rev() {
//         println!("Loop: {i}");
//     }

//     let mut r = 0;
//     while r < 3 {
//         println!("Runner: {r}");
//         r = r + 1;
//     }

//     let a: [i32; 5] = [1, 2, 3, 4, 5];
//     for e in a {
//         println!("array: {e}")
//     }
// }

// fn plus(x: i32) -> i32 {
//     return x + 1;
// }

// fn main() {
//     let s = String::from("hello word");

//     let word = first_word(&s);

//     println!("word: {word}");
// }

// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     return &s[..];
// }

fn main() {
    let r;

    {
        let x = 5;
        r = x;
    }

    println!("r is {r}");
}
