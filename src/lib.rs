// Iterating Natural Numbers

use std::i32;

struct N {
    n: i32,
}

impl N {
    fn new() -> N {
        N {
            n: 0
        }
    }
}

impl Iterator for N {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.n += 1;

        if self.n <= i32::MAX {
            Some(self.n)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::N;

    #[test]
    fn it_works() {
        println!("test starts");

        let mut n = N::new();

        println!("{:?}", n.next());
        println!("{:?}", n.next());
        println!("{:?}", n.next());
        println!("{:?}", n.next());
    }

    #[test]
    fn test0() {
        let mut n = N::new();
        let mut n_4 = n.take(4);

        println!("{:?}", n_4.next());
        println!("{:?}", n_4.next());
        println!("{:?}", n_4.next());
        println!("{:?}", n_4.next());
        println!("{:?}", n_4.next());
    }

    #[test]
    fn test1() {
        let mut n = N::new();
        let mut n_8 = n.take(8);
        //let mut v: Vec<i32> = n_8.collect();
        let mut v = n_8.collect::<Vec<_>>();  // turbofish ::<>
        // Iterators are lazy and collect() runs them.

        println!("{:?}", v);
    }

    #[test]
    fn test2() {
        let mut n_iter_0 = N::new();
        let mut n_4_0 = n_iter_0.take(4);
        let mut v0 = n_4_0.collect::<Vec<_>>();
        println!("{:?}", v0);

        let mut n_iter_1 = N::new();
        let mut n_4_1 = n_iter_1.skip(1).take(4);
        let mut v1 = n_4_1.collect::<Vec<_>>();
        println!("{:?}", v1);
    }
}
