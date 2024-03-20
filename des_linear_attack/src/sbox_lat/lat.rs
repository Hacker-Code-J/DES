use crate::sbox_lat::utils::dot;

fn bias_integer(s: &[u8], alpha: u8, beta: u8, n: u8) -> i8 {
    let range = 1 << n;
    let mut e = 0;
    for x in 0..range {
        if dot(alpha, x) ^ dot(beta, s[x as usize]) == 0 {
            e += 1;
        }
    }
    // e as i8 - (range / 2) as i8
    e as i8
}

pub fn generate_lat(s: &[u8], n: u8, m: u8) -> Vec<Vec<i8>> {
    let n_range = 1 << n;
    let m_range = 1 << m;
    let mut lat = vec![vec![0i8; m_range as usize]; n_range as usize];
    for alpha in 0..n_range {
        for beta in 0..m_range {
            lat[alpha as usize][beta as usize] = bias_integer(s, alpha, beta, n);
        }
    }
    lat
}