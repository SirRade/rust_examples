trait VerboseGraph<N, E> {
    fn has_edge(&self, &N, &N) -> bool;
    fn edges(&self, &N) -> Vec<E>;
    // Etc.
}
// Too verbose!
fn verbose_distance<N, E, G: VerboseGraph<N, E>>(graph: &G, start: &N, end: &N) -> u32 { 42 }

trait Graph {
    type N;
    type E;

    fn has_edge(&self, &Self::N, &Self::N) -> bool;
    fn edges(&self, &Self::N) -> Vec<Self::E>;
    // Etc.
}
fn distance<G: Graph>(graph: &G, start: &G::N, end: &G::N) -> u32 { 42 }

// Associated types can also have requirements
trait Foo {
    type T: std::fmt::Display; 
}


// Implementing a trait with associated types
struct Node;
struct Edge;
struct MyGraph;
impl Graph for MyGraph {
    type N = Node;
    type E = Edge;
    fn has_edge(&self, n1: &Node, n2: &Node) -> bool {
        true
    }
    fn edges(&self, n: &Node) -> Vec<Edge> {
        Vec::new()
    }
}

fn main() {
    let graph = MyGraph;
    // Doesn't work
    // Trait object doesn't know which impl to use
    // let obj = Box::new(graph) as Box<Graph>;
    let obj = Box::new(graph) as Box<Graph<N=Node, E=Edge>>;
}
