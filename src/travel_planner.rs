use std::collections::{BinaryHeap, HashMap, HashSet};

type Node = usize;
type Cost = usize;

struct Graph {
    edges: HashMap<Node, Vec<(Node, Cost)>>,
    nodes: HashSet<Node>,
}

impl Graph {
    fn from_edge_list(edge_list: &Vec<(Node, Node, Cost)>) -> Self {
        let mut adjacency_list: HashMap<Node, Vec<(Node, Cost)>> = HashMap::new();
        let mut nodes = HashSet::new();

        for &(source, destination, cost) in edge_list.iter() {
            let destinations = adjacency_list.entry(source).or_insert_with(|| Vec::new());

            destinations.push((destination, cost));

            nodes.insert(source);
            nodes.insert(destination);
        }

        Graph {
            edges: adjacency_list,
            nodes,
        }
    }
}

#[derive(PartialEq, Eq)]
struct Step {
    cost: Cost,
    position: Node,
    history: Vec<Node>,
}

impl Step {}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse the cost ordering (so we can retrieve the step with minimum cost from the priority queue)
        // In the case of a tie, compare with
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

fn shortest_path(g: &Graph, start: Node, goal: Node) -> Option<(Vec<Node>, Cost)> {
    // Shortcut: If the start is the same as the goal, don't do anything
    if start == goal {
        return Some((vec![start], 0));
    }

    // This is an implementation of Dijkstra's algorithm (which can be used since edge costs are positive)
    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(Step {
        cost: 0,
        position: start,
        history: vec![start],
    });

    let mut distances: HashMap<Node, Cost> = g
        .nodes
        .iter()
        .map(|&n| if n == start { (n, 0) } else { (n, usize::MAX) })
        .collect();

    while let Some(Step {
        cost,
        position,
        history,
    }) = priority_queue.pop()
    {
        // If the next step is the goal, then we're done
        if position == goal {
            return Some((history, cost));
        }

        // Otherwise, process all edges coming out of this node
        if let Some(edges) = g.edges.get(&position) {
            for &(next_node, next_cost) in edges {
                if next_cost < distances[&next_node] {
                    let mut next_step = Step {
                        cost: cost + next_cost,
                        position: next_node,
                        history: history.clone(),
                    };

                    next_step.history.push(next_node);
                    priority_queue.push(next_step);
                    distances.insert(next_node, next_cost);
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_graph() {
        let edge_list = vec![(1, 2, 3), (1, 3, 6), (2, 3, 1), (4, 5, 1)];

        let g = Graph::from_edge_list(&edge_list);

        let path = shortest_path(&g, 1, 3);
        assert!(path.is_some());

        let (nodes, cost) = path.unwrap();
        assert_eq!(vec![1, 2, 3], nodes);
        assert_eq!(4, cost);
    }

    #[test]
    fn large_graph() {
        let edge_list = travel_planner_graph::get_large_graph();
        let g = Graph::from_edge_list(&edge_list);

        let path = shortest_path(&g, 1000, 9000);
        assert!(path.is_some());
        assert_eq!(path.unwrap().1, 24);
    }
}
