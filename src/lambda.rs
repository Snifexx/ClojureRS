use std::env::var;
use crate::environment::Environment;
use crate::ifn::IFn;
use crate::persistent_list::ToPersistentList;
use crate::symbol::Symbol;
use crate::value::{Evaluable, ToValue, Value};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Fn {
    // Make faster implementation with custom hash: HashMap<(vararg: bool, argnum: usize)
    // For now this is a vector of args and a body
    pub implementations: Rc<Vec<(Vec<Symbol>, Rc<Value>)>>,
    // Closed over variables
    pub enclosing_environment: Rc<Environment>,
}

impl ToValue for Fn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}

impl IFn for Fn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        let local_environment = Rc::new(Environment::new_local_environment(Rc::clone(
            &self.enclosing_environment,
        )));

        for (arg_syms, body) in &*self.implementations {
            let argc = arg_syms.len();

            let mut var_args = false;
            if argc >= 2 {
                if let Some(sym) = arg_syms.get(argc - 2) {
                    if sym.to_string() == "&" {
                        var_args = true;
                        let last_sym = arg_syms.get(argc - 1).unwrap();
                        local_environment.insert(last_sym.clone(), Rc::new(Value::Nil));
                    }
                }
            }

            if (var_args && args.len() < argc - 2) || (!var_args && args.len() != argc) { continue; }

            for (i, arg) in args.iter().enumerate() {
                let curr_sym = arg_syms.get(i).unwrap();
                // We can bind the rest of the arguments, then, to the next variable and blow this popsicle stand
                if curr_sym.to_string() == "&" {
                    if !var_args {
                        return Value::Condition(String::from("Invalid function argument '&' in non-variable-argument function definition"));
                    }
                    let last_sym = arg_syms.get(i + 1).unwrap();
                    let rest_args = args.get(i..).unwrap().to_vec().into_list().to_rc_value();
                    local_environment.insert(last_sym.clone(), rest_args);
                    break;
                }
                local_environment.insert(curr_sym.clone(), arg.to_rc_value());
            }

            return body.eval(local_environment);
        }

        Value::Condition(format!("Wrong number of arguments given to function"))
    }
}

///
/// Tests
///
#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::environment::Environment;
    use crate::ifn::IFn;
    use crate::lambda;
    use crate::symbol::Symbol;
    use crate::value::Value;

    #[test]
    fn test_only_vararg() {
        // (defn func [& vararg] "Works")
        let func = lambda::Fn {
            implementations: Rc::new(vec![(
                vec![Symbol::intern("&"), Symbol::intern("varargs")],
                Rc::new(Value::String(String::from("Works")))
            )]),
            enclosing_environment: Rc::new(Environment::new_local_environment(Environment::clojure_core_environment())),
        };

        let val = func.invoke(vec![]); // (func)
        assert_eq!(val, Value::String("Works".to_string()));

        let val = func.invoke(vec![Rc::new(Value::I32(1_i32))]); // (func 1)
        assert_eq!(val, Value::String("Works".to_string()));

        let val = func.invoke(vec![    //
                                       Rc::new(Value::I32(1_i32)),      //  (func 1 2)
                                       Rc::new(Value::I32(2_i32)),      //
        ]);                                        //
        assert_eq!(val, Value::String("Works".to_string()));
    }

    #[test]
    fn test_vararg_one() {
        // (defn func [x & vararg] "Works")
        let func = lambda::Fn {
            implementations: Rc::new(vec![(
                vec![Symbol::intern("x"), Symbol::intern("&"), Symbol::intern("varargs")],
                Rc::new(Value::String(String::from("Works"))),
            )]),
            enclosing_environment: Rc::new(Environment::new_local_environment(Environment::clojure_core_environment())),
        };

        let val = func.invoke(vec![]); // (func)
        assert_eq!(val, Value::Condition("Wrong number of arguments given to function".to_string()));

        let val = func.invoke(vec![Rc::new(Value::I32(1_i32))]); // (func 1)
        assert_eq!(val, Value::String("Works".to_string()));

        let val = func.invoke(vec![                                    //  (func 1 2)
                                                                       Rc::new(Value::I32(1_i32)),
                                                                       Rc::new(Value::I32(2_i32)),
        ]);
        assert_eq!(val, Value::String("Works".to_string()));
    }

    #[test]
    fn test_vararg_two() {
        // (defn func [x y & vararg] "Works")
        let func = lambda::Fn {
            implementations: Rc::new(vec![(
                vec![Symbol::intern("x"), Symbol::intern("y"),
                     Symbol::intern("&"), Symbol::intern("varargs")],
                Rc::new(Value::String(String::from("Works"))),
            )]),
            enclosing_environment: Rc::new(Environment::new_local_environment(Environment::clojure_core_environment())),
        };

        let val = func.invoke(vec![]); // (func)
        assert_eq!(val, Value::Condition("Wrong number of arguments given to function".to_string()));

        let val = func.invoke(vec![Rc::new(Value::I32(1_i32))]); // (func 1)
        assert_eq!(val, Value::Condition("Wrong number of arguments given to function".to_string()));

        let val = func.invoke(vec![ //  (func 1 2)
                                    Rc::new(Value::I32(1_i32)),
                                    Rc::new(Value::I32(2_i32)),
        ]);
        assert_eq!(val, Value::String("Works".to_string()));

        let val = func.invoke(vec![ //  (func 1 2 "vararg here")
                                    Rc::new(Value::I32(1_i32)),
                                    Rc::new(Value::I32(2_i32)),
                                    Rc::new(Value::String(String::from("vararg here"))),
        ]);
        assert_eq!(val, Value::String("Works".to_string()));
    }
}

