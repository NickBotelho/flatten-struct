pub mod implementations;
pub use macros::*;

pub trait Flatten<T> 
where T: Copy {
    fn flatten(&self) -> Vec<T>;
}

#[cfg(test)]
mod tests;