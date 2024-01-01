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

            if !names.contains_key(start) {
                names.insert(start, names.len());
            }
            let a = *names.get(start).unwrap();

            for end in ends {
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
    fn minimum_cut(&self) -> (usize, usize) {
        let mut num_nodes = self.edges.iter().map(|e| e.b + 1).max().unwrap();

        let mut counts = vec![1usize; num_nodes];

        let mut edges = self.edges.iter().copied().collect_vec();

        while num_nodes > 2 {
            let r = random::<usize>() % edges.len();
            let edge = edges.get(r).unwrap();
            let a = edge.a;
            let b = edge.b;

            edges.iter_mut().for_each(|edge| {
                if edge.a == b {
                    edge.a = a;
                };
                if edge.b == b {
                    edge.b = a;
                };
            });
            edges.retain(|e| e.a != e.b);

            counts[a] += counts[b];
            counts[b] = 0;

            num_nodes -= 1;
        }

        let a = counts[edges.first().unwrap().a];
        let b = counts[edges.first().unwrap().b];

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
    let mut sum = 0;

    while size != 3 {
        (size, sum) = g.minimum_cut();
    }

    sum
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
