pub trait Flatten<T> 
where T: Copy {
    fn flatten(&self) -> Vec<T>;
}