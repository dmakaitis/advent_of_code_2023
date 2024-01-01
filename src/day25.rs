use itertools::Itertools;
use rand::random;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Edge {
    a: usize,
    b: usize,
}

impl Edge {
    fn new(a: usize, b: usize) -> Self {
        // Ensures nodes for an edge are always in a consistent order so when we place them into a set while we're
        // parsing the input, we're guaranteed not to get duplicate entries...
        if a < b {
            Edge { a, b }
        } else {
            Edge { a: b, b: a }
        }
    }
}

struct Graph {
    edges: HashSet<Edge>,
}

impl From<&str> for Graph {
    fn from(value: &str) -> Self {
        let mut names = HashMap::new();
        let mut edges = HashSet::new();

        for line in value.lines() {
            let (start, ends) = line.split(':').collect_tuple().unwrap();

            let ends = ends.split_whitespace().collect_vec();

            // As we encounter each node name for the first time, add it to a hash map so we can assign it a unique
            // integer value that we can easily look up the next time we encounter the same name...
            if !names.contains_key(start) {
                names.insert(start, names.len());
            }
            let a = *names.get(start).unwrap();

            for end in ends {
                // Again, assign a unique integer value for each new node name we encounter...
                if !names.contains_key(end) {
                    names.insert(end, names.len());
                }
                let b = *names.get(end).unwrap();

                edges.insert(Edge::new(a, b));
            }
        }

        Graph { edges }
    }
}

impl Graph {
    fn contract(&self) -> (usize, usize) {
        // This is an implementation of single iteration of contraction for Karger's algorithm.

        // Since we assigned consecutive integer values to each node in the graph while parsing, the
        // number of nodes in the graph is equal to the highest node ID plus one
        let mut num_nodes = self.edges.iter().map(|e| e.b + 1).max().unwrap();

        // As we collapse the graph, we'll keep track of the number of initial nodes that have been merged into any
        // remaining nodes. We'll use this to calculate the puzzle output at the end when only two nodes remain.
        // The initial value for each node is 1 since at the beginning, each node only represents itself.
        let mut counts = vec![1usize; num_nodes];

        let mut edges = self.edges.iter().copied().collect_vec();

        // We'll continue to collapse edges until only two nodes remain.
        while num_nodes > 2 {
            // Pick a random edge to collapse and get the two node ID's for the edge.
            // (Karger's algorithm works because there are far more edges NOT in the minimal cut, so odds are we'll
            // select an edge not on the minimal cut)
            let r = random::<usize>() % edges.len();
            let edge = edges.get(r).unwrap();
            let a = edge.a;
            let b = edge.b;

            // Go through all the edges and replace any references to node B with node A since we are merging the two
            // nodes together and only need to keep one.
            edges.iter_mut().for_each(|edge| {
                if edge.a == b {
                    edge.a = a;
                };
                if edge.b == b {
                    edge.b = a;
                };
            });
            // Remove any edges where both nodes are the same - those are the edges we just collapsed
            // (there may be more than one since as we continue to collapse the graph, we may end up with multiple
            // edges connecting any two nodes)
            edges.retain(|e| e.a != e.b);

            // Transfer the node counts from node B into node A since we just merged them both together.
            counts[a] += counts[b];
            counts[b] = 0;

            num_nodes -= 1;
        }

        // With only two nodes left, all the remaining edges must connect those two nodes, so we can get the two
        // node ID's from any remaining edge (like the first one).

        // Get the total counts for the two remaining nodes:
        let a = counts[edges.first().unwrap().a];
        let b = counts[edges.first().unwrap().b];

        // Return the number of edges that remain, along with the product of how many nodes had been merged into either
        // end of those edges:
        (edges.len(), a * b)
    }
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_one(input: &str) -> usize {
    let g = Graph::from(input);

    let mut size = usize::MAX;
    let mut product = 0;

    // Theoretically, we can find the minimal cut of the graph by iterating through Karger's algorithm N * ln(N)
    // times (where N is the number of nodes in the graph) and keeping the the result with the lowest number of
    // remaining edges. However, we know from the puzzle description that the minimal cut will have three edges, so
    // we only have to keep repeating until we find a solution that collapses into only three edges. This saves us from
    // having to do approximately 15,000 iterations when in practice we can find a solution in only a few hundred
    // iterations...
    while size != 3 {
        (size, product) = g.contract();
    }

    product
}

///
///
/// #Argument
///
/// 'input' - The input.
pub fn part_two(_: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use crate::day25::*;

    #[test]
    fn part_one_correct() {
        assert_eq!(
            part_one(
                "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr"
            ),
            54
        );
    }
}
