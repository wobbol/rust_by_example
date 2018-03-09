fn main() {
    let x = 5u32;
    let y = {
        let x_2 = x * x;
        let x_3 = x_2 * x;
        x_2 + x_3 + x
    };
    let z: i64 = {
        x as i64
    };
    println!("xis {:?}", x);
    println!("yis {:?}", y);
    println!("zis {:?}", z);


    let mut count = 0u32;
    let mut big_z = z;
    let z_result = loop {

        let mut z_small: bool = big_z < 10 && big_z > -10;
        big_z =
            if z_small {
                println!("z is small, make it bigger.");

                10 * big_z
            } else {
                println!("z is big, make it smaller.");
                big_z / 2
            };
        count += 1;
        if count == 20 { break big_z; }
    }
        println!("zresult {}",z_result);

    'outer: loop {
        println!("outerloop");
        'inner: loop {
            println!("innerloop");
            break 'outer;
        }
        println!("unreachable");
    }
    println!("outerloop exit");


}
