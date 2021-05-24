use std::cmp::Ordering;

use rand::prelude::IteratorRandom;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    InvalidParameters,
    UnknownInput,
}

pub fn random_max<'a, T, Item, F>(iter: T, mut compare: F) -> Option<Item>
where
    T: Iterator<Item = Item>,
    F: FnMut(&Item, &Item) -> Ordering,
{
    let mut rand = rand::thread_rng();
    let mut result: Vec<Item> = Vec::new();

    for item in iter {
        if result.len() == 0 {
            result.push(item);
        } else {
            match compare(&result[0], &item) {
                Ordering::Less => {
                    result.clear();
                    result.push(item)
                }
                Ordering::Equal => result.push(item),
                Ordering::Greater => {}
            }
        }
    }
    return result.into_iter().choose(&mut rand);
}

#[cfg(test)]
mod tests {
    use super::super::common::random_max;

    #[test]
    fn it_works() {
        let test = vec![(0, 10), (1, 20), (4, 20), (3, 4), (6, 20), (100, 20)];
        let x = vec![
            random_max(test.clone().into_iter(), |x, y| x.1.cmp(&y.1)).unwrap(),
            random_max(test.clone().into_iter(), |x, y| x.1.cmp(&y.1)).unwrap(),
        ];
        println!("{:?}", x);
    }
}
