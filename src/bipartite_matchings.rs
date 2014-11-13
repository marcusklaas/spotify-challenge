use std::collections::TreeSet;

type Vertex = uint;
type Edge = (Vertex, Vertex);
type EdgeSet = TreeSet<Edge>;

struct Path {
    starting_row: Vertex,
    trace: Vec<Edge>
}

impl Path {
    fn new(start: Vertex) -> Path {
        Path {
            starting_row: start,
            trace: Vec::new()
        }
    }

    fn get_current(&self) -> Vertex {
        let path_length = self.trace.len();
    
        match path_length > 0 {
            true  => {
                let (row, col) = self.trace[path_length - 1];
                
                match self.is_odd_length() {
                    true  => col,
                    false => row
                }
            },
            false => self.starting_row
        }
    }
    
    fn is_odd_length(&self) -> bool {
        self.trace.len() % 2 == 1
    }
    
    fn has_edge(&self, edge: Edge) -> bool {
        self.trace.iter().find(|&x| *x == edge).is_some()
    }
    
    fn add_edge(&mut self, edge: Edge) {
        self.trace.push(edge);
    }
    
    fn remove_edge(&mut self) {
        self.trace.pop();
    }
    
    fn get_edge_set(&self) -> EdgeSet {
        self.trace.iter().map(|&x| x).collect()
    }
}

pub struct BipartiteGraph {
    rows: uint,
    columns: uint,
    incidence_matrix: Vec<bool>
}

impl BipartiteGraph {
    pub fn from_closure<R, C, T: Iterator<R>, U: Iterator<C>>(rows: &mut T, columns: &mut U, closure: |&R, &C| -> bool) -> BipartiteGraph {
        let mut vec: Vec<bool> = Vec::new();
        let row_set: Vec<R> = rows.collect();
        let column_set: Vec<C> = columns.collect();
        
        for row in row_set.iter() {
            for col in column_set.iter() {
                vec.push(closure(row, col));
            }
        }
        
        BipartiteGraph {
            rows: row_set.len(),
            columns: column_set.len(),
            incidence_matrix: vec
        }
    }
    
    pub fn get_max_matching_size(&self) -> uint {
        let empty_matching: EdgeSet = TreeSet::new();
        
        self.max_matching_size(&empty_matching)
    }
    
    fn max_matching_size(&self, matching: &EdgeSet) -> uint {    
        match self.get_augmenting_path(matching) {
            None       => matching.len(),
            Some(path) => {
                let new_matching: EdgeSet = matching.symmetric_difference(&path)
                  .map(|&x| x)
                  .collect();
            
                self.max_matching_size(&new_matching)
            }
        }
    }
    
    fn get_augmenting_path(&self, matching: &EdgeSet) -> Option<EdgeSet> {
        self.get_unmatched_rows(matching).iter()
          .map(|&row| self.try_augmenting_path(matching, row))
          .find(|x| x.is_some())
          .unwrap_or(None)
    }
    
    fn try_augmenting_path(&self, matching: &EdgeSet, row: Vertex) -> Option<EdgeSet> {
        let mut path = Path::new(row);
        
        self.continue_search(matching, &mut path)
    }
    
    fn continue_search(&self, matching: &EdgeSet, path: &mut Path) -> Option<EdgeSet> {
        let current = path.get_current();
        let is_column = path.is_odd_length();
        
        if is_column && matching.iter().map(|&(_, col)| col).find(|&x| x == current).is_none() {
            return Some(path.get_edge_set());
        }
        
        let mut eligible_edges: Vec<Edge> = self.get_edges(current, is_column);
        eligible_edges.retain(|x| !path.has_edge(*x) && matching.contains(x) == is_column);
        
        for &edge in eligible_edges.iter() {
            path.add_edge(edge);
            
            match self.continue_search(matching, path) {
                Some(new_path) => { return Some(new_path); },
                None           => { path.remove_edge(); }
            }
        }
    
        None
    }
    
    fn get_unmatched_rows(&self, matching: &EdgeSet) -> Vec<Vertex> {
        range(0, self.rows)
          .filter(|&x| !self.is_row_matched(matching, x))
          .collect()
    }
    
    fn is_row_matched(&self, matching: &EdgeSet, row: Vertex) -> bool {
        range(0, self.columns)
          .map(|col| (row, col))
          .any(|x| matching.contains(&x))
    }
    
    fn get_edges(&self, node: Vertex, is_column: bool) -> Vec<Edge> {
        match is_column {
            true  => range(0, self.rows).map(|x| (x, node)),
            false => range(0, self.columns).map(|x| (node, x))
        }
          .filter(|&(row, col)| self.incidence_matrix[row * self.columns + col])
          .collect()
    }
}
