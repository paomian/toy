use std::{collections::BTreeMap, marker::PhantomData, ptr::NonNull};

pub struct MyList<K, T>
where
    K: Ord,
{
    front: Pointer<K, T>,
    back: Pointer<K, T>,
    len: usize,
    _boo: PhantomData<T>,
}

type Pointer<K, T> = Option<NonNull<Node<K, T>>>;

type Front<K, T> = Option<NonNull<BTreeMap<K, Pointer<K, T>>>>;

struct Node<K, T> {
    front: Front<K, T>,
    back: Pointer<K, T>,
    elem: T,
}

type NodeName = String;

struct TreePath {
    path: Vec<NodeName>,
}

impl<K, T> MyList<K, T>
where
    K: Ord,
{
    pub fn new() -> Self {
        Self {
            front: None,
            back: None,
            len: 0,
            _boo: PhantomData,
        }
    }

    pub fn push_front(&mut self, k: K, elem: T) {
        unsafe {
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                front: None,
                back: None,
                elem,
            })));
            if let Some(old) = self.front {
                match (*old.as_ptr()).front {
                    Some(mut front) => {
                        front.as_mut().insert(k, Some(new));
                    }
                    None => {
                        let mut map = Box::new(BTreeMap::new());
                        map.insert(k, Some(new));
                        (*old.as_ptr()).front = NonNull::new(Box::into_raw(map));
                    }
                }
            } else {
                self.back = Some(new);
            }
            self.front = Some(new);
            self.len += 1;
        }
    }

    pub fn push_back(&mut self, k: K, elem: T) {
        unsafe {
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                front: None,
                back: None,
                elem,
            })));
            if let Some(old) = self.back {
                (*old.as_ptr()).back = Some(new);
                let mut map = Box::new(BTreeMap::new());
                map.insert(k, Some(old));
                (*new.as_ptr()).front = NonNull::new(Box::into_raw(map));
            } else {
                self.front = Some(new);
            }
            self.back = Some(new);
            self.len += 1;
        }
    }
}
