extern crate regex;
use regex::Regex;
use regex::Error;
// use std::net::Shutdown::Read;

// static TEXT: &'static str = "2020-03-12 17:53:13\n2020-04-12 17:53:13\n2020-05-11 17:53:13\n2020-05-12 17:53:13\n";


fn main() -> Result<(), Error>{
    let text = "2020-03-12 17:53:13 xxx 2020-04-12 17:53:13 yyy 2020-05-11 17:53:13\n\r2020-05-12 17:53:13\n";
    let date_match = Regex::new(r"(\d+-\d+-\d+) \d+:\d+:\d+")?;
    // let matches = date_match.find_iter(text);
    // for m in matches{
    //     let string = m.as_str();
    //     println!("match: {}", &string);
    // }
    let matches:Vec<regex::Match> = date_match.find_iter(text).collect();
    for m in &matches{
        let string = m.as_str();
        println!("match: {}", &string);
    }

    for m in &matches{
        let string = &m.as_str(); // add & should be the same
        println!("match: {}", &string);
    }

    println!("---------1--------");

    let captures: Vec<regex::Captures> = date_match.captures_iter(text).collect();
    for c in &captures{
        let string = c.get(0).unwrap().as_str();
        println!("match: {}", string);

        println!("captures: {}", &c[0]);
        println!("captures: {}", &c[1]);
    }

    // for c in &captures{
    //     let string = c.get(0).unwrap().as_str();
    //     println!("match: {}", string);
    // }
    Ok(())
}



