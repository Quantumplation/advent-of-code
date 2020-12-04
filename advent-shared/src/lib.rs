use anyhow::*;
use std::path::PathBuf;
pub mod parsers;

pub fn run<P, I, S, R>(year: u32, file: &str, p: P, s: S) -> Result<R>
    where
        P : Fn(PathBuf) -> Result<I>,
        S : Fn(I) -> Result<R> {
    s(p([format!(r"advent-{}", year).as_str(), "input", file].iter().collect())?)
}

#[cfg(test)]
mod tests {
}
