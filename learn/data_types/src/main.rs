fn main() {
    // Floating-Point Types
    {
        let x = 2.0; // f64
        let y: f32 = 3.0; // f32
    }
    // The Boolean Type
    {
        let t = true;
        let f: bool = false; // with explicit type annotation
    }
    // The Character Type
    {
        let c = 'z';
        let z: char = 'ℤ'; // with explicit type annotation
        let heart_eyed_cat = '😻';
    }
    // The Tuple Type
    {
        let tup: (i32, f64, u8) = (500, 6.4, 1);
    }
    {
        let tup = (500, 6.4, 1);
        let (x, y, z) = tup;
        println!("The value of y is: {y}");
    }
    {
        let x: (i32, f64, u8) = (500, 6.4, 1);
        let five_hundred = x.0;
        let six_point_four = x.1;
        let one = x.2;
    }
    // The Array Type
    {
        let a = [1, 2, 3, 4, 5];
        let first = a[0];
        let second = a[1];
    }
    {
        let a: [i32; 5] = [1, 2, 3, 4, 5];
    }
    {
        let a = [3; 5]; // let a = [3, 3, 3, 3, 3];
    }

}
