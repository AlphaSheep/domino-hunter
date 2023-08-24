use lazy_static::lazy_static;

// Computing binomial coefficients is a little slow, so we precompute them
// and just return the value from the table.
lazy_static! {
    static ref BINOMIAL_TABLE: [[usize; 13]; 13] = precompute_binomial_table();
    static ref FACTORIAL_TABLE: [usize; 13] = precompute_factorial_table();
}

fn precompute_binomial_table() -> [[usize; 13]; 13] {
    let mut binomial_table = [[0; 13]; 13];
    for n in 0..=12 {
        binomial_table[n][0] = 1;
        for k in 1..=n {
            binomial_table[n][k] = binomial_table[n-1][k-1] + binomial_table[n-1][k];
        }
    }
    binomial_table
}

fn precompute_factorial_table() -> [usize; 13] {
    let mut factorial_table = [0; 13];
    factorial_table[0] = 1;
    for n in 1..=12 {
        factorial_table[n] = factorial_table[n-1] * n;
    }
    factorial_table
}

pub fn binomial(n: usize, k: usize) -> usize {
    BINOMIAL_TABLE[n][k]
}

pub fn factorial(n: usize) -> usize {
    FACTORIAL_TABLE[n]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_precompute_binomial_table() {
        let binomial_table = precompute_binomial_table();
        assert_eq!(binomial_table[0][0], 1);
        assert_eq!(binomial_table[1][0], 1);
        assert_eq!(binomial_table[1][1], 1);
        assert_eq!(binomial_table[2][0], 1);
        assert_eq!(binomial_table[2][1], 2);
        assert_eq!(binomial_table[2][2], 1);
        assert_eq!(binomial_table[3][0], 1);
        assert_eq!(binomial_table[3][1], 3);
        assert_eq!(binomial_table[3][2], 3);
        assert_eq!(binomial_table[3][3], 1);
        assert_eq!(binomial_table[4][0], 1);
        assert_eq!(binomial_table[4][1], 4);
        assert_eq!(binomial_table[4][2], 6);
        assert_eq!(binomial_table[4][3], 4);
        assert_eq!(binomial_table[4][4], 1);
        assert_eq!(binomial_table[11][7], 330);
        assert_eq!(binomial_table[12][4], 495);
    }

    #[test]
    fn test_precompute_factorial_table() {
        let factorial_table = precompute_factorial_table();
        assert_eq!(factorial_table[0], 1);
        assert_eq!(factorial_table[1], 1);
        assert_eq!(factorial_table[2], 2);
        assert_eq!(factorial_table[3], 6);
        assert_eq!(factorial_table[4], 24);
        assert_eq!(factorial_table[5], 120);
        assert_eq!(factorial_table[6], 720);
        assert_eq!(factorial_table[7], 5_040);
        assert_eq!(factorial_table[8], 40_320);
        assert_eq!(factorial_table[9], 362_880);
        assert_eq!(factorial_table[10], 3_628_800);
        assert_eq!(factorial_table[11], 39_916_800);
        assert_eq!(factorial_table[12], 479_001_600);
    }
}