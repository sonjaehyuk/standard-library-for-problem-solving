#![allow(clippy::doc_lazy_continuation)]
use crate::heap::{Heap, HeapType, levels_from_len};
use std::fmt::Debug;

/// 최대 힙이란 힙 루트가 가장 값이 큰 자료구조이다.
pub struct MinHeap<T: Ord + Clone> {
    item: Vec<T>,
}

impl<T: Ord + Clone + Debug> Heap for MinHeap<T> {
    type Item = T;
    const HEAP_TYPE: HeapType = HeapType::MinHeap;

    fn new() -> Self
    where
        Self: Sized,
    {
        MinHeap { item: Vec::new() }
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
