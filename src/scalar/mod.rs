pub mod gradcheck;

use std::{
    cell::RefCell,
    ops::{Add, Mul, Neg, Sub},
    process::Output,
};


//tape that logs every node/layer
thread_local! {
    static TAPE: RefCell<Vec<Node>> = RefCell::new(Vec::new());
}


pub fn zero_grads() {
        TAPE.with(|tape| {
            let mut t = tape.borrow_mut();

            for node in t.iter_mut() {
                node.grad = 0.0;
            }

        })
}

#[derive(Clone, Copy, Debug)]
enum Op {
    Leaf,
    Add(usize, usize),
    Mul(usize, usize),
    Tanh(usize),
    Exp(usize),
    Pow(usize, f32),
    Relu(usize),
}


//NODE 
#[derive(Clone, Copy, Debug)]
struct Node {
    data: f32, //result
    grad: f32, // gradient
    op: Op,    //
}

impl Node {
    
}

//NODE VALUE
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

    pub fn backwards(self) {
        TAPE.with(|tape| {
            let mut t = tape.borrow_mut();
            t[self.0].grad = 1.0;

            
            for i in (0..=self.0).rev() {
                let node = t[i];
                let g = node.grad;

                match node.op {
                    Op::Leaf => {}
                    Op::Add(p, q) => {
                        t[p].grad += g;
                        t[q].grad += g;
                    }
                    Op::Mul(p, q) => {
                        t[q].grad += t[p].data * g;
                        t[p].grad += t[q].data * g;
                    }
                    Op::Tanh(p) => {
                        t[p].grad += (1.0 - node.data * node.data) * g;
                    }
                    Op::Exp(p) => {
                        t[p].grad += node.data * g;
                    }
                    Op::Relu(p) => {
                        if t[p].data > 0.0 {
                            t[p].grad += g;
                        }
                    }
                    Op::Pow(p, n) => {
                        t[p].grad += n * t[p].data.powf(n - 1.0) * g;
                    }
                }

            }

        })
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
        Value::push(self.data() * rhs.data(), Op::Mul(self.0, rhs.0))
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
    fn forward_ops_test() {
        let tol = 1e-6;

        let t1 = ((Value::new(2.0) * Value::new(3.0)) + 1.0).tanh();
        assert_close(t1.data(), 7.0_f32.tanh(), tol);

        let t2 = Value::new(5.0) - Value::new(3.0);
        assert_close(t2.data(), 2.0, tol);

        assert_close(Value::new(-2.0).relu().data(), 0.0, tol);
        assert_close(Value::new(4.0).relu().data(), 4.0, tol);

        assert_close(Value::new(0.0).exp().data(), 1.0, tol);
        assert_close(Value::new(3.0).pow(2.0).data(), 9.0, tol);

    }

    #[test]
    fn backwards_test() {

        let tol = 1e-6;

        let c = Value::new(3.0);
        let c_output = c.pow(2.0);
        c_output.backwards();
        
        assert_close(c.grad(), 6.0, tol);

        let a = Value::new(2.0);
        let b = Value::new(5.0);
        let ab_output = a * b + a;

        ab_output.backwards();

        assert_close(a.grad(), 6.0, tol);
        assert_close(b.grad(), 2.0, tol);
    }

    #[test]
    fn backwards_grad_test() {

        let tol = 1e-2;

        let a = Value::new(0.7);
        let b = Value::new(-1.3);

        let output = (a*b + a.pow(2.0)).tanh();
        output.backwards();


        let h = 1e-3;
        let grad = gradcheck::numeric_grad(|v| {

            let (p, q) = (Value::new(v[0]), Value::new(v[1]));

            (p * q + p.pow(2.0)).tanh().data()

        }, &[0.7, -1.3], h);

        assert_close(grad[0], a.grad(), tol);
        assert_close(grad[1], b.grad(), tol);

    }

    
}
        


