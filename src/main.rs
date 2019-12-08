use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    // read this ol dingus input file
    let file = File::open("input.txt")?;
    // use the BufReader because it's buff as heck
    let buf_reader = BufReader::new(file);

    let mut _output = vec![];
    let mut arr = vec![];

    // loop this jank
    for line in buf_reader.lines() {
        let l = line.unwrap();

        // split this dang string by comma
        let split = l.split(",");
        // shove the whole thing into a vector
        for s in split {
            // numbers... who needs em?
            let num = s.parse::<i64>().unwrap();
            arr.push(num);
        }
    }
    // now I have vector to play with, yay!
    _output = chug_numbers(arr);
    println!("output: {:?}", _output);
    Ok(())
}

fn chug_numbers(numbers: Vec<i64>) -> Vec<i64> {
    println!("Initial vector: {:?}", numbers);
    let mut arr = numbers.to_vec();
    for n in 0..arr.len() {
        if n%4==0 {
            let op = numbers[n];
            if op == 99 {
                return arr;
            } else {
                // this is gross and I don't care
                let x_index = numbers[n+1] as usize;
                let x = arr[x_index];
                let y_index = numbers[n+2] as usize;
                let y = arr[y_index];
                let storage = numbers[n+3] as usize;

                if op==1 {
                    arr[storage] = x+y;
                }
                if op==2 {
                    arr[storage] = x*y;
                }
            }
        }
    }
    return arr;
}