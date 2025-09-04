struct Struct {
    e: i32,
}
fn main() {
    // 可变变量
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    // 未使用忽略
    // let _x = 5;
    // let y = 10;

    // 变量解构
    // a不可变，b可变
    // let (a, mut b): (bool, bool) = (true, true);
    // println!("a={:?},b={:?}", a, b);
    // b = true;
    // assert_eq!(a, b);

    // 解构式赋值
    // let (a, b, c, d, e);
    // (a, b) = (1, 2);
    // // 左边表示 从第一个开始赋给c 到最后一个的倒数第二个赋给d
    // [c, .., d, _] = [1, 2, 3, 4, 5, 6, 7];
    // Struct { e, .. } = Struct { e: 5 };
    // assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);

    // 常量
    // const MAX_POINTS: u32 = 100_000;
    // println!("MAX_POINTS:{:?}", MAX_POINTS);

    // 变量遮蔽 允许声明相同变量名称
    // let x = 5;
    // let x = x + 1;
    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {}", x);
    // }
    // println!("The value of x is: {}", x);

    // mut可以修改同一内存地址的值，不会发生内存对象再分配
    // 而变量遮蔽 涉及一次内存对象的再分配
    // 例子1
    // let spaces = "   ";
    // let spaces = spaces.len();
    // 例子2
    // let mut spaces = "   ";
    // spaces = spaces.len();
}
