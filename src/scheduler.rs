pub struct Scheduler {}

impl Scheduler {
    pub fn schedule(tasks: Vec<Box<(dyn Fn() + Send)>>) {
        let threads = tasks
            .into_iter()
            .map(|f| std::thread::spawn(f))
            .collect::<Vec<_>>();

        threads.into_iter().for_each(|h| h.join().unwrap());
    }
}
