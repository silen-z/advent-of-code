pub enum Part {
    One,
    Two,
}

pub fn part() -> Part {
    match std::env::args().nth(1) {
        Some(s) if s == "--part2" => Part::Two,
        _ => Part::One,
    }
}

pub mod prelude {
    use std::fmt::Display;

    pub trait UserExitErrorMessage<T> {
        fn or_exit(self) -> T;
        fn or_exit_with(self, msg: &str) -> T;
    }

    impl<T, E: Display> UserExitErrorMessage<T> for Result<T, E> {
        fn or_exit(self) -> T {
            match self {
                Result::Ok(v) => v,
                Result::Err(err) => {
                    eprintln!("{}", err);
                    std::process::exit(1);
                }
            }
        }

        fn or_exit_with(self, msg: &str) -> T {
            match self {
                Result::Ok(v) => v,
                Result::Err(err) => {
                    eprintln!("{}: {}", msg, err);
                    std::process::exit(1);
                }
            }
        }
    }

    impl<T> UserExitErrorMessage<T> for Option<T> {
        fn or_exit(self) -> T {
            match self {
                Option::Some(v) => v,
                Option::None => {
                    std::process::exit(0);
                }
            }
        }

        fn or_exit_with(self, msg: &str) -> T {
            match self {
                Option::Some(v) => v,
                Option::None => {
                    eprintln!("{}", msg);
                    std::process::exit(0);
                }
            }
        }
    }
}
