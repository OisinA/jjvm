use std::collections::HashMap;

use crate::jvm_val::JvmVal;
use logging_timer::time;

#[derive(Debug)]
pub struct Heap {
    pub heap: Vec<(JvmVal, bool)>,
}

impl Heap {
    #[time]
    pub fn alloc(self: &mut Heap, val: JvmVal) -> u32 {
        let free = self.free_block();
        if free.is_none() {
            self.heap.push((val, true));
            return self.heap.len() as u32 - 1;
        }

        self.heap[free.unwrap() as usize] = (val, true);
        free.unwrap()
    }

    pub fn fetch(self: &mut Heap, index: u32) -> &JvmVal {
        &self.heap.get(index as usize).unwrap().0
    }

    pub fn fetch_mut(self: &mut Heap, index: u32) -> &mut JvmVal {
        &mut self.heap.get_mut(index as usize).unwrap().0
    }

    fn free_block(self: &Heap) -> Option<u32> {
        for i in 0..self.heap.len() {
            if !self.heap[i].1 {
                return Some(i as u32);
            }
        }

        None
    }

    pub fn allocated_items(self: &Heap) -> usize {
        self.heap.iter().filter(|i| i.1).count()
    }

    #[time]
    pub fn gc(self: &mut Heap, references: HashMap<i32, Vec<u32>>) -> i32 {
        let references: Vec<_> = references.values().flatten().collect();
        let mut all_refs = vec![];
        for refer in references {
            all_refs.push(*refer);
            if let JvmVal::Class(_, vals) = self.heap[*refer as usize].clone().0 {
                for val in vals.values() {
                    if let JvmVal::Reference(_) = val {
                        all_refs.append(&mut self.check_class(val.clone()));
                    }
                }
            }
        }

        let mut claimed = 0;
        for i in 0..self.heap.len() {
            if !self.heap[i].1 {
                continue;
            }
            if !all_refs.contains(&(i as u32)) {
                (&mut self.heap[i]).1 = false;
                claimed += 1;
            }
        }

        claimed
    }

    fn check_class(self: &Heap, val: JvmVal) -> Vec<u32> {
        let mut refs = vec![];
        if let JvmVal::Reference(refer) = val {
            refs.push(refer);
            refs.append(&mut self.check_class(self.heap[refer as usize].clone().0));
        }

        refs
    }
}
