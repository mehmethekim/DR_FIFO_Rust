// scheduler.rs

use crate::packet::Packet;
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};

// FIFO Scheduler structure
pub struct FifoScheduler {
    input_queues: HashMap<u32, VecDeque<Packet>>, // Map input port to its queue
    output_ports: Vec<Option<Packet>>, // The output ports, each can hold at most 1 packet
    served_packets_count: usize,
    last_serve_time: Instant,
}

impl FifoScheduler {
    pub fn new() -> Self {
        let mut input_queues = HashMap::new();
        let mut output_ports = Vec::with_capacity(4);
        for _ in 0..4 {
            output_ports.push(None);
        }

        for i in 0..4 {
            input_queues.insert(i, VecDeque::new());
        }

        Self {
            input_queues,
            output_ports,
            served_packets_count: 0,
            last_serve_time: Instant::now(),
        }
    }

    pub fn enqueue(&mut self, packet: Packet) {
        let input_port = packet.incoming_source;
        self.input_queues
            .entry(input_port)
            .or_insert(VecDeque::new())
            .push_back(packet.clone());
        println!(
            "Enqueued: Packet ID {:?} at Input Port {}",
            packet.id, input_port
        );

        self.print_queues();
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
            let output_port = self.served_packets_count;
            if let Some(packet) = self.get_next_packet(output_port) {
                println!(
                    "Dequeued: Packet ID {:?} from Input Port {} to Output Port {}",
                    packet.id, packet.incoming_source, output_port
                );
                self.output_ports[output_port] = Some(packet);
                self.served_packets_count += 1;
            } else {
                break; // Break if there are no more packets to serve
            }
        }

        self.print_queues();
    }

    fn get_next_packet(&mut self, output_port: usize) -> Option<Packet> {
        // Try to get the next packet from the first non-empty input queue
        for input_queue in self.input_queues.values_mut() {
            if let Some(packet) = input_queue.pop_front() {
                return Some(packet);
            }
        }

        // If no packets are available in any input queue, try to get from the output port
        if let Some(packet) = self.output_ports[output_port].take() {
            return Some(packet);
        }

        None
    }

    fn print_queues(&self) {
        for (input_port, input_queue) in &self.input_queues {
            print!("{}st Queue: ", input_port + 1);
            for packet in input_queue {
                print!("{},", packet.id);
            }
            println!();
        }
        println!();
    }
}
