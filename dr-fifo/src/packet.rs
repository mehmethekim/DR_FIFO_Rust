// packet.rs

use rand::Rng;
use rand_distr::{Distribution, Poisson};

// Packet structure
#[derive(Debug)]
pub struct Packet {
    pub id: u32,
    pub priority: u32,
    pub data: Vec<u8>,
    pub incoming_source: u32,
    pub outgoing_source: u32,
}

// Packet generator
pub struct PacketGenerator {
    packet_id_counter: u32,
    incoming_sources: usize,
    outgoing_sources: usize,
    poisson_lambda: f64,
}

impl PacketGenerator {
    pub fn new(incoming_sources: usize, outgoing_sources: usize, poisson_lambda: f64) -> Self {
        Self {
            packet_id_counter: 0,
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

            let packet = Packet {
                id: self.packet_id_counter,
                priority,
                data,
                incoming_source: rand::thread_rng().gen_range(0..self.incoming_sources as u32),
                outgoing_source: rand::thread_rng().gen_range(0..self.outgoing_sources as u32),
            };

            self.packet_id_counter += 1;
            packets.push(packet);
        }

        println!("Time: {}s - Generated {} packets", time_elapsed, packets.len());

        packets
    }

    fn generate_priority(&mut self) -> u32 {
        let poisson = Poisson::new(self.poisson_lambda).unwrap();
        poisson.sample(&mut rand::thread_rng()) as u32
    }

    fn generate_data(&mut self) -> Vec<u8> {
        let data_len = rand::thread_rng().gen_range(1..=100);
        let mut data = Vec::with_capacity(data_len);

        for _ in 0..data_len {
            data.push(rand::thread_rng().gen());
        }

        data
    }

    fn generate_poisson_packets(&mut self, max_packets: usize) -> usize {
        let poisson = Poisson::new(self.poisson_lambda).unwrap();
        let packets = poisson.sample(&mut rand::thread_rng()) as usize;
        packets.min(max_packets)
    }
}
