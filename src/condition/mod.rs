use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

/// A conditional weight that must evaluate as true in order to be avaliable to
/// make an edge clear to traverse.
pub trait Condition<'de>: Deserialize<'de> {
    /// Returns whether the node can be traversed based on its criteria.
    fn evaluate(&self) -> bool;
}

/// A condition that always evaluates to true.
#[derive(Serialize, Deserialize)]
pub struct True {}

impl<'de> Condition<'de> for True {
    fn evaluate(&self) -> bool {
        true
    }
}

/// A condition that evaluates as true if its inner condition evaluates as
/// false.
#[derive(Serialize)]
pub struct Not<'de, T>
where
    T: Condition<'de> + Serialize + Deserialize<'de>,
{
    condition: T,
    #[serde(skip)]
    phantom: PhantomData<&'de T>,
}

impl<'de, T> Condition<'de> for Not<'de, T>
where
    T: Condition<'de> + Serialize + Deserialize<'de>,
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
    T: Condition<'de> + Serialize + Deserialize<'de>,
{
    left: T,
    right: T,
    #[serde(skip)]
    phantom: PhantomData<&'de T>,
}

impl<'de, T> Condition<'de> for And<'de, T>
where
    T: Condition<'de> + Serialize + Deserialize<'de>,
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
    T: Condition<'de> + Serialize + Deserialize<'de>,
{
    left: Box<T>,
    right: Box<T>,
    phantom: PhantomData<&'de T>,
}

impl<'de, T> Condition<'de> for Or<'de, T>
where
    T: Condition<'de> + Serialize + Deserialize<'de>,
{
    fn evaluate(&self) -> bool {
        self.left.evaluate() || self.right.evaluate()
    }
}

/// A condition that evaluates an inner function with data
#[derive(Serialize)]
pub struct Function<'de, D, F>
where
    F: Fn(&D) -> bool,
    D: Serialize + Deserialize<'de>,
{
    data: D,
    condition: F,
}

impl<'de, D, F> Condition<'de> for Function<'de, D, F>
where
    F: Fn(&D) -> bool,
{
    fn evaluate(&self) -> bool {
        (self.condition)(&self.data)
    }
}
