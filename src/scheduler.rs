pub struct Scheduler {}

impl Scheduler {
    pub fn schedule(tasks: Vec<Box<(dyn Fn() + Send)>>) {
        tasks
            .into_iter()
            .map(std::thread::spawn)
            .for_each(|h| h.join().unwrap());
    }
}
