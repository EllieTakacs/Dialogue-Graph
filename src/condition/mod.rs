use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

/// A conditional weight that must evaluate as true in order to be avaliable to
/// make an edge clear to traverse.
pub trait Condition {
    /// Returns whether the node can be traversed based on its criteria.
    fn evaluate(&self) -> bool;
}

/// A condition that always evaluates to true.
#[derive(Serialize, Deserialize)]
pub struct True {}

impl Condition for True {
    fn evaluate(&self) -> bool {
        true
    }
}

/// A condition that evaluates as true if its inner condition evaluates as
/// false.
#[derive(Serialize)]
pub struct Not<'de, T>
where
    T: Condition + Serialize + Deserialize<'de>,
{
    condition: Box<T>,
    phantom: PhantomData<&'de T>,
}

impl<'de, T> Condition for Not<'de, T>
where
    T: Condition + Serialize + Deserialize<'de>,
{
    fn evaluate(&self) -> bool {
        !self.condition.evaluate()
    }
}

/// A condition that evaluates as true if both inner conditions evaluate as
/// true.
#[derive(Serialize)]
pub struct And<'de, T>
where
    T: Condition + Serialize + Deserialize<'de>,
{
    left: Box<T>,
    right: Box<T>,
    phantom: PhantomData<&'de T>,
}

impl<'de, T> Condition for And<'de, T>
where
    T: Condition + Serialize + Deserialize<'de>,
{
    fn evaluate(&self) -> bool {
        self.left.evaluate() && self.right.evaluate()
    }
}

/// A condition that evaluates as true if either inner condition evaluates as
/// true.
#[derive(Serialize)]
pub struct Or<'de, T>
where
    T: Condition + Serialize + Deserialize<'de>,
{
    left: Box<T>,
    right: Box<T>,
    phantom: PhantomData<&'de T>,
}

impl<'de, T> Condition for Or<'de, T>
where
    T: Condition + Serialize + Deserialize<'de>,
{
    fn evaluate(&self) -> bool {
        self.left.evaluate() || self.right.evaluate()
    }
}

/// A condition that evaluates an inner function with data
#[derive(Serialize, Deserialize)]
pub struct Function<D, F>
where
    F: Fn(&D) -> bool,
{
    data: D,
    condition: F,
}

impl<D, F> Condition for Function<D, F>
where
    F: Fn(&D) -> bool,
{
    fn evaluate(&self) -> bool {
        (self.condition)(&self.data)
    }
}
