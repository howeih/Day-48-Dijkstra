use std::collections::HashMap;
use std::collections::HashSet;
use std::i32;

#[derive(Debug)]
struct Dijkstra {
    visited: HashSet<char>,
    distance_to_start: HashMap<char, i32>,
    graph: HashMap<char, HashMap<char, i32>>,
}
impl Dijkstra {
    fn new() -> Self {
        Self {
            visited: HashSet::<char>::new(),
            distance_to_start: HashMap::<char, i32>::new(),
            graph: HashMap::<char, HashMap<char, i32>>::new(),
        }
    }
    fn add(&mut self, from: char, to: char, dist: i32) {
        let from_entry = self
            .graph
            .entry(from)
            .or_insert(HashMap::<char, i32>::new());
        from_entry.insert(to, dist);
        let to_entry = self.graph.entry(to).or_insert(HashMap::<char, i32>::new());
        to_entry.insert(from, dist);
    }

    fn finish(&self, end: &char) -> bool {
        if self.visited.contains(end) {
            true
        } else {
            false
        }
    }

    fn find_closest(&mut self, current: &char) -> char {
        let dist_map = self.graph.get(current).unwrap();
        let mut closet = *current;
        let mut min_dist = i32::MAX;
        let start_to_current = { *self.distance_to_start.entry(*current).or_insert(i32::MAX) };
        for (n, d) in dist_map {
            if self.visited.contains(n) {
                continue;
            }
            let mut start_to_n = { *self.distance_to_start.entry(*n).or_insert(i32::MAX) };
            if (*d + start_to_current) < start_to_n {
                start_to_n = *d + start_to_current;

                self.distance_to_start.insert(*n, start_to_n);
            }
        }
        for (n, d) in &self.distance_to_start {
            if *d < min_dist && !self.visited.contains(n) {
                min_dist = *d;
                closet = *n;
            }
        }
        closet
    }

    fn dijkstra(&mut self, start: char, end: char) -> i32 {
        let mut current = start;
        self.visited.insert(start);
        self.distance_to_start.insert(start, 0);
        while !self.finish(&end) {
            let closest_point = self.find_closest(&current);
            self.visited.insert(closest_point);
            current = closest_point;
        }
        *self.distance_to_start.get(&end).unwrap()
    }
}
fn main() {
    let mut graph = Dijkstra::new();
    graph.add('A', 'B', 4);
    graph.add('A', 'C', 3);
    graph.add('A', 'E', 7);
    graph.add('B', 'C', 6);
    graph.add('B', 'D', 5);
    graph.add('C', 'D', 11);
    graph.add('C', 'E', 8);
    graph.add('D', 'E', 2);
    graph.add('D', 'G', 10);
    graph.add('D', 'F', 2);
    graph.add('E', 'G', 5);
    graph.add('F', 'G', 3);
    assert_eq!(graph.dijkstra('A', 'F'), 11);
}
