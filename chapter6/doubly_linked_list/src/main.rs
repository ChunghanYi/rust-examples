use std::rc::{Rc, Weak};
use std::cell::RefCell;

/// 이중 연결 리스트의 각 노드를 정의합니다.
/// `next`는 소유권을 공유하기 위해 `Rc`를 사용하고,
/// `prev`는 순환 참조를 방지하기 위해 `Weak`를 사용합니다.
struct Node<T> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Weak<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            data,
            next: None,
            prev: None,
        }))
    }
}

/// 이중 연결 리스트 구조체입니다.
/// 특정 값을 찾아 삭제하기 위해 T에 PartialEq 제약 조건이 필요할 수 있습니다.
pub struct DoublyLinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    length: usize,
}

impl<T: PartialEq> DoublyLinkedList<T> {
    /// 비어있는 새 리스트를 생성합니다.
    pub fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    /// 리스트의 길이를 반환합니다.
    pub fn len(&self) -> usize {
        self.length
    }

    /// 리스트의 맨 앞에 아이템을 추가합니다.
    pub fn push_front(&mut self, data: T) {
        let new_node = Node::new(data);

        match self.head.take() {
            Some(old_head) => {
                new_node.borrow_mut().next = Some(Rc::clone(&old_head));
                old_head.borrow_mut().prev = Some(Rc::downgrade(&new_node));
                self.head = Some(new_node);
            }
            None => {
                self.tail = Some(Rc::clone(&new_node));
                self.head = Some(new_node);
            }
        }
        self.length += 1;
    }

    /// 리스트의 맨 뒤에 아이템을 추가합니다.
    pub fn push_back(&mut self, data: T) {
        let new_node = Node::new(data);

        match self.tail.take() {
            Some(old_tail) => {
                new_node.borrow_mut().prev = Some(Rc::downgrade(&old_tail));
                old_tail.borrow_mut().next = Some(Rc::clone(&new_node));
                self.tail = Some(new_node);
            }
            None => {
                self.head = Some(Rc::clone(&new_node));
                self.tail = Some(new_node);
            }
        }
        self.length += 1;
    }

    /// 리스트의 맨 앞 아이템을 삭제하고 반환합니다.
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            self.length -= 1;
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev = None;
                    self.head = Some(new_head);
                }
                None => {
                    self.tail = None;
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().data
        })
    }

    /// 리스트의 맨 뒤 아이템을 삭제하고 반환합니다.
    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            self.length -= 1;
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail_weak) => {
                    let new_tail = new_tail_weak.upgrade().unwrap();
                    new_tail.borrow_mut().next = None;
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head = None;
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().data
        })
    }

    /// 전달받은 값과 일치하는 첫 번째 노드를 삭제합니다.
    /// 삭제 성공 시 true, 값을 찾지 못하면 false를 반환합니다.
    pub fn remove(&mut self, data: T) -> bool {
        let mut current = self.head.clone();

        while let Some(node) = current {
            if node.borrow().data == data {
                // 노드를 찾았으므로 연결 수정
                let prev_weak = node.borrow().prev.clone();
                let next_rc = node.borrow().next.clone();

                // 1. 이전 노드의 next를 현재 노드의 next로 연결
                if let Some(ref weak) = prev_weak {
                    if let Some(prev_node) = weak.upgrade() {
                        prev_node.borrow_mut().next = next_rc.clone();
                    }
                } else {
                    // 이전 노드가 없다면 현재 노드가 head임
                    self.head = next_rc.clone();
                }

                // 2. 다음 노드의 prev를 현재 노드의 prev로 연결
                if let Some(ref next_node) = next_rc {
                    next_node.borrow_mut().prev = prev_weak;
                } else {
                    // 다음 노드가 없다면 현재 노드가 tail임
                    self.tail = prev_weak.and_then(|w| w.upgrade());
                }

                self.length -= 1;
                return true;
            }
            // 다음 노드로 이동
            current = node.borrow().next.clone();
        }
        false
    }
}

fn main() {
    let mut list = DoublyLinkedList::new();

    println!("--- 초기 데이터 추가 ---");
    list.push_back(10);
    list.push_back(20);
    list.push_back(30);
    list.push_back(40);
    // 상태: [10, 20, 30, 40]

    println!("리스트 길이: {}", list.len());

    println!("--- 중간 삭제 테스트 (20 제거) ---");
    if list.remove(20) {
        println!("20 삭제 성공");
    }

    println!("--- 경계 조건 삭제 테스트 (10, 40 제거) ---");
    list.remove(10); // Head 삭제
    list.remove(40); // Tail 삭제

    println!("남은 리스트 길이: {}", list.len());
    println!("마지막 남은 값 (예상 30): {:?}", list.pop_front());
    println!("리스트가 비었는가? {:?}", list.pop_front().is_none());
}