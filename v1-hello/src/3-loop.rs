pub struct Stepper {
    curr: i32,
    step: i32,
    max: i32,
}

impl Iterator for Stepper {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if self.curr >= self.max {
            return None;
        }
        let res = self.curr;
        self.curr += self.step;
        Some(res)
    }
}

fn main() {
    /*
    let mut st = Stepper {
        curr: 0,
        step: 1,
        max: 10,
    };

    loop {
        match st.next() {
            Some(v) => println!("loop {}", v),
            None => break,
        }
    }
    */

    /*
    let mut st = Stepper {
        curr: 0,
        step: 1,
        max: 10,
    };

    while let Some(n) = st.next() {
        println!("While, {}", n);
    }
    */

    let it = Stepper {
        curr: 5,
        step: 10,
        max: 50,
    };

    for i in it {
        println!("For loop {}", i);
    }

    /*
    let mut n = 0;
    // loop {
    //     n += 1;
    //     if n > 10 {
    //         break;
    //     }

    //     println!("Hello {}!", n);
    // }
    while n < 10 {
        n += 1;
        println!("Hello {}!", n);
    }

    for i in 0..10 {
        println!("Hello {}", i);
    }
    */

    println!("All Done!");
}
