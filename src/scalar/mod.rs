use std::{
    cell::RefCell,
    ops::{Add, Mul, Neg, Sub},
    process::Output,
};

use rand_distr::num_traits::Pow;

//tape that logs every node/layer
thread_local! {
    static TAPE: RefCell<Vec<Node>> = RefCell::new(Vec::new());
}

#[derive(Clone, Copy)]
enum Op {
    Leaf,
    Add(usize, usize),
    Mul(usize, usize),
    Tanh(usize),
    Exp(usize),
    Pow(usize, f32),
    Relu(usize),
}
#[derive(Clone, Copy)]
struct Node {
    data: f32, //result
    grad: f32, // gradient
    op: Op,    //
}

#[derive(Clone, Copy, Debug)]
pub struct Value(usize);

impl Value {
    fn push(data: f32, op: Op) -> Value {
        TAPE.with(|tape| {
            let mut t = tape.borrow_mut();
            let i = t.len();

            t.push(Node {
                data,
                grad: 0.0,
                op,
            });

            Value(i)
        })
    }

    pub fn new(x: f32) -> Value {
        Self::push(x, Op::Leaf)
    }

    pub fn data(self) -> f32 {
        TAPE.with(|tape| {
            let t = tape.borrow();

            t[self.0].data
        })
    }

    pub fn grad(self) -> f32 {
        TAPE.with(|tape| {
            let t = tape.borrow();

            t[self.0].grad
        })
    }

    pub fn set_data(self, x: f32) {
        TAPE.with(|tape| {
            let mut t = tape.borrow_mut();

            t[self.0].data = x
        })
    }

    pub fn tanh(self) -> Value {
        Value::push(self.data().tanh(), Op::Tanh(self.0))
    }
    pub fn relu(self) -> Value {
        Value::push(self.data().max(0.0), Op::Relu(self.0))
    }
    pub fn exp(self) -> Value {
        Value::push(self.data().exp(), Op::Exp(self.0))
    }
    pub fn pow(self, n: f32) -> Value {
        Value::push(self.data().powf(n), Op::Pow(self.0, n))
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Value) -> Value {
        Value::push(self.data() + rhs.data(), Op::Add(self.0, rhs.0))
    }
}

impl Add<f32> for Value {
    type Output = Value;
    fn add(self, rhs: f32) -> Value {
        self + Value::new(rhs)
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Value) -> Value {
        Value::push(self.data() * rhs.data(), Op::Add(self.0, rhs.0))
    }
}

impl Mul<f32> for Value {
    type Output = Value;
    fn mul(self, rhs: f32) -> Value {
        self * Value::new(rhs)
    }
}

impl Neg for Value {
    type Output = Value;

    fn neg(self) -> Value {
        self * -1.0
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Value) -> Value {
        self + -(rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::assert_close;

    #[test]
    fn test() {
        let v = Value::new(3.0);
        assert_eq!((v.data(), v.grad()), (3.0, 0.0));
    }

    #[test]
    fn forward_ops() {
        let tol = 1e-6;

        let t = ((Value::new(2.0) * Value::new(3.0)) + 1.0).tanh();
        assert_close(t.data(), 7.0_f32.tanh(), tol);

        let s = Value::new(5.0) - Value::new(3.0);
        assert_close(s.data(), 2.0, tol);

        assert_close(Value::new(-2.0).relu().data(), 0.0, tol);
        assert_close(Value::new(4.0).relu().data(), 4.0, tol);

        assert_close(Value::new(0.0).exp().data(), 1.0, tol);
        assert_close(Value::new(3.0).pow(2.0).data(), 9.0, tol);
    }
}
