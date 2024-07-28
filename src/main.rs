use num::complex::Complex;

// fn main() {
//     let southern_germany = "Grüß Gott!";
//     let chinese = "世界，你好！";
//     let english = "World, hello!";
//     let regions = [southern_germany, chinese, english];
//     for region in regions {
//         println!("{}", region);
//     }
// }

// fn main() {
//     let penguin_data = "\
//         common name, length (cm)
//         Little penguin, 33
//         Yellow-eyed penguin, 65
//         Fiordland penguin, 60
//         Invalid, data
//     ";
//     let records = penguin_data.lines();

//     for (i, record) in records.enumerate() {
//         if i == 0 || record.trim().len() == 0 {
//             continue;
//         }
//         let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();
//         if cfg!(debug_assertions) {
//             println!("debbug: {:?} -> {:?}", record, fields);
//         }

//         let name = fields[0];

//         if let Ok(length) = fields[1].parse::<f32>() {
//             println!("{}, {}cm", name, length);
//         }
//     }
// }

// fn add(i: i32, j: i32) -> i32 {
//     i + j
// }

// struct Struct {
//     e: i32,
// }

fn main() {
    // let _x = 1;
    // let a = 10;
    // let b: i32 = 20;
    // let mut c = 30i32;
    // c += 1;
    // let d = 30_i32;
    // let e = add(add(a, b), add(c, d));

    // println!("(a + b) + (c + d) = {}", e);

    // let (a, mut b) = (true, false);
    // println!("a = {:?}, b = {:?}", a, b);

    // b = true;
    // assert_eq!(a, b);

    // let (a, b, c, d, e);

    // (a, b) = (1, 2);

    // [c, .., d, _] = [1, 2, 3, 4, 5];

    // Struct { e, .. } = Struct { e: 5 };

    // assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);

    // let x = 5;
    // let x = x + 1;

    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope os {}", x);
    // }

    // println!("The value of x is {}", x);

    // let spaces = "   ";
    // let spaces = spaces.len();

    // println!("The length of the space is {}", spaces);

    // let (aa, .., bb, _) = (1, 2, 3, 4);
    // println!("aa: {}, bb: {}", aa, bb);

    // let guesss: u32 = "42".parse().expect("Not a number!");

    // println!("The answer is {}", guesss);

    // let a: u8 = 255;
    // let b = a.wrapping_add(20);
    // println!("a: {}, b: {}", a, b);

    // assert_eq!(100u8.saturating_add(1), 101);
    // assert_eq!(u8::MAX.saturating_add(127), u8::MAX);
    // assert!(0.1 + 0.2 == 0.3);

    // let x: f32 = (-42.0_f32).sqrt();
    // // assert_eq!(x, x);
    // if x.is_nan() {
    //     println!("未定义的数学行为");
    // }

    // let forty_twos = [42.0, 42f32, 42.0_f32];

    // println!("{:.3}", forty_twos[2]);

    // 位运算
    // let a: i32 = 2; // 二进制为00000010
    // let b: i32 = 3; // 二进制为00000011

    // // 按位与
    // println!("(a & b) value is {}", a & b); // 2
    //                                         // 解释：00000010 & 00000011 = 00000010 (即2)

    // // 按位或
    // println!("(a | b) value is {}", a | b); // 3
    //                                         // 解释：00000010 | 00000011 = 00000011 (即3)

    // // 按位异或
    // println!("a ^ b value is {}", a ^ b); // 1
    //                                       // 解释：00000010 ^ 00000011 = 00000001 (即1)

    // // 按位取反
    // println!("(!b) value is {}", !b);
    // // 解释：!00000011 = 11111100 (即-4, 注意这是按位取反后的补码形式)

    // // 原数： 11111100
    // // 取反： 00000011
    // // 加1： 00000011 + 1 = 00000100
    // // 结果：原码 00000100 是 4，符号位是负号，所以是 -4。

    // // 左移
    // println!("a << b value is {}", a << b);
    // // 解释：2 << 3 = 00000010 左移 3 位 = 00010000 (即16)

    // // 右移
    // println!("a >> b value is {}", a >> b);
    // // 解释：2 >> 3 = 00000010 右移 3 位 = 00000000 (即0)

    // // 使用 <<= 进行左移赋值
    // let mut a = a;

    // a <<= b;
    // println!("a << b value is {}", a);
    // // 解释：2 <<= 3 = 2 左移 3 位 = 16

    // 序列
    // for i in 1..=5 {
    //     println!("{}", i);
    // }

    // for i in 'a'..='z' {
    //     println!("{}", i);
    // }

    // 有理数和复数
    // let a = Complex { re: 2.1, im: -1.2 };
    // let b = Complex::new(11.1, 22.2);
    // let result = a + b;

    // println!("{} + {}i", result.re, result.im);

    // let a = 13.54_f32.round();
    // println!("{}", a);

    let c = 'z';
    let z = 'ℤ';
    let g = '国';
    let heart_eyed_cat = '😻';
    println!("字符'中'占用了{}字节的内存大小", std::mem::size_of_val(&g));
}
