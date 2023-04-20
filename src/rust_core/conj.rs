use std::rc::Rc;
use itertools::fold;
use crate::error_message;
use crate::ifn::IFn;
use crate::persistent_list::PersistentList;
use crate::persistent_vector::{PersistentVector, ToPersistentVectorIter};
use crate::rust_core::ConcatFn;
use crate::type_tag::TypeTag;
use crate::value::{ToValue, Value};

/// (conj coll x)
///
#[derive(Debug, Clone)]
pub struct ConjFn {}
impl ToValue for ConjFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for ConjFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        if args.len() < 2 {
            return Value::Condition(format!(
                "Wrong number of arguments given to function (Given: {}, Expected: 2)",
                args.len()
            ));
        }

        let coll = args.get(0).unwrap();
        let Value::PersistentVector(vec) = &**coll
            else { return error_message::type_mismatch(TypeTag::PersistentVector, &coll.to_value()) };

        let vals = Rc::new(vec.clone()).iter().collect::<Vec<Rc<Value>>>();
        let args = &args[1..];

        let concatted_vec = args.iter().fold(vals, |mut sum, coll| {
            sum.push(Rc::clone(coll));
            sum
        });
        Value::PersistentVector(concatted_vec.into_iter().collect::<PersistentVector>())
    }
}