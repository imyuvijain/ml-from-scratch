use std::{cell::RefCell, os::unix::thread};

//tape that logs every node/layer
thread_local! {
    static TAPE: RefCell<Vec<Node>> = RefCell::new(Vec::new());
}

#[derive(Clone,Copy)]
enum Op {
    Leaf,
    Add (usize, usize),
    Mul (usize, usize),
    Tanh (usize),
    Exp (usize),
    Pow (usize, f32),
}
#[derive(Clone,Copy)]
struct Node {
    data: f32, //result
    grad: f32, // gradient
    op: Op, //
}

#[derive(Clone, Copy, Debug)]
pub struct Value(usize);

impl Value {
    fn push(data: f32, op: Op) -> Value {
        
        TAPE.with(|tape| {
            let mut t = tape.borrow_mut();
            let i = t.len();

            t.push(Node { data, grad: 0.0, op });

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
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let v = Value::new(3.0);
        assert_eq!((v.data(), v.grad()), (3.0, 0.0));
    }
}