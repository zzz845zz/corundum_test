use corundum::alloc::*;
use corundum::*;


use std::vec;
use std::env;
use std::fmt::{Display, Error, Formatter};


type P = corundum::default::BuddyAlloc;
//type P = Heap;
type Pbox<T> = corundum::boxed::Pbox<T, P>;
// type Ptr = Option<Pbox<BTreeNode>>;

pub struct vec_with_size {
    size: usize,
    elements: Vec<String>,
}

impl vec_with_size {
    pub fn new() -> Self {
        vec_with_size {size:0, elements: Vec::new()}
    }

    pub fn put(&mut self, value: String) {
        self.size += 1;
        self.elements.push(value);
    }


    pub fn get(&self, index: usize) -> Option<String> {
        if index >= self.size {
            return None;
        }
        Some(self.elements[index].clone())
    }

    pub fn delete(&mut self, index: usize) -> Option<String> {
        if index >= self.size {
            return None;
        }
        self.size -= 1;
        Some(self.elements.remove(index))
    }

    pub fn print_state(&self) {
        // format!(
        //     "{} {}",
        //     self.specifiers
        //         .iter()
        //         .map(WriteString::write_string)
        //         .collect::<Vec<_>>()
        //         .join(" "),
        //     self.declarators
        //         .iter()
        //         .map(WriteString::write_string)
        //         .collect::<Vec<_>>()
        //         .join(", ")
        // )

        let str_elements = self.elements.iter()
            .map(|v| v.clone())
            .collect::<Vec<_>>()
            .join(", ");

        println!(
            "size: {}\nelements: {}\nstate:{}",
            self.size,
            str_elements,
            self.size==self.elements.len()
        );
    }
}


#[test]
fn test() {

    let mut vec_with_size = vec_with_size::new();

    vec_with_size.put("ha".to_string());
    vec_with_size.put("ho".to_string());

    vec_with_size.print_state();
}

