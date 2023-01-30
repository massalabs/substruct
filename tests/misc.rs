use substruct::SubStruct;

#[allow(missing_docs)]
pub struct Config {
    a: i32,
    b: u32,
    c: f32,
    d: f64,
}

#[allow(missing_docs)]
#[derive(Debug)]
#[derive(SubStruct)]
#[parent(type = "Config")]
pub struct SubConfig {
    c: f32,
    d: f64,
}

fn main() {

    let c_value = 4.2;
    let d_value = 0.4242422110;
    let cfg = Config {
        a: -3,
        b: 2,
        c: c_value,
        d: d_value,
    };

    let sc: SubConfig = (&cfg).into();

    assert_eq!(sc.c, c_value);
    assert_eq!(sc.d, d_value);
    println!("sc: {:?}", sc);
}
