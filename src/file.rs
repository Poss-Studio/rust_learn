use std::io::{self,BufRead,BufReader};
use std::fs::File;
pub fn read_file() -> io::Result<()>{
    let file = File::open("/home/poss/record")?;
    let reader = BufReader::new(file);
    for line in reader.lines()
    {
        let line = line?;
        println!("{}",line);
    }
    Ok(())
}
