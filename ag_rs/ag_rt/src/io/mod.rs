use ::{
    crossbeam::channel::{unbounded, Receiver, Sender},
    std::thread::{spawn, JoinHandle},
};
fn io_thread(receiver: &Receiver<IoTask>) -> Result<(), String> {
    loop {
        let task = receiver
            .recv()
            .map_err(|e| format!("IO thread crashed unexpectedly: {e}"))?;

        use IoTask::*;
        match task {
            Kill => return Ok(()),
            Print(msg) => {
                print!("{msg}");
            }
            Println(msg) => {
                println!("{msg}");
            }
        };
    }
}

pub enum IoTask {
    Print(String),
    Println(String),
    Kill,
}

pub fn create_io() -> (JoinHandle<()>, Sender<IoTask>) {
    let (sender, receiver) = unbounded();

    let handle = spawn(|| {
        let receiver = receiver;
        loop {
            match io_thread(&receiver) {
                Ok(_) => return,
                Err(e) => {
                    eprintln!("{e}\nRebooting...");
                    continue;
                }
            }
        }
    });

    (handle, sender)
}
