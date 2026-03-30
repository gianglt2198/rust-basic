// #[derive(Debug)]
// enum List {
//     Cons(Rc<RefCell<i32>>, Rc<List>),
//     Nil,
// }

// use crate::List::{Cons, Nil};

// use std::ops::Deref;

// struct MyBox<T>(T);

// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }

// impl<T> Deref for MyBox<T> {
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// use std::mem::drop;
// use std::ops::Drop;

// struct CustomSmartPointer {
//     data: String,
// }

// impl Drop for CustomSmartPointer {
//     fn drop(&mut self) {
//         println!("Dropping Custom Smart Pointer with data {}", self.data);
//     }
// }

use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

#[derive(Debug)]
struct Node {
    // value: i32,
    parent: RefCell<Weak<Node>>,
    // children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    // let b = Box::new(5);
    // println!("b = {b}");

    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // let x = 5;
    // let y = MyBox::new(5);

    // assert_eq!(5, x);
    // assert_eq!(5, *y);

    // let c = CustomSmartPointer {
    //     data: String::from("Game so easy"),
    // };
    // let d = CustomSmartPointer {
    //     data: String::from("So hard"),
    // };
    // println!("CustomSmartPointers created");
    // drop(d);
    // println!("CustomSmartPointer dropped before the end of main");

    // let a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Nil)))));
    // println!("count after creating a = {}", Rc::strong_count(&a));
    // let b = Cons(3, Rc::clone(&a));
    // println!("count after creating b = {}", Rc::strong_count(&a));
    // {
    //     let c = Cons(4, Rc::clone(&a));
    //     println!("count after creating c = {}", Rc::strong_count(&a));
    // }
    // println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    // let value = Rc::new(RefCell::new(5));

    // let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    // let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    // let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // *value.borrow_mut() += 10;

    // println!("a after = {a:?}");
    // println!("b after = {b:?}");
    // println!("c after = {c:?}");

    let leaf = Rc::new(Node {
        // value: 3,
        parent: RefCell::new(Weak::new()),
        // children: RefCell::new(vec![]),
    });

    // println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {} weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    {
        let branch = Rc::new(Node {
            // value: 5,
            parent: RefCell::new(Weak::new()),
            // children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
