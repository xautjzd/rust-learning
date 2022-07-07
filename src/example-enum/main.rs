use std::str::FromStr;

#[derive(PartialEq, Debug)]
enum LogLevel {
    TRACE,
    DEBUG,
    INFO,
    ERROR,
    FATAL,
}

impl FromStr for LogLevel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "TRACE" => Ok(Self::TRACE),
            "DEBUG" => Ok(Self::DEBUG),
            "INFO" => Ok(Self::INFO),
            "ERROR" => Ok(Self::ERROR),
            "FATAL" => Ok(Self::FATAL),
            _ => Err(()),
        }
    }
}

fn main() {
    let level = LogLevel::from_str("Debug").unwrap();
    assert_eq!(level, LogLevel::DEBUG);

    match LogLevel::from_str("Debug@") {
        Ok(x) => println!("result is: {:?}", x),
        Err(_) => panic!("unknown loglevel enum"),
    }
}
