use condition::Condition;
use petgraph::{
    graph::{Edges, Graph, NodeIndex},
    Directed,
};
#[allow(unused_imports)]
use rand::seq::IteratorRandom;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

pub mod condition;
mod serde_impls;

#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub struct Node {
    pub content: String,
}

/// A directed connection between two `Node` objects, with a condition that
/// predicates the ability to traverse it.
#[derive(Serialize)]
pub struct Edge<'de, T>
where
    T: Condition<'de> + Serialize + Deserialize<'de>,
{
    pub condition: T,
    #[serde(skip)]
    phantom: PhantomData<&'de T>,
}

/// A dialogue graph consisting of dialogue nodes connected by conditional
/// edges.
#[derive(Serialize)]
pub struct DialogueGraph<'a, 'de, T>
where
    T: Condition<'de> + Serialize + Deserialize<'de>,
{
    pub data: Graph<Node, Edge<'de, T>, Directed, u32>,
    #[serde(skip)]
    phantom: PhantomData<&'a T>,
}

impl<'a, 'de, T> DialogueGraph<'a, 'de, T>
where
    T: Condition<'de> + Serialize + Deserialize<'de>,
{
    pub fn new() -> Self {
        Self {
            data: Graph::new(),
            phantom: PhantomData,
        }
    }

    /// Evaluates the condition to determine whether the edge can be traversed.
    pub fn open(edge: Edge<'de, T>) -> bool {
        edge.condition.evaluate()
    }

    /// Opposite of `open`, evaluates the condition to determine whether the
    /// edge is closed off.
    pub fn closed(edge: Edge<'de, T>) -> bool {
        !DialogueGraph::open(edge)
    }

    pub fn open_edges(&'a self, node: NodeIndex) -> OpenEdges<'a, 'de, T> {
        OpenEdges {
            edges: self.data.edges(node),
        }
    }
}

/// Iterator over open edges leading out of a given node.
pub struct OpenEdges<'a, 'de, T>
where
    T: Condition<'de> + Serialize + Deserialize<'de>,
{
    edges: Edges<'a, Edge<'de, T>, Directed>,
}

impl<'a, 'de, T> Iterator for OpenEdges<'a, 'de, T>
where
    T: Condition<'de> + Serialize + Deserialize<'de>,
{
    type Item = &'a Edge<'de, T>;
    fn next(&mut self) -> Option<&'a Edge<'de, T>> {
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
