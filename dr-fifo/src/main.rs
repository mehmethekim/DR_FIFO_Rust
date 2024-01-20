// main.rs

mod packet;

use packet::{Packet, PacketGenerator};
use std::{thread, time};

fn main() {
    let mut packet_generator = PacketGenerator::new(4, 4, 5.0);
    let mut time_elapsed = 0;

    loop {
        let packets = packet_generator.generate_packets(time_elapsed);

        // Process packets or integrate with your scheduler here

        // Simulate processing time
        simulate_processing_time();

        time_elapsed += 1;
    }
}

fn simulate_processing_time() {
    // Simulate processing time by sleeping for 1 second
    thread::sleep(time::Duration::from_secs(1));
}
