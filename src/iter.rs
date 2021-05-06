pub struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    pub fn new(first: T, second: T) -> Pair<T> {
        return Pair { first, second };
    }
}

impl<'a, T> IntoIterator for &'a Pair<T> {
    type Item = &'a T;

    type IntoIter = PairIterator<&'a T>;

    fn into_iter(self) -> Self::IntoIter {
        PairIterator {
            pair: self,
            counter: 0,
        }
    }
}

impl<T: Copy> IntoIterator for Pair<T> {
    type Item = T;

    type IntoIter = PairIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        PairIterator {
            counter: 0,
            pair: self,
        }
    }
}

pub struct PairIterator<T: Copy> {
    counter: u8,
    pair: Pair<T>,
}

impl<'a, T: Copy> Iterator for PairIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match &self.counter {
            0 => Some(self.pair.first),
            1 => Some(self.pair.second),
            _ => None,
        };
        self.counter += 1;
        return result;
    }
}

fn main() {
    println!("{}", Next::<i32>::);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test1() {
        let pair = Pair::new(1, 2);
        let result: Vec<_> = pair.into_iter().collect();
        assert_eq!(vec![1, 2], result);
    }
}
