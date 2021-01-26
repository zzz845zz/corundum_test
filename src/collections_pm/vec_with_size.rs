use corundum::alloc::*;
use corundum::*;
use corundum::default::*;

use crate::utils;
type P = crate::collections_pm::P;

// #[derive(Root)]
pub struct vec_with_size {
    size: Parc<PMutex<usize>>,
    elements: Parc<PMutex<PVec<PString>>>,
}

// impl<V:PSafe> RootObj<P> for vec_with_size<V> {
impl RootObj<P> for vec_with_size {
    fn init(j: &Journal) -> Self {
        vec_with_size {
            size: Parc::new(PMutex::new(0, j), j),
            elements: Parc::new(PMutex::new(PVec::new(j), j), j),
        }
    }
}

impl vec_with_size {
    pub fn put(&self, value: &str) {
        P::transaction(|j| {
            self.elements.lock(j).push(PString::from_str(value, j), j);
            utils::env_crash("crash_put");
            *self.size.lock(j) += 1;
        }).unwrap();
    }

    pub fn get(&self, index: usize) -> Option<String> {
        P::transaction(|j| {
            let size = self.size.lock(j);
            if index >= *size {
                return None;
            }
            Some(self.elements.lock(j)[index].to_string())
        }).unwrap()

    }

    pub fn delete(&self, index: usize) -> Option<String> {
        P::transaction(|j| {
            let mut size = self.size.lock(j);
            if index >= *size {
                return None;
            }
            *size -= 1;
            utils::env_crash("crash_del");
            Some(self.elements.lock(j).remove(index).to_string())
        }).unwrap()
    }

    pub fn print_state(&self) {
        let (size, elements_string, state) = P::transaction(|j| {
            let size = *self.size.lock(j);
            let elements = self.elements.lock(j);
            let elements_string = elements.iter()
                .map(|pstr| pstr.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            let state = size == elements.len();
            (size, elements_string, state)
        }).unwrap();

        println!("-----------------------");
        println!(
            "size: {}\nelements: {}\nstate: {}",
            size, elements_string, if state {"consistent"} else {"inconsistent"}
        );
        println!("-----------------------");
    }
}