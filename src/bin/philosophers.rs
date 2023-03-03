use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Fork;

struct Philosopher {
    name: String,
    left_fork: Arc<Mutex<bool>>,
    right_fork:Arc<Mutex<bool>>,
    thoughts: mpsc::SyncSender<String>
}

impl Philosopher {
    fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
    }

    fn eat(&self) {
        self.right_fork.lock().unwrap();
        self.left_fork.lock().unwrap();
        println!("{} is eating...", &self.name);
        thread::sleep(Duration::from_millis(10));
    }
}

static PHILOSOPHERS: &[&str] =
    &["Socrates", "Plato", "Aristotle", "Thales", "Pythagoras"];

fn main() {
    // Create forks
    let mut forks = Vec::new();
    for _ in 0..PHILOSOPHERS.len() {
        forks.push(Arc::new(Mutex::new(true)));
    }

    let (tx, rx) = mpsc::sync_channel(10);
    for (i,philosopher) in PHILOSOPHERS.iter().enumerate() {
        let philosopher = Philosopher{name:philosopher.to_string(),left_fork:forks.get(i).unwrap().clone(), right_fork:forks.get((i+1)%forks.len()).unwrap().clone(),thoughts:tx.clone()};
        thread::spawn(move || {
            for i in 1..10000 {
                philosopher.eat();
                philosopher.think();
            }
        });

    }
    

    // Create philosophers

    // Make them think and eat
    drop(tx);
    for msg in rx {
        println!("{}",msg);
    }

    // Output their thoughts
}
