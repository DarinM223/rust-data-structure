//! A graph implementation using a mutable
//! pointer container UnsafeCell with a TypedArena.
//!
//! It is safer than raw pointers because although the
//! UnsafeCell operations are unsafe, you won't have to
//! worry about use after free because the deque and all of
//! the nodes have the lifetime of the parent TypedArena.

use arena::TypedArena;
use std::cell::UnsafeCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::i32;

pub struct Node<'a, T: 'a> {
    id: i32,
    data: T,
    edges: UnsafeCell<Vec<(i32, &'a Node<'a, T>)>>,
}

impl<'a, T> Node<'a, T> {
    pub fn new<'b>(id: i32, data: T, arena: &'b TypedArena<Node<'b, T>>) -> &'b Node<'b, T> {
        arena.alloc(Node {
            id: id,
            data: data,
            edges: UnsafeCell::new(Vec::new()),
        })
    }
}

pub struct Graph<'a, T: 'a> {
    pub root: i32,
    arena: &'a TypedArena<Node<'a, T>>,
    id_map: HashMap<i32, &'a Node<'a, T>>,
    curr_id: i32,
}

impl<'a, T: Clone> Graph<'a, T> {
    pub fn new(data: T, arena: &'a TypedArena<Node<'a, T>>) -> Graph<'a, T> {
        let mut id_map = HashMap::new();
        id_map.insert(0, Node::new(0, data, arena));

        Graph {
            arena: arena,
            id_map: id_map,
            root: 0,
            curr_id: 1,
        }
    }

    pub fn add_node(&mut self, data: T) -> i32 {
        let node_id = self.curr_id;
        self.curr_id += 1;

        self.id_map.insert(node_id, Node::new(node_id, data, self.arena));
        node_id
    }

    pub fn add_edge(&self, from_id: i32, to_id: i32, cost: i32) {
        if let (Some(from), Some(to)) = (self.id_map.get(&from_id), self.id_map.get(&to_id)) {
            unsafe {
                (*from.edges.get()).push((cost, *to));
            }
        }
    }

    pub fn set_root(&mut self, id: i32) {
        self.root = id;
    }

    pub fn bfs_map<U, F>(&self, mut func: F)
        where F: FnMut(&Node<'a, T>) -> U
    {
        let mut queue = VecDeque::new();
        let mut explored_nodes = HashSet::new();
        match self.id_map.get(&self.root) {
            Some(node) => queue.push_back(*node),
            _ => return,
        }

        while let Some(node) = queue.pop_front() {
            func(&node);
            explored_nodes.insert(node.id);

            for &(_, edge) in unsafe { &*node.edges.get() } {
                if !explored_nodes.contains(&edge.id) {
                    queue.push_back(edge);
                }
            }
        }
    }

    pub fn dfs_map<U, F>(&self, mut func: F)
        where F: FnMut(&Node<'a, T>) -> U
    {
        let mut stack = Vec::new();
        let mut explored_nodes = HashSet::new();
        match self.id_map.get(&self.root) {
            Some(node) => stack.push(*node),
            _ => return,
        }

        while let Some(node) = stack.pop() {
            func(&node);
            explored_nodes.insert(node.id);

            for &(_, edge) in unsafe { &*node.edges.get() } {
                if !explored_nodes.contains(&edge.id) {
                    stack.push(edge);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use arena::TypedArena;
    use super::*;

    #[test]
    fn test_bfs_map() {
        let arena = TypedArena::new();
        let mut graph = Graph::new(2, &arena);

        let three_node = graph.add_node(3);
        let four_node = graph.add_node(4);
        let five_node = graph.add_node(5);

        graph.add_edge(graph.root, three_node, 0);
        graph.add_edge(graph.root, five_node, 0);
        graph.add_edge(three_node, graph.root, 0);
        graph.add_edge(three_node, four_node, 0);
        graph.add_edge(four_node, five_node, 0);

        let mut results = Vec::new();
        graph.bfs_map(|ref node| results.push(node.data.clone()));

        assert_eq!(results, vec![2, 3, 5, 4]);
    }

    #[test]
    fn test_dfs_map() {
        let arena = TypedArena::new();
        let mut graph = Graph::new(2, &arena);

        let three_node = graph.add_node(3);
        let four_node = graph.add_node(4);
        let five_node = graph.add_node(5);
        let six_node = graph.add_node(6);

        graph.add_edge(graph.root, three_node, 0);
        graph.add_edge(graph.root, five_node, 0);
        graph.add_edge(three_node, graph.root, 0);
        graph.add_edge(three_node, four_node, 0);
        graph.add_edge(four_node, five_node, 0);
        graph.add_edge(five_node, six_node, 0);

        let mut results = Vec::new();
        graph.dfs_map(|ref node| results.push(node.data.clone()));

        assert_eq!(results, vec![2, 5, 6, 3, 4]);
    }
}
