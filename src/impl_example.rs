struct Retangle {
    width: u32,
    height: u32,
}

impl Retangle {
    fn description(&self) {
        println!("Retangle: {} x {}", self.width, self.height);
    }

    fn is_square(&self) -> bool {
        return self.width == self.height;
    }
}

pub fn run() {
    let my_ret = Retangle {
        width: 5,
        height: 5,
    };
    my_ret.description();
    println!("{}", my_ret.is_square());
}
