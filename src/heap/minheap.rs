#![allow(clippy::doc_lazy_continuation)]
use crate::heap::{Heap, levels_from_len};
use std::cmp::Ordering;
use std::fmt::Debug;

/// 최대 힙이란 힙 루트가 가장 값이 큰 자료구조이다.
pub struct MinHeap<T: Ord + Clone> {
    item: Vec<T>,
}

impl<T: Ord + Clone + Debug> MinHeap<T> {
    /// heapify 매개변수 root가 전체 heap에서 자신의 자리를 찾아가도록 하는 연산입니다.
    /// 만약 root의 모든 하위자식들이 heapify를 수행해서 root 미만 Heap 노드 전체가 안정되었다면,
    /// root의 heapify 연산 결과는 root 이하 Heap 노드 전체가 안정됨을 보장할 수 있습니다.
    /// ## 과정
    /// 1. 기준 노드의 두 직계 자식 중 값이 더 큰 자식을 고릅니다.
    /// 자식이 없으면 leaf이므로 그만합니다. 만약 한쪽 자식만 있는 경우 그 한쪽 자식을 사용합니다.
    /// 2. 가장 작은 자식과 기준 노드를 비교합니다.
    /// 3. 가장 작은 자식이 부모보다 작은 경우 swap을 수행하고, 기준 노드를 가장 작은 자식으로 하여 새롭게 heapify합니다.
    /// 4. 가장 작은 자식이 부모보다 큰 경우 Heap이 완성되었다는 의미이므로 그만합니다.
    /// > MaxHeap에서는 자식이 부모보다 작은 경우 swap을 수행하고 계속 노드 아래로 내려가며 heapify를 수행해야 합니다.
    /// 그래야 부모가 자식보다 작은 경우 그 아래까지 heapify가 되었음을 보장할 수 있기 때문입니다.
    ///
    /// > heapify를 올바르게 호출하기 위해서는 반드시 root의 모든 하위자식들이 Heap인지 고려해야 합니다.
    fn heapify(&mut self, root: usize) {
        let len = self.len();
        let mut current_index = root;
        loop {
            let left_child_index = 2 * current_index + 1;
            let right_child_index = 2 * current_index + 2;
            if left_child_index >= len {
                // current_index가 leaf인 상황
                break;
            }
            let max_child_index: usize = if right_child_index < len {
                match self.item[right_child_index].cmp(&self.item[left_child_index]) {
                    // Equal은 어디에 가든 상관없음.
                    Ordering::Less | Ordering::Equal => right_child_index,
                    Ordering::Greater => left_child_index,
                }
            } else {
                // 오른쪽 자식은 없는 상황
                left_child_index
            };

            match self.item[max_child_index].cmp(&self.item[current_index]) {
                Ordering::Less | Ordering::Equal => {
                    self.item.swap(current_index, max_child_index);
                    current_index = max_child_index;
                }
                Ordering::Greater => break,
            }
        }
    }
}

impl<T: Ord + Clone + Debug> Heap for MinHeap<T> {
    type Item = T;

    fn new() -> Self
    where
        Self: Sized,
    {
        MinHeap { item: Vec::new() }
    }

    /// MinHeap에 새 원소를 추가합니다.
    /// ## 과정
    /// 1. 너무 작은 Heap은 그냥 바로 넣습니다.
    /// 2. 함부로 노드를 추가할 수 없는 경우, 일단 새 값을 맨 뒤에 넣습니다.
    /// 3. 뒷 인덱스부터 반복문으로 순회하며 아래쪽 노드부터 [`Maxheap::heapify`]를 수행합니다.
    fn push(&mut self, item: Self::Item) {
        let len: usize = self.len();
        if len <= 1 {
            self.item.push(item);
            return;
        }
        self.item.push(item);

        for i in (0..len).rev() {
            self.heapify(i);
        }
    }

    /// MinHeap의 root를 제거합니다.
    /// MinHeap의 root는 Heap 구성 원소 중 가장 작은 값입니다.
    /// ## 원리
    /// root를 마지막과 교환하고 마지막을 삭제하면, 새 root 위치만 [`Maxheap::heapify`]를 수행하면 다시 heap 복구가 됩니다.
    /// ## 과정
    /// 1. heap이 작은 크기인 경우 바로 반환합니다. 때로는 heap이 비어있어 [`None`]을 반환할 수도 있습니다.
    /// 2. root를 마지막에 위치한 leaf와 교환합니다.
    /// 3. 마지막 leaf가 된 원래 root를 삭제합니다.
    /// 4. 새 root가 된 원래 leaf가 제자리를 찾아가도록 [`MaxHeap::heapify`]를 호출합니다.
    fn pop(&mut self) -> Option<Self::Item> {
        match self.item.len() {
            0 => None,
            1 => self.item.pop(),
            2.. => {
                let root = &self.item[0].clone();
                let len = self.len();
                self.item.swap(0, len - 1);
                self.item.pop();
                self.heapify(0);
                Some(root.clone())
            }
        }
    }

    fn peek(&self) -> Option<Self::Item> {
        self.item.first().cloned()
    }

    fn len(&self) -> usize {
        self.item.len()
    }

    fn clear(&mut self) {
        self.item.clear();
    }

    fn from_vec(vec: Vec<Self::Item>) -> Self
    where
        Self: Sized,
    {
        let mut init = Self::new();
        init.item = vec;
        for i in (0..init.item.len()).rev() {
            init.heapify(i);
        }
        init
    }

    fn tree_view(&self) {
        let mut result = String::new();
        let len = self.len();
        if len == 0 {
            return;
        }

        let level = levels_from_len(len);
        for i in 0..level {
            let start = (1usize << i) - 1;
            let end = ((1usize << (i + 1)) - 2).min(len.saturating_sub(1));
            result += "L{i}: ";
            for i in start..=end {
                result += format!("{:?} ", self.item[i]).as_str();
            }
            result += "\n";
        }
        println!("{result}")
    }
}
