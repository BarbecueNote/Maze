use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

#[derive(PartialEq)]
enum Exploration {
    Explored,
    PartialExplored,
    UnExplored,
}

struct Branch {
    label: String,
    left: Arc<Mutex<Maze>>,
    right: Arc<Mutex<Maze>>,
    status: Exploration,
}

impl Branch {
    fn from(label: String, left: &Arc<Mutex<Maze>>, right: &Arc<Mutex<Maze>>) -> Arc<Mutex<Maze>> {
        Arc::new(Mutex::new(Maze::Branch(Branch {
            label,
            left: Arc::clone(left),
            right: Arc::clone(right),
            status: Exploration::UnExplored,
        })))
    }
}

struct Leaf(String);

impl Leaf {
    fn from(label: String) -> Arc<Mutex<Maze>> {
        Arc::new(Mutex::new(Maze::Leaf(label)))
    }
}

enum Maze {
    Branch(Branch),
    Leaf(String),
}

impl Maze {
    fn explore(
        &mut self,
        node: Arc<Mutex<Maze>>,
        list: &mut Vec<String>,
        work: &mut Arc<Mutex<Vec<Arc<Mutex<Maze>>>>>,
        counter: &mut Arc<Mutex<i32>>,
    ) {
        match self {
            Maze::Branch(b) => {
                if b.status == Exploration::UnExplored {
                    b.status = Exploration::PartialExplored;
                    work.lock().unwrap().push(node);
                    list.push(b.label.clone());
                    b.left
                        .lock()
                        .unwrap()
                        .explore(Arc::clone(&b.left), list, work, counter);
                } else if b.status == Exploration::PartialExplored {
                    b.status = Exploration::Explored;
                    b.right
                        .lock()
                        .unwrap()
                        .explore(Arc::clone(&b.right), list, work, counter);
                } else {
                    list.push(b.label.clone());
                    let mut num = counter.lock().unwrap();
                    *num -= 1;
                }
            }
            Maze::Leaf(l) => {
                list.push(l.clone());
                let mut num = counter.lock().unwrap();
                *num -= 1;
            }
        }
    }
}
fn main() {
    let leaf2 = Leaf::from(String::from("2"));
    let leaf4 = Leaf::from(String::from("4"));
    let leaf5 = Leaf::from(String::from("5"));
    let leaf8 = Leaf::from(String::from("8"));
    let branch3 = Branch::from(String::from("3"), &leaf4, &leaf5);
    let branch1 = Branch::from(String::from("1"), &leaf2, &branch3);
    let branch7 = Branch::from(String::from("7"), &leaf5, &leaf8);
    let branch6 = Branch::from(String::from("6"), &branch3, &branch7);
    let branch0 = Branch::from(String::from("0"), &branch1, &branch6);

    let work = Arc::new(Mutex::new(vec![Arc::clone(&branch0)]));
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    let mut worker_id = 0;
loop {
    let num = Arc::clone(&counter);
    if work.lock().unwrap().is_empty() && *num.lock().unwrap() == 0 {
        break;
    } else if let Some(node) = work.lock().unwrap().pop() {
        let wwork = Arc::clone(&work);
        let nnum = Arc::clone(&num);
        *nnum.lock().unwrap() += 1;
        let current_worker_id = worker_id;
        worker_id += 1; 
        let handle = thread::spawn(move || {
            let mut trace: Vec<String> = vec![];
            node.lock().unwrap().explore(
                Arc::clone(&node),
                &mut trace,
                &mut Arc::clone(&wwork),
                &mut Arc::clone(&nnum),
            );
            println!("worker {} explored nodes: {:?}", current_worker_id, trace);
        });
        handles.push(handle);
    }
}
    for handle in handles {
        handle.join().unwrap();
    }
}
