use anyhow::Result;
use std::str::FromStr;

pub fn read_for_input<T>() -> Result<T>
where
    T: FromStr,
    T::Err: std::error::Error + Send + Sync + 'static,
{
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;
    Ok(buf.trim().parse::<T>()?)
}


//let service_type: u32 = read_input();
//let count: usize = read_input();
//let rate: f64 = read_input();
