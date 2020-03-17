use serde::{Deserialize, Serialize};

/// A conditional weight that must evaluate as true in order to be avaliable to
/// make an edge clear to traverse.
#[allow(single_use_lifetimes)]
pub trait Condition: Serialize + for<'de> Deserialize<'de> {
    /// Returns whether the node can be traversed based on its criteria.
    fn evaluate(&self) -> bool;
}

/// A condition that always evaluates to true.
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct True {}

impl Condition for True {
    fn evaluate(&self) -> bool {
        true
    }
}

/// A condition that evaluates as true if its inner condition evaluates as
/// false.
#[derive(Copy, Clone, Debug, Serialize, PartialEq, Eq)]
pub struct Not<T>
where
    T: Condition,
{
    /// The condition to evaluate on.
    pub condition: T,
}

impl<T> Not<T>
where
    T: Condition,
{
    /// Create a new `Not` condition.
    pub fn new(condition: T) -> Self {
        Self { condition }
    }
}

impl<T> Condition for Not<T>
where
    T: Condition,
{
    fn evaluate(&self) -> bool {
        !self.condition.evaluate()
    }
}

/// A condition that evaluates as true if both inner conditions evaluate as
/// true.
#[derive(Copy, Clone, Debug, Serialize, PartialEq, Eq)]
pub struct And<T>
where
    T: Condition,
{
    /// The first condition to evaluate on.
    pub left: T,
    /// The second condition to evaluate on.
    pub right: T,
}

impl<T> And<T>
where
    T: Condition,
{
    /// Create a new `And` condition.
    pub fn new(left: T, right: T) -> Self {
        Self { left, right }
    }
}

impl<T> Condition for And<T>
where
    T: Condition,
{
    fn evaluate(&self) -> bool {
        self.left.evaluate() && self.right.evaluate()
    }
}

/// A condition that evaluates as true if either inner condition evaluates as
/// true.
#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq)]
pub struct Or<T>
where
    T: Condition,
{
    /// The first condition to evaluate on.
    pub left: T,
    /// The second condition to evaluate on.
    pub right: T,
}

impl<T> Or<T>
where
    T: Condition,
{
    /// Create a new `Or` condition.
    pub fn new(left: T, right: T) -> Self {
        Self { left, right }
    }
}

impl<T> Condition for Or<T>
where
    T: Condition,
{
    fn evaluate(&self) -> bool {
        self.left.evaluate() || self.right.evaluate()
    }
}

/// A condition that evaluates an inner function with data.
///
/// To create a closure that implements serde's [`Serialize`] and
/// [`Deserialize`] traits, you can use the
/// [`serde_closure` crate](https://docs.rs/serde_closure).
///
/// # Example use
/// ```
/// # use dialogue_graph::condition::{Condition, Function};
/// # use serde_closure::Fn;
/// let data = 1;
/// let closure = Fn!(|x: &i32| *x > 0);
///
/// let function = Function::new(data, closure);
/// assert!(function.evaluate());
/// ```
#[derive(Copy, Clone, Debug, Serialize, PartialEq, Eq)]
#[allow(single_use_lifetimes)]
pub struct Function<T, U>
where
    T: Serialize + for<'de> Deserialize<'de>,
    U: Fn(&T) -> bool + Serialize + for<'de> Deserialize<'de>,
{
    /// The data to pass to the closure.
    pub data: T,
    /// The closure with which to evaluate the data.
    pub condition: U,
}

#[allow(single_use_lifetimes)]
impl<T, U> Function<T, U>
where
    T: Serialize + for<'de> Deserialize<'de>,
    U: Fn(&T) -> bool + Serialize + for<'de> Deserialize<'de>,
{
    /// Create a new `Function` condition.
    pub fn new(data: T, condition: U) -> Self {
        Self { data, condition }
    }
}

#[allow(single_use_lifetimes)]
impl<T, U> Condition for Function<T, U>
where
    T: Serialize + for<'de> Deserialize<'de>,
    U: Fn(&T) -> bool + Serialize + for<'de> Deserialize<'de>,
{
    fn evaluate(&self) -> bool {
        (self.condition)(&self.data)
    }
}
