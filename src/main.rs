use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use math::round;

fn main() -> std::io::Result<()> {
    // read this ol dingus input file
    let file = File::open("input.txt")?;
    // use the BufReader because it's buff as heck
    let buf_reader = BufReader::new(file);

    let mut sum = 0.0;

    // loop this jank
    for line in buf_reader.lines() {
        let l = line.unwrap();
        let x = l.parse::<f64>().unwrap();
        let y = round::floor(x/3.0, 0)-2.0;

        // println!("round({}/3)-2={}", x, y);
        sum += y;
    }
    println!("sum: {}", sum);
    Ok(())
}