use serde::{Deserialize, Serialize};

/// A conditional weight that must evaluate as true in order to be avaliable to
/// make an edge clear to traverse.
#[allow(single_use_lifetimes)]
pub trait Condition: Serialize + for<'de> Deserialize<'de> {
    /// Returns whether the node can be traversed based on its criteria.
    fn evaluate(&self) -> bool;
}

/// A condition that always evaluates to true.
///
/// # Example use
/// ```
/// # use dialogue_graph::condition::{Condition, True};
/// let condition = True::new();
///
/// assert_eq!(condition.evaluate(), true);
/// ```
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct True {}

impl True {
    /// Create a new `True` condition.
    pub fn new() -> Self {
        Self {}
    }
}

impl Condition for True {
    fn evaluate(&self) -> bool {
        true
    }
}

/// A condition that evaluates as true if its inner condition evaluates as
/// false.
///
/// # Example use
/// ```
/// # use dialogue_graph::condition::{Condition, Not, True};
/// let not = Not::new(True::new());
///
/// assert_eq!(not.evaluate(), false);
/// ```
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
///
/// # Example use
/// ```
/// # use dialogue_graph::condition::{Condition, And, Not, True};
/// let first = Not::new(True::new());
/// let second = True::new();
/// let and = And::new(first, second);
///
/// assert_eq!(and.evaluate(), false);
/// ```
#[derive(Copy, Clone, Debug, Serialize, PartialEq, Eq)]
pub struct And<T, U>
where
    T: Condition,
    U: Condition,
{
    /// The first condition to evaluate on.
    pub left: T,
    /// The second condition to evaluate on.
    pub right: U,
}

impl<T, U> And<T, U>
where
    T: Condition,
    U: Condition,
{
    /// Create a new `And` condition.
    pub fn new(left: T, right: U) -> Self {
        Self { left, right }
    }
}

impl<T, U> Condition for And<T, U>
where
    T: Condition,
    U: Condition,
{
    fn evaluate(&self) -> bool {
        self.left.evaluate() && self.right.evaluate()
    }
}

/// A condition that evaluates as true if either inner condition evaluates as
/// true.
///
/// # Example use
/// ```
/// # use dialogue_graph::condition::{Condition, Function, True, Not, Or};
/// # use serde_closure::Fn;
/// let first = True::new();
/// let second = Not::new(True::new());
///
/// let or = Or::new(first, second);
///
/// assert_eq!(or.evaluate(), true);
/// ```
#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq)]
pub struct Or<T, U>
where
    T: Condition,
    U: Condition,
{
    /// The first condition to evaluate on.
    pub left: T,
    /// The second condition to evaluate on.
    pub right: U,
}

impl<T, U> Or<T, U>
where
    T: Condition,
    U: Condition,
{
    /// Create a new `Or` condition.
    pub fn new(left: T, right: U) -> Self {
        Self { left, right }
    }
}

impl<T, U> Condition for Or<T, U>
where
    T: Condition,
    U: Condition,
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
