use crate::flatten::Flatten;

impl Flatten<f64> for u8 {
    fn flatten(&self) -> Vec<f64> {
        vec![*self as f64]
    }
}

impl Flatten<f64> for i8 {
    fn flatten(&self) -> Vec<f64> {
        vec![*self as f64]
    }
}

impl Flatten<f64> for u16 {
    fn flatten(&self) -> Vec<f64> {
        vec![*self as f64]
    }
}

impl Flatten<f64> for i16 {
    fn flatten(&self) -> Vec<f64> {
        vec![*self as f64]
    }
}

impl Flatten<f64> for u32 {
    fn flatten(&self) -> Vec<f64> {
        vec![*self as f64]
    }
}

impl Flatten<f64> for i32 {
    fn flatten(&self) -> Vec<f64> {
        vec![*self as f64]
    }
}

impl Flatten<f64> for u64 {
    fn flatten(&self) -> Vec<f64> {
        vec![*self as f64]
    }
}

impl Flatten<f64> for i64 {
    fn flatten(&self) -> Vec<f64> {
        vec![*self as f64]
    }
}