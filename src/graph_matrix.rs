use std::vec;

#[derive(Debug)]
struct Vertex<'a> {
    id: usize,
    name: &'a str,
}

impl Vertex<'_> {
    fn new(id: usize, name: &'static str) -> Self {
        Self {
            id,
            name,
        }
    }
}

#[derive(Debug, Clone)]
struct Edge {
    edge: bool,
}

impl Edge {
    fn new() -> Self {
        Self {
            edge: false,
        }
    }

    fn set_edge() -> Self {
        Edge {
            edge: true,
        }
    }
}


//Graph
#[derive(Debug)]
struct Graph {
    nodes: usize,
    graph: Vec<Vec<Edge>>,
}

impl Graph {
    fn new(nodes: usize) -> Self {
        Self {
            nodes,
            graph: vec![vec![Edge::new(); nodes]; nodes],
        }
    }

    fn is_empty(&self) -> bool { 0 == self.nodes }

    fn len(&self) -> usize { self.nodes }

    fn add_edge(&mut self, n1: &Vertex, n2: &Vertex) {
        if n1.id < self.nodes && n2.id < self.nodes {
            self.graph[n1.id][n2.id] = Edge::set_edge();
        } else {
            println!("Error, vertex beyond the graph");
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test1() {
        let mut g = Graph::new(4);
        let n1 = Vertex::new(0, "A");
        let n2 = Vertex::new(1, "B");
        let n3 = Vertex::new(2, "C");
        let n4 = Vertex::new(3, "D");
        g.add_edge(&n1, &n2);
        g.add_edge(&n1, &n3);
        g.add_edge(&n2, &n3);
        g.add_edge(&n2, &n4);
        g.add_edge(&n3, &n4);
        g.add_edge(&n3, &n1);
        println!("{:#?}", g);
        println!("Graph is empty: {}", g.is_empty());
        println!("Graph length: {}", g.len());
    }
}