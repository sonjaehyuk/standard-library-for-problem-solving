use std::cmp::*;

/// 공통 힙 인터페이스
/// 어떤 부모의 자식은 2 * 부모 인덱스 + 1과 2 * 부모 인덱스 + 2이다.
/// TODO 힙 이론과 구현 방안
pub trait Heap {
    type Item: Ord+Clone;

    /// 비어있는 힙 생성
    fn new() -> Self where Self: Sized;

    /// 원소 추가
    fn push(&mut self, item: Self::Item);

    /// 최상단(루트) 원소 제거+반환
    fn pop(&mut self) -> Option<Self::Item>;

    /// 최상단(루트)만 반환. Clone을 사용하므로 메서드 호출한 곳에서 소유권 사용해도 됨.
    fn peek(&self) -> Option<Self::Item>;

    /// 원소 개수
    fn len(&self) -> usize;

    /// 비었는지
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 모두 제거
    fn clear(&mut self);
}

/// 최대 힙이란 힙 루트가 가장 값이 큰 자료구조이다.
pub struct MaxHeap<T: Ord+Clone> {
    item: Vec<T>
}

impl<T: Ord+Clone> MaxHeap<T> {
    /// heapify 매개변수 root가 전체 heap에서 자신의 자리를 찾아가도록 하는 연산입니다.
    /// 만약 root의 모든 하위자식들이 heapify를 수행해서 root 미만 Heap 노드 전체가 안정되었다면,
    /// root의 heapify 연산 결과는 root 이하 Heap 노드 전체가 안정됨을 보장할 수 있습니다.
    /// ## 과정
    /// 1. 기준 노드의 두 직계 자식 중 값이 더 큰 자식을 고릅니다.
    /// 자식이 없으면 leaf이므로 그만합니다. 만약 한쪽 자식만 있는 경우 그 한쪽 자식을 사용합니다.
    /// 2. 가장 큰 자식과 기준 노드를 비교합니다.
    /// 3. 가장 큰 자식이 부모보다 큰 경우 swap을 수행하고, 기준 노드를 가장 큰 자식으로 하여 새롭게 heapify합니다.
    /// 4. 가장 큰 자식이 부모보다 작은 경우 Heap이 완성되었다는 의미이므로 그만합니다.
    /// > MaxHeap에서는 자식이 부모보다 큰 경우 swap을 수행하고 계속 노드 아래로 내려가며 heapify를 수행해야 합니다.
    /// 그래야 부모가 자식보다 큰 경우 그 아래까지 heapify가 되었음을 보장할 수 있기 때문입니다.
    ///
    /// > heapify를 올바르게 호출하기 위해서는 반드시 root의 모든 하위자식들이 Heap인지 고려해야 합니다.
    fn heapify(&mut self, root: usize) {
        let len = self.len();
        let mut current_index = root;
        loop { // heapify
            // 1. 자식을 고르고 둘 중 가장 큰 자식을 고릅니다. 자식이 없으면 그만합니다.
            let left_child_index = 2 * current_index + 1;
            let right_child_index = 2 * current_index + 2;
            if left_child_index >= len { // current_index가 leaf인 상황
                break;
            }
            let max_child_index: usize = (|l: usize, r: usize| {
                if r < len {
                    match self.item[r].cmp(&self.item[l]) {
                        // Equal은 어디에 가든 상관없음.
                        Ordering::Less | Ordering::Equal => l,
                        Ordering::Greater => r,
                    }
                } else { // 오른쪽 자식은 없는 상황
                    l
                }
            })(left_child_index, right_child_index);

            // 2. 가장 큰 자식과 현재 노드를 비교합니다.
            // MaxHeap에서는 자식이 부모보다 큰 경우 swap을 수행하고 계속 노드 아래로 내려가며 heapify를 수행해야 합니다.
            // 그래야 부모가 자식보다 큰 경우 그 아래까지 heapify가 되었음을 보장할 수 있기 때문입니다.
            match self.item[max_child_index].cmp(&self.item[current_index]) {
                Ordering::Less | Ordering::Equal => break,
                Ordering::Greater => {
                    self.item.swap(current_index, max_child_index);
                    current_index = max_child_index;
                },
            }
        }
    }
}

impl<T: Ord+Clone> Heap for MaxHeap<T> {
    type Item = T;

    fn new() -> Self
    where
        Self: Sized
    {
        MaxHeap {
            item: Vec::new()
        }
    }

    /// MaxHeap에 새 원소를 추가합니다.
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

    /// MaxHeap의 root를 제거합니다.
    /// MaxHeap의 root는 Heap 구성 원소 중 가장 큰 값입니다.
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
                self.item.swap(0, len-1);
                self.item.pop();
                self.heapify(0);
                Some(root.clone())
            }
        }
    }

    fn peek(&self) -> Option<Self::Item> {
        match self.item.first() {
            None => None,
            Some(root) => Some(root.clone())
        }
    }

    fn len(&self) -> usize {
        self.item.len()
    }

    fn clear(&mut self) {
        self.item.clear();
    }
}

#[cfg(test)]
mod tests {
    use crate::heap::{Heap, MaxHeap};
    use core::cmp::Ordering;

    // 이 테스트들이 컴파일되려면 crate 루트에 다음 심볼이 있어야 함:
    // pub use crate::heap::{Heap, MaxHeap, MinHeap};
    use crate::{MinHeap};

    /// 공통 시나리오를 실행하는 제네릭 테스트 러너
    fn run_basic_suite<H>(is_min_heap: bool)
    where
        H: Heap<Item = i32> + Debug,
    // new()를 호출해 비어있는 힙을 만들 수 있어야 함
    // (네 구현에서 new()가 없다면 Heap::new()는 필수)
    {
        // 빈 상태
        let mut h = H::new();
        assert!(h.is_empty());
        assert_eq!(h.len(), 0);
        assert_eq!(h.peek(), None);
        assert_eq!(h.pop(), None);

        // 몇 개 넣어보기(중복 포함)
        for &x in &[3, 1, 4, 1, 5, 9, 2, 6, 5] {
            h.push(x);
        }
        assert!(!h.is_empty());
        assert_eq!(h.len(), 9);

        // peek 확인: min-heap이면 최소값(=1), max-heap이면 최대값(=9)
        let top = *h.peek().expect("peek must exist");
        if is_min_heap { assert_eq!(top, 1); } else { assert_eq!(top, 9); }

        // pop 순서가 올바른지 확인
        let mut popped = Vec::new();
        while let Some(x) = h.pop() {
            popped.push(x);
        }
        assert!(h.is_empty());
        assert_eq!(h.len(), 0);

        // 정답 시퀀스 생성
        let mut sorted = vec![3,1,4,1,5,9,2,6,5];
        sorted.sort();
        let expected = if is_min_heap {
            sorted.clone()                   // 오름차순
        } else {
            let mut r = sorted.clone();
            r.reverse();                     // 내림차순
            r
        };
        assert_eq!(popped, expected);

        // clear 동작 확인
        let mut h2 = H::new();
        for &x in &[10, 20, 20, -1] { h2.push(x); }
        h2.clear();
        assert!(h2.is_empty());
        assert_eq!(h2.peek(), None);
        assert_eq!(h2.pop(), None);
    }

    /// 문자열로도 동작하는지(Ord만 있다면) 간단 확인
    fn run_string_check<H>()
    where
        H: Heap<Item = &'static str> + Debug,
    {
        let mut h = H::new();
        h.push("delta");
        h.push("alpha");
        h.push("charlie");
        h.push("bravo");

        // 전부 pop해서 수집
        let mut out = Vec::new();
        while let Some(s) = h.pop() { out.push(s); }

        // 이 함수는 "H가 MinHeap인지 MaxHeap인지"를 모르므로
        // 그냥 정렬해 두 케이스 중 하나와 일치하는지만 본다.
        let mut asc = vec!["alpha","bravo","charlie","delta"];
        let mut desc = asc.clone(); desc.reverse();
        assert!(out == asc || out == desc);
    }

    // -------- 여기서 실제로 두 힙 타입을 테스트에 바인딩 --------
    #[test]
    fn max_heap_basic() {
        run_basic_suite::<MaxHeap<i32>>(/* is_min_heap = */ false);
        run_string_check::<MaxHeap<&'static str>>();
    }

    #[test]
    fn min_heap_basic() {
        run_basic_suite::<MinHeap<i32>>(/* is_min_heap = */ true);
        run_string_check::<MinHeap<&'static str>>();
    }

    // 경계 조건/특수 케이스
    #[test]
    fn single_element_and_duplicates() {
        // MaxHeap
        {
            let mut h: MaxHeap<i32> = MaxHeap::new();
            h.push(42);
            assert_eq!(h.peek(), Some(&42));
            assert_eq!(h.pop(), Some(42));
            assert_eq!(h.pop(), None);

            for _ in 0..5 { h.push(7); }
            for _ in 0..5 { assert_eq!(h.pop(), Some(7)); }
            assert!(h.is_empty());
        }
        // MinHeap
        {
            let mut h: MinHeap<i32> = MinHeap::new();
            h.push(42);
            assert_eq!(h.peek(), Some(&42));
            assert_eq!(h.pop(), Some(42));
            assert_eq!(h.pop(), None);

            for _ in 0..5 { h.push(7); }
            for _ in 0..5 { assert_eq!(h.pop(), Some(7)); }
            assert!(h.is_empty());
        }
    }

    // peek이 pop에 영향을 주지 않는지
    #[test]
    fn peek_is_non_destructive() {
        let mut maxh: MaxHeap<i32> = MaxHeap::new();
        let mut minh: MinHeap<i32> = MinHeap::new();
        for &x in &[2,8,3,8] { maxh.push(x); minh.push(x); }

        assert_eq!(maxh.peek(), Some(&8));
        assert_eq!(maxh.len(), 4);
        assert_eq!(maxh.pop(), Some(8));
        assert_eq!(maxh.pop(), Some(8));

        assert_eq!(minh.peek(), Some(&2));
        assert_eq!(minh.len(), 4);
        assert_eq!(minh.pop(), Some(2));
        assert_eq!(minh.pop(), Some(3));
    }

    // 다양한 입력 크기에 대해 힙 성질을 간단히 체크
    #[test]
    fn many_elements_shape_check() {
        let mut maxh: MaxHeap<i32> = MaxHeap::with_capacity(1024);
        let mut minh: MinHeap<i32> = MinHeap::with_capacity(1024);

        for i in 0..1000 {
            let v = (i * 37) % 997; // 다양한 값
            maxh.push(v as i32);
            minh.push(v as i32);
        }
        assert_eq!(maxh.len(), 1000);
        assert_eq!(minh.len(), 1000);

        // Max는 내림차순, Min은 오름차순으로 모두 꺼내졌는지
        let mut max_out = Vec::new();
        let mut min_out = Vec::new();
        while let Some(x) = maxh.pop() { max_out.push(x); }
        while let Some(x) = minh.pop() { min_out.push(x); }

        let mut sorted: Vec<i32> = (0..1000).map(|i| ((i * 37) % 997) as i32).collect();
        sorted.sort();

        let mut sorted_desc = sorted.clone();
        sorted_desc.reverse();

        assert_eq!(max_out, sorted_desc);
        assert_eq!(min_out, sorted);
    }
}
