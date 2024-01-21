
use crate::packet::Packet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, Write};
use std::time::{Duration, Instant};

// FIFO Scheduler structure
pub struct FifoScheduler {
    input_queues: Vec<VecDeque<Packet>>, // List of input queues
    output_ports: Vec<Option<(Packet, Instant)>>, // The output ports, each can hold at most 1 packet and its departure time
    served_packets_count: usize,
    latency_log: File,
}

impl FifoScheduler {
    pub fn new() -> Self {
        let mut input_queues = Vec::new();
        for _ in 0..4 {
            input_queues.push(VecDeque::new());
        }

        let mut output_ports = Vec::with_capacity(4);
        for _ in 0..4 {
            output_ports.push(None);
        }

        let latency_log = File::create("latency_log.txt").expect("Unable to create latency log file");

        Self {
            input_queues,
            output_ports,
            served_packets_count: 0,
            latency_log,
        }
    }

    pub fn enqueue(&mut self, packet: Packet) {
        let input_port = packet.incoming_source;
        self.input_queues[input_port as usize].push_back(packet.clone());
    }

    pub fn serve_packets(&mut self) {

        self.print_queues();
        let now = Instant::now();
        self.served_packets_count = 0;
        

        // Serve up to 4 packets if the limit has not been reached
        println!("Dequeued: ");
        while self.served_packets_count < 4 {
            let output_port = self.served_packets_count;
            if let Some((packet, entry_time)) = self.get_next_packet(output_port) {
                println!(
                    "ID {:?} from {} to {}",
                    packet.id, packet.incoming_source, output_port
                );

                let latency = now.duration_since(packet.incoming_time);
                self.log_latency(packet.id, latency);

                self.output_ports[output_port] = Some((packet.clone(), now));
                self.served_packets_count += 1;
            } else {
                self.served_packets_count += 1;
            } 
        }

        self.print_queues();
    }

    fn get_next_packet(&mut self, output_port: usize) -> Option<(Packet, Instant)> {
        // Try to get the next packet from the first non-empty input queue
        if let Some(packet) = self.input_queues[output_port].pop_front() {
            return Some((packet, Instant::now()));
        }

        

        None
    }

    fn print_queues(&self) {
        for (input_port, input_queue) in self.input_queues.iter().enumerate() {
            print!("Queue {}: ", input_port);
            for packet in input_queue {
                print!("{},", packet.id);
            }
            println!();
        }
        println!();
    }

    fn log_latency(&mut self, packet_id: u64, latency: Duration) {
        let latency_log_line = format!("Packet ID: {} , latency: {} sec\n", packet_id, latency.as_secs_f64());
        if let Err(err) = self.latency_log.write_all(latency_log_line.as_bytes()) {
            eprintln!("Error writing to latency log: {}", err);
        }
    }
}
