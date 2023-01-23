# substruct

substruct is a macro to easily deal with sub structure

# example

```Rust
use substruct::SubStruct;

// A 'config' struct with lot of fields

pub struct Config {
    a: i32,
    b: u32,
    c: f32,
    d: f64,
}

// A 'sub config' of 'config' 

#[derive(Debug)]
#[derive(SubStruct)]
#[parent(type = "Config")]
pub struct SubConfig {
    c: f32,
    d: f64,
}

fn main() {
    let cfg = Config {
        a: -3,
        b: 2,
        c: 4.2,
        d: 0.4242422110,
    };

    // the macro auto. generates: impl From<&Config> for SubConfig
    // so you can use:
    let sc: SubConfig = (&cfg).into();
}
```