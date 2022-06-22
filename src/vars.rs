pub mod sub_a;
pub mod sub_b;

const MAX_POINT: u32 = 100_000;

pub fn run() {
    println!("Hello, vars!");
    // sub_a::func_a();
    // sub_b::func_b();
    let mut x  = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    let _i1 = 3;
    let _f1 = 1.0;

    println!("{}", usize::BITS);
    println!("{:p}", &MAX_POINT);

    let i2:i64 = 1;
    let i3:i64 = 2;
    println!("{:p}", &i2);
    println!("{:p}", &i3);


    // 以下の処理はスタック上に残ってしまう
    let y = 5;
    println!("{:p}", &y);
    let y = y + 1;
    println!("{:p}", &y);
    let y = y * 2;
    println!("{:p}", &y);
    println!("{}", &y);

    {
        let y = 0;
        println!("{:p}", &y);
        println!("{}", &y);
    }

    println!("{:p}", &y);
    println!("{}", &y);

    let tuple1 = (1, "hello", true);
    let (_x, _y, _z) = tuple1;
    println!("xは{}, yは{}, zは{}", tuple1.0, tuple1.1, tuple1.2);

    let mut tuple2 = ((1, "hello"), (true, false));
    let ((ref mut x1, ref mut x2), _) = tuple2; // x1は1、x2は"hello"

    //　x1を"5"に変更
    *x1 = 5;

    //　x2を"world"に変更

    *x2 = "world";
    println!("{:?}", tuple2);


    let array1 = [1, 2, 3, 4, 5];
    let array2 = [0;10]; // 10個の要素を0で初期化
    let array3 = array2.map(|x| x + 1);

    println!("{:?} , {:?}", array2, array1[0]);

    println!("{:?}", array3);
}
