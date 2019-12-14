use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::path::Path;

fn main() -> std::io::Result<()> {
    // read this ol dingus input file
    let file = File::open("input.txt")?;
    // use the BufReader because it's buff as heck
    let buf_reader = BufReader::new(file);
    let mut _output: HashMap<String, String> = HashMap::new();

    let mut a: HashSet<String> = HashSet::new();
    let mut b: HashSet<String> = HashSet::new();

    let _wire = ".";
    let _overlap = ".";
    let mut line_count = 0;

    // loop this jank
    for line in buf_reader.lines() {
        line_count+=1;
        let l = line.unwrap();

        // split this dang string by comma
        let split = l.split(",");

        // starting point
        let mut _x = 0;
        let mut _y = 0;
        // just a buch of moves
        for s in split {
            let (_dir, _mov) = split_direction_and_movement(s);

            // bout to triple stack these dang for loops
            if _dir=="R" {
                let xz = _x+_mov.parse::<i64>().unwrap();
                for x in _x..xz {
                    let key = format!("{},{}", x, _y);
                    if line_count == 2 {
                        b.insert(key);
                    } else {
                        a.insert(key);
                    }
                }
                _x = xz;
            }
            if _dir=="L" {
                let xz = _x-_mov.parse::<i64>().unwrap();
                for x in xz.._x {
                    let key = format!("{},{}", x, _y);
                    if line_count == 2 {
                        b.insert(key);
                    } else {
                        a.insert(key);
                    }
                }
                _x = xz;
            }
            if _dir=="U" {
                let yz = _y+_mov.parse::<i64>().unwrap();
                for y in _y..yz {
                    let key = format!("{},{}", _x, y);
                    if line_count == 2 {
                        b.insert(key);
                    } else {
                        a.insert(key);
                    }
                }
                _y = yz;
            }
            if _dir=="D" {
                let yz = _y-_mov.parse::<i64>().unwrap();
                for y in yz.._y {
                    let key = format!("{},{}", _x, y);
                    if line_count == 2 {
                        b.insert(key);
                    } else {
                        a.insert(key);
                    }
                }
                _y = yz;
            }
        }
    }
    // now I have vector to play with, yay!
    // _output = move_junk("test".to_string());
    let _intersection = a.intersection(&b).collect::<Vec<&String>>();

    let mut _vec = vec![];
    for key in _intersection {
        // do key math
        println!("key: {}", key);
        let mut sum = 0;
        let split = key.split(",");
        for s in split {
            sum+=s.parse::<i32>().unwrap().abs();
        }
        _vec.push(sum);
    }

    _vec.sort();

    // println!("output: {:?}", _output);
    println!("vec: {:?}", _vec);
    // write_to_file(_output.to_string());
    Ok(())
}

// Splits the stupid R,L,U,D from the actual distance of movement
fn split_direction_and_movement(s: &str) -> (&str, &str) {
    for i in 1..5 {
        let r = s.get(0..i);
        match r {
            Some(x) => return (x, &s[i..]),
            None => (),
        }
    }

    (&s[0..0], s)
}

fn write_to_file(data: String) {
    let path = Path::new("./output.txt");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(data.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}