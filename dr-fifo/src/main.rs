// main.rs

mod packet;
mod fifo;

use packet::{Packet, PacketGenerator};
use fifo::FifoScheduler;
use std::{thread, time};

fn main() {
    let mut packet_generator = PacketGenerator::new(4, 4, 5.0);
    let mut fifo_scheduler = FifoScheduler::new();
    let mut time_elapsed = 0;

    loop {
        

        let packets: Vec<Packet> = packet_generator.generate_packets(time_elapsed);
        println!("Enqueued: ");

        // Enqueue packets into the scheduler
        for packet in packets {
            println!("ID {}: from {} to {}",packet.id,packet.incoming_source,packet.outgoing_source);
            fifo_scheduler.enqueue(packet);
        }
        // Serve packets from the scheduler
        fifo_scheduler.serve_packets();
        // Simulate processing time
        simulate_processing_time();

        time_elapsed += 1;
    }
}

fn simulate_processing_time() {
    // Simulate processing time by sleeping for 1 second
    println!("Processing...");
    thread::sleep(time::Duration::from_secs(1));
}