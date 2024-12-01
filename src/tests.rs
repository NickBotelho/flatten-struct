use std::vec;

use macros::FlattenF32;

use crate::flatten::Flatten;

#[derive(Debug)]
struct Model {
    pub a: i32,
    pub b: u32,
}

impl Flatten<f32> for Model {
    fn flatten(&self) -> Vec<f32> {
        Vec::from([
            self.a.flatten(),
            self.b.flatten()
        ]).concat()
    }
}

#[test]
fn test_implementation() {
    let model = Model {
        a: 1,
        b: 2
    };

    assert_eq!(model.flatten(), vec![1., 2.])
}


#[derive(Debug, FlattenF32)]
struct DeriveStruct {
    pub a: i32,
    pub b: u32,
}

#[test]
fn test_macro() {
    let model = DeriveStruct {
        a: 1,
        b: 2
    };

    assert_eq!(model.flatten(), vec![1., 2.])
}