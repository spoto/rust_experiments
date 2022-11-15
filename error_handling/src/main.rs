use std::io;
use std::io::BufRead;

fn main() {
    println!("Hello, world!");
    use_it();
    pirate_share(1000, 0);
}

fn pirate_share(total: u64, crew_size: usize) -> u64 {
    let half = total / 2;
    half / crew_size as u64
}

fn get_weather() -> Result<String, io::Error> {
    Ok("ciao".to_string())
}

fn use_it() {
    let result = get_weather();

    match result {
        Ok(report) => {
            println!("{}", report);
        }
        Err(err) => {
            println!("How could this ever happen?");
        }
    }

    //let copy = result; // use of moved value: doesn't compile
}

fn get_weather2() -> Result<i32, io::Error> {
    Ok(42)
}

fn use_it2() {
    let result = get_weather2();

    match result {
        Ok(report) => {
            println!("{}", report);
        }
        _ => {
            println!("How could this ever happen?");
        }
    }

    //let copy = result; // this compiles, but makes the following not compile
    println!("{}", result.is_err())
}

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;

fn read_numbers(file: &mut dyn BufRead) -> GenericResult<Vec<i64>> {
    let mut numbers = vec![];
    for line_result in file.lines() {
        let line = line_result?;
        numbers.push(line.parse()?)
    }
    if numbers.len() != 42 {
        Ok(numbers)
    }
    else {
        let io_error = io::Error::new(io::ErrorKind::Other, "timed out");
        Err(GenericError::from(io_error))
    }
}