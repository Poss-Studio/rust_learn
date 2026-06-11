pub fn string_use(){
    //let s1 = String::from("This is the first method");
    //s1是直接读取
   //let mut s2 = String::new();
   //io::stdin().read_line(&mut s2).unwrap();
   //println!("The str of the s2 is {}",s2.as_str());
    //Rust match + string chose
   //let mut guess = String::new();
   //io::stdin().read_line(&mut guess).unwrap();
   //match guess.trim(){
   //    "Black" => {
   //        println!("Just so cute");
   //    } 
   //    "White" => {
   //        println!("You Are a Lovely femboy");
   //    }
   //    _ => todo!()
   //}
    //match + Option<value>
    //Rust权威指南6-5
    let hx = Some(5);
    fn plus_one(x:Option<i32>) -> Option<i32>{
        match x{
            None => None,
            Some(x) => Some(x+1)
       };
       x
    }
    println!("The value of hx is {:?}",plus_one(hx));
   /*在Rust中,if let 是match匹配的语法糖,
    * match能够做到清晰可读的穷举匹配
    * if let能在只有一个或者几个匹配值的时候方便使用
    * 日常开发中要做好关于便携性和可穷举性的取舍,注意安全
    */ 
    let  count = Some(3);
    if let Some(3) = count {
        println!("Is three!");
    }
}
