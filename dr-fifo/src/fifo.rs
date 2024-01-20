// scheduler.rs

use crate::packet::Packet;
use std::collections::VecDeque;
use std::time::{Duration, Instant};

// FIFO Scheduler structure
pub struct FifoScheduler {
    queue: VecDeque<Packet>,
    served_packets_count: usize,
    last_serve_time: Instant,
}

impl FifoScheduler {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            served_packets_count: 0,
            last_serve_time: Instant::now(),
        }
    }

    pub fn enqueue(&mut self, packet: Packet) {
        self.queue.push_back(packet);
        println!("Enqueued: Packet ID {:?}", self.queue.back().unwrap().id);
    }

    pub fn serve_packets(&mut self) {
        let now = Instant::now();

        // Check if a second has elapsed since the last serve
        if now.duration_since(self.last_serve_time) >= Duration::from_secs(1) {
            // Reset the served_packets_count if a second has passed
            self.served_packets_count = 0;
            self.last_serve_time = now;
        }

        // Serve up to 4 packets if the limit has not been reached
        while self.served_packets_count < 4 {
            if let Some(packet) = self.queue.pop_front() {
                println!("Dequeued: Packet ID {:?}", &packet.id);
                self.served_packets_count += 1;
            } else {
                break; // Break if there are no more packets in the queue
            }
        }
    }
}
