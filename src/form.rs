use std::str::FromStr;

pub fn read_for_input<T>() -> T
where
    T: FromStr,
    T::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse::<T>().unwrap()
}

//let service_type: u32 = read_input();
//let count: usize = read_input();
//let rate: f64 = read_input();

