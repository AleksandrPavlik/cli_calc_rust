use std::io;

struct calc { first: i32, two: i32 }

impl calc {
    fn plus(&self ) -> i32 {
        self.first + self.two
    }

    fn minus(&self) -> i32 {
        self.first - self.two
    }

    fn multiplication(&self ) -> i32 {
        self.first * self.two
    }

    fn division(&self) -> i32 {
        self.first / self.two
    }
}

fn main() {
    let mut x = String::new();
    let mut z = String::new();

    let mut choose = String::new();

    println!("Введите первое число. ");

    io::stdin().read_line(&mut x).expect("panic!");

    println!("Введите второе число. ");

    io::stdin().read_line(&mut z).expect("panic!");

    println!("что будем делать?
    1.+
    2.-
    3.*
    4./");

    io::stdin().read_line(&mut choose).expect("panic!");

    let x: i32 = match x.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("err"),
    };
    let z: i32 = match z.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("err"),
    };

    let choose: i32 = match choose.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("err"),
    };

    let calculator = calc { first: x, two: z };

    match choose {
        1=> println!("{} + {} => {}", x, z, calculator.plus()),
        2=> println!("{} - {} => {}", x, z, calculator.minus()),
        3=> println!("{} * {} => {}", x, z, calculator.multiplication()),
        4=> println!("{} / {} => {}", x, z, calculator.division()),
        _=> panic!("err"),
    };
}
