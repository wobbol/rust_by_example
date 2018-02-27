fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (i, b) = pair;
    return (b, i);
}
fn transpose(mat: Matrix) -> Matrix {
    return Matrix(mat.0, mat.2, mat.1, mat.3);
}


#[derive(Debug)]
struct Matrix(f32,f32,f32,f32);
impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f,"( {} {} )\n( {} {} )",
            self.0,
            self.1,
            self.2,
            self.3);
    }
}

fn main() {
    // One more element in this tuple and it would be unprintable.
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64,
        -1i8, -2i16, -3i32, -4i64,
        0.1f32, 0.2f64,
        'a', true);
    println!("");
    println!("long tuple first: {}", long_tuple.0);
    println!("long tuple second: {}", long_tuple.1);
    println!("{:?}", long_tuple);

    let tuple_of_tuples = ((1u8, 2u16,2u32), (4u64, -1i8), -2i16);
    println!("");
    println!("tuple of tuples {:?}", tuple_of_tuples);

    let pair = (1, true);
    println!("");
    println!("pear is {:?}", pair);
    println!("raep is {:?}", reverse(pair));

    let matrix = Matrix(1.1,1.2,2.1,2.2);
    println!("");
    println!("Matrix");
    println!("{}", matrix);
    println!("{:?}", matrix);
    println!("Transpose:\n{}", transpose(matrix));
}
