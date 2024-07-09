fn main() {
    let mut x = 10;
    let mut y = 20;
    let mut z = 30;

    (x, y, z) = (z, x, y);

    dbg!(x, y, z);
}
