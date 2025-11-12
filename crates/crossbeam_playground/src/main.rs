use crossbeam_channel::{Receiver, Sender};

fn print_sender<T>(sender: Sender<T>) {
    println!("sender = {sender:?}");
}

fn _print_receiver<T>(receiver: Receiver<T>) {
    println!("receiver = {receiver:?}");
}

fn main() {
    let (b_sender, _) = crossbeam_channel::bounded::<()>(0);
    let (ub_sender, _) = crossbeam_channel::unbounded::<()>();

    print_sender(b_sender);
    print_sender(ub_sender);
}
