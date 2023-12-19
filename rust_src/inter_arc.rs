use std::{cell::RefCell, sync:: Arc};
use crate::Exploration::UnExplored;

enum Exploration {
    Explored, UnExplored, PartiallyExplored
}
struct Branch {
    label: String,
    left: Arc<RefCell<Maze>>,
    right: Arc<RefCell<Maze>>,
    status: Exploration
}
enum Maze {
    Branch(Branch),
    Leaf{label: String}
}
impl Maze {
    fn explore(&mut self, node:  Arc<RefCell<Maze>>, work: &mut Vec< Arc<RefCell<Maze>>>,trace: &mut Vec<String>) {
        match self {
            Maze::Branch(b) =>
                match b.status {
                    Exploration::UnExplored => {
                        b.status = Exploration::PartiallyExplored;
                        trace.push(b.label.clone());
                        work.push(node);
                        b.left.borrow_mut().explore( Arc::clone(&b.left),work, trace);
                    }
                    Exploration::PartiallyExplored => {
                        trace.push(b.label.clone());
                        b.status = Exploration::Explored;
                        b.right.borrow_mut().explore( Arc::clone(&b.right),work, trace);
                    }
                    Exploration::Explored => trace.push(b.label.clone())
                }
            Maze::Leaf{label} => trace.push(String::from(label.clone()))
        }
    }
}
fn main(){
    let leaf2 =  Arc::new(RefCell::new(Maze::Leaf{label: "2".to_string()}));
    let leaf4 = Arc::new(RefCell::new(Maze::Leaf{label: "4".to_string()}));
    let leaf5 = Arc::new(RefCell::new(Maze::Leaf{label: "5".to_string()}));
    let leaf8 = Arc::new(RefCell::new(Maze::Leaf{label: "8".to_string()}));
    let branch3 = Arc::new(RefCell::new(Maze::Branch(Branch{label: "3".to_string(), left: Arc::clone(&leaf4), right: Arc::clone(&leaf5), status: UnExplored})));
    let branch1 = Arc::new(RefCell::new(Maze::Branch(Branch{label: "1".to_string(), left: Arc::clone(&leaf2), right: Arc::clone(&branch3), status: UnExplored})));
    let branch7 = Arc::new(RefCell::new(Maze::Branch(Branch{label: "7".to_string(), left: Arc::clone(&leaf5), right: Arc::clone(&leaf8), status : UnExplored})));
    let branch6 = Arc::new(RefCell::new(Maze::Branch(Branch{label: "6".to_string(), left: Arc::clone(&branch3), right: Arc::clone(&branch7), status: UnExplored})));
    let branch0 = Arc::new(RefCell::new(Maze::Branch(Branch{label: "0".to_string(), left: Arc::clone(&branch1), right: Arc::clone(&branch6), status: UnExplored})));

    let mut work = vec! [Arc::clone(&branch0)];
    let mut trace = vec![];
    while work.len() != 0 {
        let node = work.pop().expect("unexpected");
        node.borrow_mut().explore (Arc::clone(&node),&mut work, &mut trace);
        println!("trace so far: {:?}", trace);
    }
}