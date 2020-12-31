use pbrust::vector3::Vector3;

fn main() {
    let v1 = Vector3::<i64> {x: 1, y: 2, z: 3};
    let v2 = Vector3::new(3, 4, 5);
    println!("{:?}", v1);
    println!("{:?}", v1 + v2);
    println!("{:?}", v2);
    let mut v3 = v2;
    v3 += v3;
    println!("{:?}", v3);
    println!("{:?}", v3-v1);
    println!("{:?} {:?}", -v1, (-v1).abs())
}
