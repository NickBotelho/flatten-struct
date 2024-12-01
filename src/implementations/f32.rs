use crate::Flatten;

impl Flatten<f32> for u8 {
    fn flatten(&self) -> Vec<f32> {
        vec![*self as f32]
    }
}

impl Flatten<f32> for i8 {
    fn flatten(&self) -> Vec<f32> {
        vec![*self as f32]
    }
}

impl Flatten<f32> for u16 {
    fn flatten(&self) -> Vec<f32> {
        vec![*self as f32]
    }
}

impl Flatten<f32> for i16 {
    fn flatten(&self) -> Vec<f32> {
        vec![*self as f32]
    }
}

impl Flatten<f32> for u32 {
    fn flatten(&self) -> Vec<f32> {
        vec![*self as f32]
    }
}

impl Flatten<f32> for i32 {
    fn flatten(&self) -> Vec<f32> {
        vec![*self as f32]
    }
}

impl Flatten<f32> for u64 {
    fn flatten(&self) -> Vec<f32> {
        vec![*self as f32]
    }
}

impl Flatten<f32> for i64 {
    fn flatten(&self) -> Vec<f32> {
        vec![*self as f32]
    }
}

impl Flatten<f32> for f32 {
    fn flatten(&self) -> Vec<f32> {
        vec![*self as f32]
    }
}