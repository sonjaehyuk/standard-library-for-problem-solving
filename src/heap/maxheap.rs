#![allow(clippy::doc_lazy_continuation)]
use crate::heap::{Heap, HeapType, levels_from_len};
use std::fmt::Debug;

/// 최대 힙이란 힙 루트가 가장 값이 큰 자료구조이다.
pub struct MaxHeap<T: Ord + Clone + Debug> {
    item: Vec<T>,
}

impl<T: Ord + Clone + Debug> Heap for MaxHeap<T> {
    type Item = T;
    const HEAP_TYPE: HeapType = HeapType::MaxHeap;

    fn new() -> Self
    where
        Self: Sized,
    {
        MaxHeap { item: Vec::new() }
    }

    fn item(&mut self) -> &mut Vec<Self::Item> {
        &mut self.item
    }

    fn from_vec(vec: Vec<Self::Item>) -> Self
    where
        Self: Sized,
    {
        let mut init = Self::new();
        init.item = vec;
        for i in (0..init.item.len()).rev() {
            init.shift_down();
        }
        init
    }
}
