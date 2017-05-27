use std::mem;
use {TRUE, FALSE};
use core::Core;
use exception::Exception::InvalidMemoryAddress;

pub trait Float: Core {
    fn add_float(&mut self) {
        self.add_primitive("fconstant", Float::fconstant);
        self.add_primitive("fvariable", Float::fvariable);
        self.add_primitive("f!", Float::fstore);
        self.add_primitive("f@", Float::ffetch);
        self.add_primitive("fabs", Float::fabs);
        self.add_primitive("fsin", Float::fsin);
        self.add_primitive("fcos", Float::fcos);
        self.add_primitive("ftan", Float::ftan);
        self.add_primitive("fasin", Float::fasin);
        self.add_primitive("facos", Float::facos);
        self.add_primitive("fatan", Float::fatan);
        self.add_primitive("fatan2", Float::fatan2);
        self.add_primitive("fsqrt", Float::fsqrt);
        self.add_primitive("fdrop", Float::fdrop);
        self.add_primitive("fdup", Float::fdup);
        self.add_primitive("fswap", Float::fswap);
        self.add_primitive("fnip", Float::fnip);
        self.add_primitive("frot", Float::frot);
        self.add_primitive("fover", Float::fover);
        self.add_primitive("n>f", Float::n_to_f);
        self.add_primitive("f+", Float::fplus);
        self.add_primitive("f-", Float::fminus);
        self.add_primitive("f*", Float::fstar);
        self.add_primitive("f/", Float::fslash);
        self.add_primitive("f~", Float::fproximate);
        self.add_primitive("f0<", Float::f_zero_less_than);
        self.add_primitive("f0=", Float::f_zero_equals);
        self.add_primitive("f<", Float::f_less_than);
        self.add_primitive("fmin", Float::fmin);
        self.add_primitive("fmax", Float::fmax);
        self.add_primitive("floor", Float::floor);
        self.add_primitive("fround", Float::fround);
        self.add_primitive("fceil", Float::fceil);
        self.add_primitive("fnegate", Float::fnegate);
    }

    fn p_fconst(&mut self) {
        let wp = self.state().word_pointer();
        let dfa = self.wordlist()[wp].dfa();
        let v = self.data_space().get_f64(dfa);
        self.f_stack().push(v);
    }

    fn fvariable(&mut self) {
        self.define(Core::p_fvar, Core::compile_word);
        self.data_space().compile_f64(0.0);
    }

    fn fconstant(&mut self) {
        let v = self.f_stack().pop();
        self.define(Float::p_fconst, Core::compile_word);
        self.data_space().compile_f64(v);
    }

    // Floating point primitives

    fn ffetch(&mut self) {
        let t = self.s_stack().pop();
        if (t as usize + mem::size_of::<f64>()) <= self.data_space().capacity() {
            let value = self.data_space().get_f64(t as usize);
            self.f_stack().push(value);
        } else {
            self.abort_with(InvalidMemoryAddress);
        }
    }

    fn fstore(&mut self) {
        let t = self.s_stack().pop();
        let n = self.f_stack().pop();
        if (t as usize + mem::size_of::<f64>()) <= self.data_space().capacity() {
            self.data_space().put_f64(n, t as usize);
        } else {
            self.abort_with(InvalidMemoryAddress);
        }
    }

    fn fabs(&mut self) {
        let t = self.f_stack().pop();
        self.f_stack().push(t.abs());
    }

    fn fsin(&mut self) {
        let t = self.f_stack().pop();
        self.f_stack().push(t.sin());
    }

    fn fcos(&mut self) {
        let t = self.f_stack().pop();
        self.f_stack().push(t.cos());
    }

    fn ftan(&mut self) {
        let t = self.f_stack().pop();
        self.f_stack().push(t.tan());
    }

    fn fasin(&mut self) {
        let t = self.f_stack().pop();
        self.f_stack().push(t.asin());
    }

    fn facos(&mut self) {
        let t = self.f_stack().pop();
        self.f_stack().push(t.acos());
    }

    fn fatan(&mut self) {
        let t = self.f_stack().pop();
        self.f_stack().push(t.atan());
    }

    fn fatan2(&mut self) {
        let t = self.f_stack().pop();
        let n = self.f_stack().pop();
        self.f_stack().push(n.atan2(t));
    }

    fn fsqrt(&mut self) {
        let t = self.f_stack().pop();
        self.f_stack().push(t.sqrt());
    }

    fn fswap(&mut self) {
        let t = self.f_stack().pop();
        let n = self.f_stack().pop();
        self.f_stack().push2(t, n);
    }

    fn fnip(&mut self) {
        let t = self.f_stack().pop();
        let _ = self.f_stack().pop();
        self.f_stack().push(t);
    }

    fn fdup(&mut self) {
        let t = self.f_stack().pop();
        self.f_stack().push2(t, t);
    }

    fn fdrop(&mut self) {
        let _ = self.f_stack().pop();
    }

    fn frot(&mut self) {
        let x3 = self.f_stack().pop();
        let x2 = self.f_stack().pop();
        let x1 = self.f_stack().pop();
        self.f_stack().push3(x2, x3, x1);
    }

    fn fover(&mut self) {
        let t = self.f_stack().pop();
        let n = self.f_stack().pop();
        self.f_stack().push3(n, t, n);
    }

    fn n_to_f(&mut self) {
        let t = self.s_stack().pop();
        self.f_stack().push(t as f64);
    }

    fn fplus(&mut self) {
        let t = self.f_stack().pop();
        let n = self.f_stack().pop();
        self.f_stack().push(n + t);
    }

    fn fminus(&mut self) {
        let t = self.f_stack().pop();
        let n = self.f_stack().pop();
        self.f_stack().push(n - t);
    }

    fn fstar(&mut self) {
        let t = self.f_stack().pop();
        let n = self.f_stack().pop();
        self.f_stack().push(n * t);
    }

    fn fslash(&mut self) {
        let t = self.f_stack().pop();
        let n = self.f_stack().pop();
        self.f_stack().push(n / t);
    }

    fn fproximate(&mut self) {
        let (x1, x2, x3) = self.f_stack().pop3();
        if x3 > 0.0 {
            self.s_stack()
                .push(if (x1 - x2).abs() < x3 { TRUE } else { FALSE });
        } else if x3 == 0.0 {
            self.s_stack().push(if x1 == x2 { TRUE } else { FALSE });
        } else {
            self.s_stack()
                .push(if (x1 - x2).abs() < (x3.abs() * (x1.abs() + x2.abs())) {
                          TRUE
                      } else {
                          FALSE
                      });
        }
    }

    fn f_zero_less_than(&mut self) {
        let t = self.f_stack().pop();
        self.s_stack().push(if t < 0.0 { TRUE } else { FALSE });
    }

    fn f_zero_equals(&mut self) {
        let t = self.f_stack().pop();
        self.s_stack().push(if t == 0.0 { TRUE } else { FALSE });
    }

    fn f_less_than(&mut self) {
        let t = self.f_stack().pop();
        let n = self.f_stack().pop();
        self.s_stack().push(if n < t { TRUE } else { FALSE });
    }

    fn fmin(&mut self) {
        let (n, t) = self.f_stack().pop2();
        self.f_stack().push(t.min(n));
    }

    fn fmax(&mut self) {
        let (n, t) = self.f_stack().pop2();
        self.f_stack().push(t.max(n));
    }

    fn fround(&mut self) {
        let t = self.f_stack().pop();
        self.f_stack().push(t.round());
    }

    fn floor(&mut self) {
        let t = self.f_stack().pop();
        self.f_stack().push(t.floor());
    }

    fn fceil(&mut self) {
        let t = self.f_stack().pop();
        self.f_stack().push(t.ceil());
    }

    fn fnegate(&mut self) {
        let t = self.f_stack().pop();
        self.f_stack().push(-t);
    }
}

#[cfg(test)]
mod tests {
    use vm::VM;
    use core::Core;
    use super::Float;

    #[test]
    fn test_evaluate_f64() {
        let vm = &mut VM::new(16);
        vm.set_source("1.0 2.5");
        vm.evaluate();
        assert_eq!(vm.last_error(), None);
        assert_eq!(vm.f_stack().len(), 2);
        assert!(0.99999 < vm.f_stack().as_slice()[0]);
        assert!(vm.f_stack().as_slice()[0] < 1.00001);
        assert!(2.49999 < vm.f_stack().as_slice()[1]);
        assert!(vm.f_stack().as_slice()[1] < 2.50001);
    }

    #[test]
    fn test_fconstant() {
        let vm = &mut VM::new(16);
        vm.set_source("1.1 fconstant x x x");
        vm.evaluate();
        assert_eq!(vm.last_error(), None);
        assert_eq!(vm.f_stack().as_slice(), [1.1, 1.1]);
    }

    #[test]
    fn test_fvariable_and_fstore_ffetch() {
        let vm = &mut VM::new(16);
        vm.set_source("fvariable fx  fx f@  3.3 fx f!  fx f@");
        vm.evaluate();
        assert_eq!(vm.last_error(), None);
        assert_eq!(vm.f_stack().as_slice(), [0.0, 3.3]);
    }

    #[test]
    fn test_fabs() {
        let vm = &mut VM::new(16);
        vm.set_source("-3.14 fabs");
        vm.evaluate();
        assert_eq!(vm.last_error(), None);
        assert_eq!(vm.f_stack().len(), 1);
        assert!(match vm.f_stack().pop() {
                    t => t > 3.13999 && t < 3.14001,
                });
    }

    #[test]
    fn test_fsin() {
        let vm = &mut VM::new(16);
        vm.set_source("3.14 fsin");
        vm.evaluate();
        assert_eq!(vm.last_error(), None);
        assert_eq!(vm.f_stack().len(), 1);
        assert!(match vm.f_stack().pop() {
                    t => t > 0.0015925 && t < 0.0015927,
                });
    }

    #[test]
    fn test_fcos() {
        let vm = &mut VM::new(16);
        vm.set_source("3.0 fcos");
        vm.evaluate();
        assert_eq!(vm.last_error(), None);
        assert_eq!(vm.f_stack().len(), 1);
        assert!(match vm.f_stack().pop() {
                    t => t > -0.989993 && t < -0.989991,
                });
    }

    #[test]
    fn test_ftan() {
        let vm = &mut VM::new(16);
        vm.set_source("3.0 ftan");
        vm.evaluate();
        assert_eq!(vm.last_error(), None);
        assert_eq!(vm.f_stack().len(), 1);
        assert!(match vm.f_stack().pop() {
                    t => t > -0.142547 && t < -0.142545,
                });
    }

    #[test]
    fn test_fasin() {
        let vm = &mut VM::new(16);
        vm.set_source("0.3 fasin");
        vm.evaluate();
        assert_eq!(vm.last_error(), None);
        assert_eq!(vm.f_stack().len(), 1);
        assert!(match vm.f_stack().pop() {
                    t => t > 0.304691 && t < 0.304693,
                });
    }

    #[test]
    fn test_facos() {
        let vm = &mut VM::new(16);
        vm.set_source("0.3 facos");
        vm.evaluate();
        assert_eq!(vm.last_error(), None);
        assert_eq!(vm.f_stack().len(), 1);
        assert!(match vm.f_stack().pop() {
                    t => t > 1.266102 && t < 1.266104,
                });
    }

    #[test]
    fn test_fatan() {
        let vm = &mut VM::new(16);
        vm.set_source("0.3 fatan");
        vm.evaluate();
        assert_eq!(vm.last_error(), None);
        assert_eq!(vm.f_stack().len(), 1);
        assert!(match vm.f_stack().pop() {
                    t => t > 0.291455 && t < 0.291457,
                });
    }

    #[test]
    fn test_fatan2() {
        let vm = &mut VM::new(16);
        vm.set_source("3.0 4.0 fatan2");
        vm.evaluate();
        assert_eq!(vm.last_error(), None);
        assert_eq!(vm.f_stack().len(), 1);
        assert!(match vm.f_stack().pop() {
                    t => t > 0.643500 && t < 0.643502,
                });
    }

    #[test]
    fn test_fsqrt() {
        let vm = &mut VM::new(16);
        vm.set_source("0.3 fsqrt");
        vm.evaluate();
        assert_eq!(vm.last_error(), None);
        assert_eq!(vm.f_stack().len(), 1);
        assert!(match vm.f_stack().pop() {
                    t => t > 0.547721 && t < 0.547723,
                });
    }

    #[test]
    fn test_fdrop() {
        let vm = &mut VM::new(16);
        vm.f_stack().push(1.0);
        vm.fdrop();
        assert_eq!(vm.last_error(), None);
        assert_eq!(vm.f_stack().as_slice(), []);
    }

    #[test]
    fn test_fnip() {
        let vm = &mut VM::new(16);
        vm.f_stack().push2(1.0, 2.0);
        vm.check_stacks();
        match vm.last_error() {
            Some(_) => assert!(true, "Floating point stack overflow"),
            None => {}
        };
        vm.fnip();
        assert_eq!(vm.last_error(), None);
        assert_eq!(vm.f_stack().as_slice(), [2.0]);
    }

    #[test]
    fn test_fswap() {
        let vm = &mut VM::new(16);
        vm.f_stack().push2(1.0, 2.0);
        vm.check_stacks();
        match vm.last_error() {
            Some(_) => assert!(true, "Floating point stack overflow"),
            None => {}
        };
        vm.fswap();
        assert_eq!(vm.last_error(), None);
        assert_eq!(vm.f_stack().as_slice(), [2.0, 1.0]);
    }

    #[test]
    fn test_fdup() {
        let vm = &mut VM::new(16);
        vm.f_stack().push(1.0);
        vm.fdup();
        vm.check_stacks();
        assert_eq!(vm.last_error(), None);
        assert_eq!(vm.f_stack().as_slice(), [1.0, 1.0]);
    }

    #[test]
    fn test_fover() {
        let vm = &mut VM::new(16);
        vm.f_stack().push2(1.0, 2.0);
        vm.fover();
        vm.check_stacks();
        assert_eq!(vm.last_error(), None);
        assert_eq!(vm.f_stack().as_slice(), [1.0, 2.0, 1.0]);
    }

    #[test]
    fn test_frot() {
        let vm = &mut VM::new(16);
        vm.f_stack().push3(1.0, 2.0, 3.0);
        vm.frot();
        vm.check_stacks();
        assert_eq!(vm.last_error(), None);
        assert_eq!(vm.f_stack().as_slice(), [2.0, 3.0, 1.0]);
    }

    #[test]
    fn test_fplus_fminus_fstar_fslash() {
        let vm = &mut VM::new(16);
        vm.set_source("9.0 10.0 f+ 11.0 f- 12.0 f* 13.0 f/");
        vm.evaluate();
        assert_eq!(vm.last_error(), None);
        assert_eq!(vm.f_stack().len(), 1);
        assert!(match vm.f_stack().pop() {
                    t => t > 7.384614 && t < 7.384616,
                });
        vm.check_stacks();
        assert_eq!(vm.last_error(), None);
    }

    #[test]
    fn test_f_zero_less_than() {
        let vm = &mut VM::new(16);
        vm.set_source("0.0 f0<   0.1 f0<   -0.1 f0<");
        vm.evaluate();
        assert_eq!(vm.last_error(), None);
        assert_eq!(vm.s_stack().len(), 3);
        assert_eq!(vm.s_stack().pop(), -1);
        assert_eq!(vm.s_stack().pop(), 0);
        assert_eq!(vm.s_stack().pop(), 0);
        assert_eq!(vm.f_stack().as_slice(), []);
        vm.check_stacks();
        assert_eq!(vm.last_error(), None);
    }

    #[test]
    fn test_f_zero_equals() {
        let vm = &mut VM::new(16);
        vm.set_source("0.0 f0=   0.1 f0=   -0.1 f0=");
        vm.evaluate();
        assert_eq!(vm.last_error(), None);
        assert_eq!(vm.s_stack().len(), 3);
        assert_eq!(vm.s_stack().pop(), 0);
        assert_eq!(vm.s_stack().pop(), 0);
        assert_eq!(vm.s_stack().pop(), -1);
        assert_eq!(vm.f_stack().as_slice(), []);
        vm.check_stacks();
        assert_eq!(vm.last_error(), None);
    }

    #[test]
    fn test_f_less_than() {
        let vm = &mut VM::new(16);
        vm.set_source("0.0 0.0 f<   0.1 0.0 f<   -0.1 0.0 f<");
        vm.evaluate();
        assert_eq!(vm.last_error(), None);
        assert_eq!(vm.s_stack().len(), 3);
        assert_eq!(vm.s_stack().pop(), -1);
        assert_eq!(vm.s_stack().pop(), 0);
        assert_eq!(vm.s_stack().pop(), 0);
        assert_eq!(vm.f_stack().as_slice(), []);
        vm.check_stacks();
        assert_eq!(vm.last_error(), None);
    }

    #[test]
    fn test_fproximate() {
        let vm = &mut VM::new(16);
        vm.set_source("0.1 0.1 0.0 f~   0.1 0.10000000001 0.0 f~");
        vm.evaluate();
        assert_eq!(vm.last_error(), None);
        assert_eq!(vm.s_stack().len(), 2);
        assert_eq!(vm.s_stack().pop(), 0);
        assert_eq!(vm.s_stack().pop(), -1);
        assert_eq!(vm.f_stack().as_slice(), []);
        vm.check_stacks();
        assert_eq!(vm.last_error(), None);
        vm.s_stack().reset();
        vm.set_source("0.1 0.1 0.001 f~   0.1 0.109 0.01 f~   0.1 0.111  0.01 f~");
        vm.evaluate();
        assert_eq!(vm.last_error(), None);
        assert_eq!(vm.s_stack().len(), 3);
        assert_eq!(vm.s_stack().pop(), 0);
        assert_eq!(vm.s_stack().pop(), -1);
        assert_eq!(vm.s_stack().pop(), -1);
        assert_eq!(vm.f_stack().as_slice(), []);
        vm.check_stacks();
        assert_eq!(vm.last_error(), None);
        vm.s_stack().reset();
        vm.set_source("0.1 0.1 -0.001 f~   0.1 0.109 -0.1 f~   0.1 0.109  -0.01 f~");
        vm.evaluate();
        assert_eq!(vm.last_error(), None);
        assert_eq!(vm.s_stack().len(), 3);
        assert_eq!(vm.s_stack().pop(), 0);
        assert_eq!(vm.s_stack().pop(), -1);
        assert_eq!(vm.s_stack().pop(), -1);
        assert_eq!(vm.f_stack().as_slice(), []);
        vm.check_stacks();
        assert_eq!(vm.last_error(), None);
        vm.s_stack().reset();
    }

    #[test]
    fn test_n_to_f() {
        let vm = &mut VM::new(16);
        vm.set_source("0 n>f -1 n>f 1 n>f");
        vm.evaluate();
        assert_eq!(vm.last_error(), None);
        assert_eq!(vm.f_stack().as_slice(), [0.0, -1.0, 1.0]);
    }

    #[test]
    fn test_flit_and_compile_float() {
        let vm = &mut VM::new(16);
        vm.set_source(": test 1.0 2.0 ; test");
        vm.evaluate();
        assert_eq!(vm.last_error(), None);
        assert_eq!(vm.f_stack().as_slice(), [1.0, 2.0]);
    }
}
