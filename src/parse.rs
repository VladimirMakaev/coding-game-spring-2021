use std::{fmt::Debug, io::stdin, usize};
use std::{marker::PhantomData, str::FromStr};

pub struct Next<T> {
    __data: PhantomData<T>,
}

impl<T> Next<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    pub fn read_from(lines: &Vec<&str>, index: &mut usize) -> T {
        let result = <T as FromStr>::from_str(lines[*index]);
        *index += 1;
        result.unwrap()
    }

    fn read_line() -> String {
        let mut buffer = String::new();
        let _ = stdin().read_line(&mut buffer).unwrap();

        let result = buffer.trim().to_owned();
        eprintln!("{}", result);
        result
    }

    pub fn read() -> T {
        <T as FromStr>::from_str(Self::read_line().as_str()).unwrap()
    }

    pub fn read_many() -> Vec<T> {
        let line = Self::read_line();
        Self::read_many_from(line.as_str())
    }

    pub fn read_many_from(s: &str) -> Vec<T> {
        s.split(' ')
            .map(|x| <T as FromStr>::from_str(x).unwrap())
            .collect()
    }
}
