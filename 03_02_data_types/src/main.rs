fn main() {
    let _t8bit_unsigned_int: u8 = b'A';
    let _t16bit_unsigned_intt: u16 = 16;
    let _t8bit_signed_int: i8 = 0b1010;
    let _t32bit_signed_int: i32 = 0o77;
    let _t128bit_signed_int: i128 = 0xFF0000;

    let water = 2.0;
    let get_it: f32 = 4.2;
    let because_you_can_float_on_water: f32 = 12.9;

    let yes = false;

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("{}", heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup;
    let fivehundred = tup.0;

    let mut x: (i32, i32) = (1, 2);
    x.0 = 0;
    x.1 += 5;

    let a: [i32; 5] = [1, 2, 3, 4, 5];
}
