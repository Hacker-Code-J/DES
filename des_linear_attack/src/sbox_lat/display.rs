pub fn print_lat(s: &[u8], n: u8, m: u8) {
    use crate::sbox_lat::lat::generate_lat;

    let lat = generate_lat(s, n, m);
    let n_range = ((1 << n) >> 1) + 1;
    let m_range = 1 << m;

    print!("     ");
    for beta in 0..m_range {
        print!("{:03X} ", beta);
    }
    println!();

    print!("   +");
    for _ in 0..m_range {
        print!("----");
    }
    println!();

    for alpha in 0..n_range {
        print!("{:02X} | ", alpha);
        for beta in 0..m_range {
            print!("{:3} ", lat[alpha as usize][beta as usize]);
        }
        println!();
    }
}
