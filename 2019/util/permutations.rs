pub struct Permutations<'s, T> {
    src: &'s [T],

    done: bool,
    iters: Vec<usize>,
}

impl<'s, T> Permutations<'s, T> {
    fn new(src: &'s [T]) -> Self {
        let n = src.len();

        let mut iters = Vec::new();
        for i in 0..n {
            iters.push(i);
        }

        Permutations {
            src,
            iters,
            done: false,
        }
    }

    fn current_from_iters(&mut self) -> Vec<&'s T> {
        let n = self.src.len();

        let mut current = vec![&self.src[0]; n];

        for (idx, i) in self.iters.iter().zip(0..) {
            current[*idx] = &self.src[i];
        }
        current
    }

    fn increment_withdups(&mut self) {
        let n = self.src.len();
        let mut carry = 1;

        //println!("incrementing {:?}", self.iters);

        for idx in (0..n).rev() {
            //println!("  [{}] {} + {} --> {}", idx, self.iters[idx], carry, self.iters[idx] + carry);

            self.iters[idx] += carry;
            if self.iters[idx] == n {
                self.iters[idx] = 0;
                carry = 1;
            } else {
                carry = 0;
            }

            if carry == 0 {
                break;
            }
        }

        //println!("  done: {:?}, carry = {}", self.iters, carry);

        self.done = carry != 0;
    }

    fn increment(&mut self) {
        let n = self.iters.len();

        loop {
            self.increment_withdups();
            if self.done {
                return;
            }

            let have_dup = {
                let mut dup = false;
                'outer: for i in 0..n {
                    for j in i + 1..n {
                        if self.iters[i] == self.iters[j] {
                            dup = true;
                            break 'outer;
                        }
                    }
                }
                dup
            };

            if !have_dup {
                break;
            }
        }
    }
}

impl<'s, T> Iterator for Permutations<'s, T> {
    type Item = Vec<&'s T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let current = self.current_from_iters();

        self.increment();

        Some(current)
    }
}

pub fn permutations<T>(src: &[T]) -> Permutations<T> {
    Permutations::new(src)
}

fn main() {
    let ints = [1, 2, 3];
    let iter = permutations(&ints);

    for permut in iter {
        println!("{:?}", permut);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn factorial(i: usize) -> usize {
        if i == 1 {
            return 1;
        }

        i * factorial(i - 1)
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(4), 24);
    }

    #[test]
    fn permutations_of_3_ints() {
        let ints = [1, 2, 3];
        let mut iter = permutations(&ints);

        assert_eq!(iter.next(), Some(vec![&1, &2, &3]));
        assert_eq!(iter.next(), Some(vec![&1, &3, &2]));
        assert_eq!(iter.next(), Some(vec![&2, &1, &3]));
        assert_eq!(iter.next(), Some(vec![&3, &1, &2]));
        assert_eq!(iter.next(), Some(vec![&2, &3, &1]));
        assert_eq!(iter.next(), Some(vec![&3, &2, &1]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn permutations_of_4_strings() {
        let strs = ["a", "b", "c", "d"];
        let mut iter = permutations(&strs);

        assert_eq!(iter.next(), Some(vec![&"a", &"b", &"c", &"d"]));
        assert_eq!(iter.next(), Some(vec![&"a", &"b", &"d", &"c"]));
        assert_eq!(iter.next(), Some(vec![&"a", &"c", &"b", &"d"]));
        assert_eq!(iter.next(), Some(vec![&"a", &"d", &"b", &"c"]));
        assert_eq!(iter.next(), Some(vec![&"a", &"c", &"d", &"b"]));
        assert_eq!(iter.next(), Some(vec![&"a", &"d", &"c", &"b"]));
        assert_eq!(iter.next(), Some(vec![&"b", &"a", &"c", &"d"]));
        assert_eq!(iter.next(), Some(vec![&"b", &"a", &"d", &"c"]));
        assert_eq!(iter.next(), Some(vec![&"c", &"a", &"b", &"d"]));
        assert_eq!(iter.next(), Some(vec![&"d", &"a", &"b", &"c"]));
        assert_eq!(iter.next(), Some(vec![&"c", &"a", &"d", &"b"]));
        assert_eq!(iter.next(), Some(vec![&"d", &"a", &"c", &"b"]));
        assert_eq!(iter.next(), Some(vec![&"b", &"c", &"a", &"d"]));
        assert_eq!(iter.next(), Some(vec![&"b", &"d", &"a", &"c"]));
        assert_eq!(iter.next(), Some(vec![&"c", &"b", &"a", &"d"]));
        assert_eq!(iter.next(), Some(vec![&"d", &"b", &"a", &"c"]));
        assert_eq!(iter.next(), Some(vec![&"c", &"d", &"a", &"b"]));
        assert_eq!(iter.next(), Some(vec![&"d", &"c", &"a", &"b"]));
        assert_eq!(iter.next(), Some(vec![&"b", &"c", &"d", &"a"]));
        assert_eq!(iter.next(), Some(vec![&"b", &"d", &"c", &"a"]));
        assert_eq!(iter.next(), Some(vec![&"c", &"b", &"d", &"a"]));
        assert_eq!(iter.next(), Some(vec![&"d", &"b", &"c", &"a"]));
        assert_eq!(iter.next(), Some(vec![&"c", &"d", &"b", &"a"]));
        assert_eq!(iter.next(), Some(vec![&"d", &"c", &"b", &"a"]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn permutation_counts() {
        fn check_count(ints: &[usize]) {
            let iter = permutations(ints);
            println!("permutations of {:?} should have length {}", ints, factorial(ints.len()));
            assert_eq!(iter.count(), factorial(ints.len()));
        }

        check_count(&[1, 2, 3]);
        check_count(&[1, 2, 3, 4]);
        check_count(&[1, 2, 3, 4, 5]);
    }
}
