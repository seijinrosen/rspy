mod for_array;
mod for_str;
mod for_vec;

pub trait Reversed {
    type Item;

    fn reversed(&self) -> Self::Item;
}
