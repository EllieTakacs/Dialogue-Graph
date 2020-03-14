use condition::Condition;
use petgraph::{
    graph::{Edges, Graph, NodeIndex},
    Directed,
};
#[allow(unused_imports)]
use rand::seq::IteratorRandom;
use serde::{Deserialize, Serialize};

/// Conditions
pub mod condition;
mod serde_impls;

/// A single unit of dialogue which makes up the `DialogueGraph`.
type Node = String;

/// A directed connection between two `Node` objects, with a condition that
/// predicates the ability to traverse it.
#[derive(Serialize, Debug)]
pub struct Edge<T>
where
    for<'de> T: Condition<'de> + Serialize + Deserialize<'de>,
{
    /// The condition for traversing.
    pub condition: T,
}

/// A dialogue graph consisting of dialogue nodes connected by conditional
/// edges.
#[derive(Serialize, Debug)]
pub struct DialogueGraph<T>
where
    for<'de> T: Condition<'de> + Serialize + Deserialize<'de>,
{
    /// The underlying graph.
    pub data: Graph<Node, Edge<T>, Directed, u32>,
}

impl<T> DialogueGraph<T>
where
    for<'de> T: Condition<'de> + Serialize + Deserialize<'de>,
{
    /// Create a new DialogueGraph instance.
    pub fn new() -> Self {
        Self { data: Graph::new() }
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
    for<'de> T: Condition<'de> + Serialize + Deserialize<'de>,
{
    edges: Edges<'a, Edge<T>, Directed>,
}

impl<'a, T> Iterator for OpenEdges<'a, T>
where
    for<'de> T: Condition<'de> + Serialize + Deserialize<'de>,
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
