//! Dialogue graph
#![warn(
    anonymous_parameters,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_qualifications,
    variant_size_differences
)]

use condition::Condition;
use petgraph::{
    graph::{Edges, Graph, NodeIndex},
    Directed,
};
#[allow(unused_imports)]
use rand::seq::IteratorRandom;
use serde::Serialize;

/// Conditions
pub mod condition;
mod serde_impls;

/// A single unit of dialogue which makes up the `DialogueGraph`.
pub type Node = String;

/// A directed connection between two `Node` objects, with a condition that
/// predicates the ability to traverse it.
#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct Edge<T>
where
    T: Condition,
{
    /// The condition for traversing.
    pub condition: T,
}

impl<T> Edge<T>
where
    T: Condition,
{
    /// Create a new edge.
    pub fn new(condition: T) -> Self {
        Self { condition }
    }
}

/// A dialogue graph consisting of dialogue nodes connected by conditional
/// edges.
#[derive(Serialize, Debug)]
pub struct DialogueGraph<T>
where
    T: Condition,
{
    /// The underlying graph.
    pub data: Graph<Node, Edge<T>, Directed, u32>,
}

impl<T> PartialEq for DialogueGraph<T>
where
    T: Condition + PartialEq,
{
    /// Compare the two graphs' nodes and edges.
    ///
    /// Algorithm taken from [this comment on GitHub](https://github.com/petgraph/petgraph/issues/199#issuecomment-484077775).
    fn eq(&self, other: &DialogueGraph<T>) -> bool {
        let a_ns = self.data.raw_nodes().iter().map(|n| &n.weight);
        let b_ns = other.data.raw_nodes().iter().map(|n| &n.weight);
        let a_es = self
            .data
            .raw_edges()
            .iter()
            .map(|e| (e.source(), e.target(), &e.weight));
        let b_es = other
            .data
            .raw_edges()
            .iter()
            .map(|e| (e.source(), e.target(), &e.weight));
        a_ns.eq(b_ns) && a_es.eq(b_es)
    }
}

impl<T> DialogueGraph<T>
where
    T: Condition,
{
    /// Create a new DialogueGraph instance.
    pub fn new() -> Self {
        Self {
            data: Graph::<Node, Edge<T>>::new(),
        }
    }

    /// Evaluates the condition to determine whether the edge can be traversed.
    pub fn open(edge: Edge<T>) -> bool {
        edge.condition.evaluate()
    }

    /// Opposite of `open`, evaluates the condition to determine whether the
    /// edge is closed off.
    pub fn closed(edge: Edge<T>) -> bool {
        !DialogueGraph::open(edge)
    }

    /// Returns an iterator over the open edges connected to the given node
    /// index.
    pub fn open_edges<'a, 'de>(&'a self, node: NodeIndex) -> OpenEdges<'a, T> {
        OpenEdges {
            edges: self.data.edges(node),
        }
    }
}

/// Iterator over open edges leading out of a given node.
#[allow(missing_debug_implementations)]
pub struct OpenEdges<'a, T>
where
    T: Condition,
{
    edges: Edges<'a, Edge<T>, Directed>,
}

impl<'a, T> Iterator for OpenEdges<'a, T>
where
    T: Condition,
{
    type Item = &'a Edge<T>;
    fn next(&mut self) -> Option<&'a Edge<T>> {
        let mut result = None;
        while let Some(edge) = self.edges.next() {
            if edge.weight().condition.evaluate() {
                result = Some(edge.weight());
                break;
            }
        }
        result
    }
}
