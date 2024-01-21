// packet.rs

use rand::Rng;
use rand_distr::{Distribution, Poisson};
use std::time::{Duration, Instant};

// Packet structure
#[derive(Debug, Clone)]
pub struct Packet {
    pub id: u64,
    pub priority: u32,
    pub data: Vec<u8>,
    pub incoming_source: u32,
    pub outgoing_source: u32,
    pub incoming_time: Instant,
}

// Packet generator
pub struct PacketGenerator {
    global_packet_id_counter: u64,
    incoming_sources: usize,
    outgoing_sources: usize,
    poisson_lambda: f64,
}

impl PacketGenerator {
    pub fn new(incoming_sources: usize, outgoing_sources: usize, poisson_lambda: f64) -> Self {
        Self {
            global_packet_id_counter: 0,
            incoming_sources,
            outgoing_sources,
            poisson_lambda,
        }
    }

    pub fn generate_packets(&mut self, time_elapsed: u32) -> Vec<Packet> {
        let mut packets = Vec::new();

        let max_packets_per_second = 5;
        let packets_to_generate = self.generate_poisson_packets(max_packets_per_second);

        for _ in 0..packets_to_generate {
            let priority = self.generate_priority();
            let data = self.generate_data();
            let incoming_time = Instant::now();

            let packet = Packet {
                id: self.global_packet_id_counter,
                priority,
                data,
                incoming_source: rand::thread_rng().gen_range(0..self.incoming_sources as u32),
                outgoing_source: rand::thread_rng().gen_range(0..self.outgoing_sources as u32),
                incoming_time,
            };

            self.global_packet_id_counter += 1;
            packets.push(packet);
        }

        // Print the IDs of the enqueued packets
        let enqueued_ids: Vec<_> = packets.iter().map(|packet| packet.id).collect();
        println!(
            "Time: {}s - Generated {} packets. Enqueued Packet IDs: {:?}",
            time_elapsed,
            packets.len(),
            enqueued_ids
        );

        packets
    }

    fn generate_priority(&mut self) -> u32 {
        let poisson = Poisson::new(self.poisson_lambda).unwrap();
        poisson.sample(&mut rand::thread_rng()) as u32
    }

    fn generate_data(&mut self) -> Vec<u8> {
        let data_len = rand::thread_rng().gen_range(1..=100);
        rand::thread_rng().sample_iter(rand::distributions::Standard)
            .take(data_len)
            .collect()
    }

    fn generate_poisson_packets(&mut self, max_packets: usize) -> usize {
        let poisson = Poisson::new(self.poisson_lambda).unwrap();
        let packets = poisson.sample(&mut rand::thread_rng()) as usize;
        packets.min(max_packets)
    }
}

//Make an input queue of size X, outgoing ports can only send 1 packet at a time
//for example if packet A and packet B would like to go 4, only one of them can go at a time
//if packet A goes first, then packet B will have to wait until packet A is done. 