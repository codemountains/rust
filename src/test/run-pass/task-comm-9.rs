use std;
import std::task;
import std::comm;

fn main() { test00(); }

fn test00_start(c: comm::chan<int>, number_of_messages: int) {
    let i: int = 0;
    while i < number_of_messages { comm::send(c, i + 0); i += 1; }
}

fn test00() {
    let r: int = 0;
    let sum: int = 0;
    let p = comm::port();
    let number_of_messages: int = 10;

    let thunk = bind test00_start(comm::chan(p), number_of_messages);
    let t0 = task::spawn_joinable(thunk);

    let i: int = 0;
    while i < number_of_messages { sum += comm::recv(p); log r; i += 1; }

    task::join(t0);

    assert (sum == number_of_messages * (number_of_messages - 1) / 2);
}
