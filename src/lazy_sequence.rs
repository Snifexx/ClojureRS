use std::fmt;
use std::iter::FromIterator;
use std::rc::Rc;
use crate::iterable::Iterable;
use crate::persistent_list_map::PersistentListMap;
use crate::traits;
use crate::ifn::IFn;
use crate::value::{ToValue, Value};

#[derive(fmt::Debug, Clone, Hash)]
pub struct LazySequence {
    pub cons: Rc<Value>,
    pub next: Rc<Value>, // This Value must be subclass of Ifn
}

impl LazySequence {
    pub fn new(cons: Rc<Value>, calc: Rc<Value>) -> Option<Self> {
        if let Value::IFn(_) = &*calc {
            return Some(LazySequence {
                cons,
                next: calc,
            })
        }
        None
    }
}

impl traits::IMeta for LazySequence {
    fn meta(&self) -> PersistentListMap {
        // @TODO implement
        PersistentListMap::Empty
    }
}
impl traits::IObj for LazySequence {
    fn with_meta(&self, meta: PersistentListMap) -> LazySequence {
        // @TODO implement
        self.clone()
    }
}
impl fmt::Display for LazySequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = format!("({}, |{:?}|)", self.cons.to_string_explicit(), self.next.to_string_explicit());
        write!(f, "{}", str)
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////
//
//  Iterating through Persistent List
//
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct LazySequenceIter {
    list: Rc<LazySequence>
}
pub trait ToLazySequenceIter {
    fn iter(&self) -> LazySequenceIter;
    fn nth(&self, ind: usize) -> Rc<Value> {
        self.iter().nth(ind).expect("Error Implementing LazySeqs")
    }
}
impl ToLazySequenceIter for Rc<LazySequence> {
    fn iter(&self) -> LazySequenceIter {
        LazySequenceIter {
            list: Rc::clone(self)
        }
    }
}
impl ToLazySequenceIter for &Rc<LazySequence> {
    fn iter(&self) -> LazySequenceIter {
        LazySequenceIter {
            list: Rc::clone(self)
        }
    }
}
impl ToLazySequenceIter for &LazySequence {
    fn iter(&self) -> LazySequenceIter {
        LazySequenceIter {
            list: Rc::new((*self).clone())
        }
    }
}
impl Iterator for LazySequenceIter {
    type Item = Rc<Value>;
    fn next(&mut self) -> Option<Self::Item> {
        let lseq = &*(self.list.clone());
        let Value::IFn(ifn) = &*(lseq.next) else { panic!("Error implementing LazySeq") };
        let next_val = ifn.invoke(vec![lseq.cons.clone()]);
        self.list = Rc::new(LazySequence { cons: Rc::new(next_val), next: ifn.to_rc_value() });
        Some(Rc::clone(&lseq.cons))
    }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
// End Iteration
////////////////////////////////////////////////////////////////////////////////////////////////////