mod cell;
mod position;

pub use cell::Cell;
pub use position::Position;

fn n_choose_k(n: usize, k: usize) -> usize {
    if k == 0 || n == k {
        1
    } else {
        let mut dividend = n;
        let mut divisor = 1;

        for i in 2..=k {
            dividend *= n + 1 - i;
            divisor *= i;
        }

        dividend / divisor
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_binominal() {
        let nck = n_choose_k(0, 0);
        assert_eq!(nck, 1);
        let nck = n_choose_k(6, 3);
        assert_eq!(nck, 20);
        let nck = n_choose_k(7, 0);
        assert_eq!(nck, 1);
        let nck = n_choose_k(7, 1);
        assert_eq!(nck, 7);
        let nck = n_choose_k(7, 2);
        assert_eq!(nck, 21);
        let nck = n_choose_k(7, 3);
        assert_eq!(nck, 35);
        let nck = n_choose_k(7, 4);
        assert_eq!(nck, 35);
        let nck = n_choose_k(7, 5);
        assert_eq!(nck, 21);
        let nck = n_choose_k(7, 6);
        assert_eq!(nck, 7);
        let nck = n_choose_k(7, 7);
        assert_eq!(nck, 1);
    }
}
