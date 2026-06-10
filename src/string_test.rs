use std::io;
pub fn string_use(){
    //let s1 = String::from("This is the first method");
    //s1是直接读取
   //let mut s2 = String::new();
   //io::stdin().read_line(&mut s2).unwrap();
   //println!("The str of the s2 is {}",s2.as_str());
    //Rust match + string chose
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).unwrap();
    match guess.trim(){
        "Black" => {
            println!("Just so cute");
        } 
        "White" => {
            println!("You Are a Lovely femboy");
        }
        _ => todo!()
    }
}