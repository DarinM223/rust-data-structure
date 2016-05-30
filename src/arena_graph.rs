//! A graph implementation using a mutable
//! pointer container UnsafeCell with a TypedArena.
//!
//! It is safer than raw pointers because although the
//! UnsafeCell operations are unsafe, you won't have to
//! worry about use after free because the deque and all of
//! the nodes have the lifetime of the parent TypedArena.

use arena::TypedArena;
use std::cell::UnsafeCell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
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

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd)]
pub struct NodeState {
    id: i32,
    cost: i32,
}

impl Ord for NodeState {
    fn cmp(&self, other: &NodeState) -> Ordering {
        other.cost.cmp(&self.cost)
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

    pub fn dijkstra(&self, start: i32, end: i32) -> Vec<i32> {
        // ID of node -> best distance from start to the node
        let mut dist: HashMap<i32, i32> = HashMap::new();
        // ID of node -> previous node ID for best path
        let mut prev: HashMap<i32, Option<i32>> = HashMap::new();
        // Checks if node has already been visited
        let mut visited = HashMap::new();

        // Initialize distances of nodes to 'infinity' and the previous link to None
        self.bfs_map(|ref node| {
            dist.insert(node.id, i32::MAX);
            prev.insert(node.id, None);
        });

        let mut heap = BinaryHeap::new();
        heap.push(NodeState {
            id: start,
            cost: 0,
        });

        while let Some(state) = heap.pop() {
            visited.insert(state.id, true);

            // Ignore states which have more distance than the best distance for the path
            if state.cost > dist[&state.id] {
                continue;
            }

            let node = match self.id_map.get(&state.id) {
                Some(node) => *node,
                _ => return vec![],
            };

            for &(edge_dist, edge) in unsafe { &*node.edges.get() } {
                if !visited.contains_key(&edge.id) {
                    let alt = state.cost + edge_dist;
                    // If the state has less distance than the best distance, set the previous
                    // node and set the best distance to the new smallest distance
                    if alt < dist[&edge.id] {
                        *dist.get_mut(&edge.id).unwrap() = alt;
                        *prev.get_mut(&edge.id).unwrap() = Some(state.id);

                        heap.push(NodeState {
                            id: edge.id,
                            cost: alt,
                        });
                    }
                }
            }
        }

        // Build path vector at the end
        let mut path = VecDeque::new();
        let mut curr_id = end;
        path.push_front(end);

        while let Some(prev_id) = prev[&curr_id] {
            path.push_front(prev_id);
            curr_id = prev_id;
        }

        path.into_iter().collect()
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

    #[test]
    fn test_dijkstra() {
        let arena = TypedArena::new();
        let mut graph = Graph::new(2, &arena);

        let two_node = graph.root;
        let three_node = graph.add_node(3);
        let four_node = graph.add_node(4);
        let five_node = graph.add_node(5);

        graph.add_edge(two_node, three_node, 24);
        graph.add_edge(three_node, two_node, 24);

        graph.add_edge(three_node, four_node, 20);
        graph.add_edge(four_node, three_node, 20);

        graph.add_edge(three_node, five_node, 3);
        graph.add_edge(five_node, three_node, 3);

        graph.add_edge(four_node, five_node, 12);
        graph.add_edge(five_node, four_node, 12);

        assert_eq!(graph.dijkstra(three_node, two_node),
                   vec![three_node, two_node]);
        assert_eq!(graph.dijkstra(three_node, five_node),
                   vec![three_node, five_node]);
        assert_eq!(graph.dijkstra(three_node, four_node),
                   vec![three_node, five_node, four_node]);
    }
}
