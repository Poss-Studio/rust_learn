use std::io;
mod enums;
mod string_test;
mod file;
//由于Rust是亲函数式编程的所以默认没有保留类Class方案
//但是存在方法集impl
#[derive(Debug)]
struct FemboyRexim
{
    name:&'static str,
    hp:i32,
    sex:&'static str,
    tall:f32,
    address:&'static str
}
impl FemboyRexim
{
    fn new() -> self::FemboyRexim
    {
	FemboyRexim{
	    name:"Rexim",
	    hp:17,
	    sex:"Femboy",
	    tall:1.69,
	    address:"ddt1472582022@outlook.com"
	}
    }
}
fn another_function(v1:&'static str,v2:&'static str) -> (&'static str,&'static str)
{
    println!("{v1}");
    FemboyRexim::new();
    println!("The profile of the Rexim entity is {:#?}",FemboyRexim::new());
    (v1,v2)
}
fn main() -> io::Result<()>
{
    //Println是一个宏用于打印文本在命令行上
    println!("hello,World!");
    //这是一个变量的定义,mut代表这个变量是可变的,Rust高级特性:变量默认是不可变的
    let mut a = 5;
    println!("The value of a is {a}");
    let b = &mut a;
    //指针初级学习(警告一次)
    *b += 8;
    println!("The value of a is {a}");
    //声明这是一个静态的str类型
    let s1:&'static str = "Another Function is called";
    let s2:&'static str = "I am a Femboy Yet";
    another_function(&s1,&s2);
    enums::light();
    string_test::string_use();
    let nulls = file::read_file()?;
    println!("{:?}",nulls);
    Ok(())
}
