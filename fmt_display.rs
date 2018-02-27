use std::fmt;

#[derive(Debug)]
struct MinMax(i64, i64);
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "({}, {})", self.0, self.1);
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "x: {}, y: {}", self.x, self.y);
    }
}
impl fmt::Binary for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let xp: *const u64 = &self.x as *const f64 as *const u64;
        let yp: *const u64 = &self.y as *const f64 as *const u64;

        unsafe {
        return write!(f, "x: {:b}, y: {:b}", *xp, *yp);
        }
    }
}

#[derive(Debug)]
struct Complex {
    re: f64,
    i: f64,
}
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{real}{imag:+}i",
            real = &self.re,
            imag = &self.i);
    }
}

fn main() {
    let minmax = MinMax(0, 15);
    println!("");
    println!("MinMax");
    println!("reg {}", minmax);
    println!("dbg {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3,3);
    println!("");
    println!("Big range is {big}. Small range is {small}",
        small = small_range,
        big = big_range);

    let point = Point2D{x: 3.2, y: 5.2};
    println!("");
    println!("Point2D");
    println!("reg {}", point);
    println!("dbg {:?}", point);
    println!("bin {:b}", point);

    let complex = Complex{re: 4.1, i:-7.4};
    println!("");
    println!("Complex");
    println!("reg {}", complex);
    println!("dbg {:?}", complex);

}
