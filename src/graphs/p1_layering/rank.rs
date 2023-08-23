use std::{collections::HashMap, ops::Index};

use petgraph::stable_graph::{NodeIndex, StableDiGraph};

use crate::util::layers::Layers;

use super::tree::{TightTreeDFSs};

/// Ranks of the vertices of a graph.
/// Needs to contain all the vertices of a graph
#[derive(Debug, Clone)]
pub struct Ranks {
_inner: HashMap<NodeIndex, isize>,
minimum_length: usize
}

impl Ranks {
    pub fn new<T>(ranks: HashMap<NodeIndex, isize>, graph: &StableDiGraph<Option<T>, usize>, minimum_length: usize) -> Self {
        assert!(Self::is_valid(&ranks, graph));
        Ranks { _inner: ranks, minimum_length }
    }

    #[cfg(test)]
    pub fn new_unchecked(ranks: HashMap<NodeIndex, isize>, minimum_length: usize) -> Self {
        Ranks { _inner: ranks, minimum_length }
    }
    
    fn is_valid<T>(ranks: &HashMap<NodeIndex, isize>, graph: &StableDiGraph<Option<T>, usize>) -> bool {
        for v in graph.node_indices() {
            if !ranks.contains_key(&v) {
                return false;
            }
        } 

        true
    }

    // tail = predecessor, head = successor
    pub(super) fn slack(&self, tail: NodeIndex, head: NodeIndex) -> isize {
        self._inner.get(&head).unwrap() - self._inner.get(&tail).unwrap() - self.minimum_length as isize
    }

    pub(super) fn get_minimum_length(&self) -> usize {
        self.minimum_length
    }

    pub(super) fn update(&mut self, vertex: NodeIndex, delta: isize) {
        self._inner.entry(vertex).and_modify(|rank| *rank += delta);
    }

    pub(super) fn tighten_edge(&mut self, tree: &TightTreeDFSs, delta: isize) {
        for v in tree.vertices() {
            self.update(*v, delta);
        }
    }

    /// Normalize ranking so the least rank is 0
    pub fn normalize(&mut self) {
        let minimum_rank = *self._inner.iter().min_by(|(_, rank_a), (_, rank_b)| rank_a.cmp(&rank_b)).unwrap().1;
        for (_, rank) in self._inner.iter_mut() {
            *rank -= minimum_rank;
        }
    }

    /// Note: Ranks have to be normalized, or this will fail.
    pub(super) fn into_layers<T>(mut self, graph: &StableDiGraph<Option<T>, usize>) -> Layers {
        self.normalize();
        let mut layers = vec![];
        for (vertex, layer) in self._inner {
            while layers.len() <= layer as usize {
                layers.push(vec![]);
            }
            layers[layer as usize].push(vertex);
        
        }
        Layers::new(layers, graph)
    }
}

impl Index<NodeIndex> for Ranks {
    type Output = isize;

    fn index(&self, index: NodeIndex) -> &Self::Output {
        self._inner.get(&index).unwrap()
    }
}

#[cfg(test)]
pub mod tests {
    use std::collections::HashMap;

    use crate::graphs::p1_layering::tests::create_test_graph;

    use super::{Ranks, super::UnlayeredGraph};

    pub(crate) fn create_test_ranking_not_tight() -> Ranks {
        let ranks_raw = HashMap::from([
            (0.into(), 0),
            (1.into(), 1),
            (2.into(), 2),
            (3.into(), 3),
            (4.into(), 1),
            (5.into(), 1),
            (6.into(), 3),
            (7.into(), 4),
            (8.into(), 6),
        ]);
        Ranks{ _inner: ranks_raw, minimum_length: 1 }
    }

    #[test]
    fn test_initial_ranking() {
        let graph = create_test_graph::<isize>();
        let ul_graph = UnlayeredGraph { graph };
        let ranks = ul_graph.initial_ranking(1).ranks;
        assert_eq!(ranks._inner.get(&0.into()), Some(&0));
        assert_eq!(ranks._inner.get(&1.into()), Some(&1));
        assert_eq!(ranks._inner.get(&2.into()), Some(&2));
        assert_eq!(ranks._inner.get(&3.into()), Some(&3));
        assert_eq!(ranks._inner.get(&4.into()), Some(&2));
        assert_eq!(ranks._inner.get(&5.into()), Some(&2));
        assert_eq!(ranks._inner.get(&6.into()), Some(&3));
        assert_eq!(ranks._inner.get(&7.into()), Some(&4));

        dbg!(&ranks);
    }

    #[test]
    fn test_is_valid() {
        let graph = create_test_graph::<isize>();
        let ranks_raw = HashMap::from([
            (0.into(), 0),
            (1.into(), 0),
            (2.into(), 0),
            (3.into(), 0),
            (4.into(), 0),
            (5.into(), 0),
            (6.into(), 0),
            (7.into(), 0),
        ]);

        assert!(Ranks::is_valid(&ranks_raw, &graph));
    }

    #[test]
    fn test_is_not_valid() {
        let graph = create_test_graph::<isize>();
        let ranks_raw = HashMap::from([
            (0.into(), 0),
            (1.into(), 0),
            (2.into(), 0),
            (3.into(), 0),
            (5.into(), 0),
            (6.into(), 0),
            (7.into(), 0),
        ]);

        assert!(!Ranks::is_valid(&ranks_raw, &graph));
    }
}
