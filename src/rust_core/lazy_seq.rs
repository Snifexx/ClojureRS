use std::rc::Rc;
use crate::lazy_sequence::LazySequence;
use crate::error_message;
use crate::ifn::IFn;
use crate::protocol::ProtocolCastable;
use crate::type_tag::TypeTag;
use crate::value::{ToValue, Value};

// (lazy-seq 0 (fn [prev] (inc prev)))
// Creates a lazy sequence differently from standard clojure
#[derive(Debug, Clone)]
pub struct LazySeqFn {}
impl ToValue for LazySeqFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for LazySeqFn {
    fn invoke(&self, mut args: Vec<Rc<Value>>) -> Value {
        if args.len() != 2 {
            return error_message::wrong_arg_count(2, args.len());
        }

        let fn_val = args.pop().unwrap();
        if let Value::IFn(_) = &*fn_val {
            let cons = args.pop().unwrap();
            let lseq = LazySequence::new(cons, fn_val).unwrap();
            return Value::LazySequence(lseq)
        }
        Value::Condition(format!(
            "Type mismatch; Expected instance of {}, Recieved type {}",
            TypeTag::IFn,
            args.len()
        ))
    }
}
