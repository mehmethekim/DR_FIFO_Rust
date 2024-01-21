use crate::packet::Packet;

pub trait PacketScheduler {
    fn enqueue(&mut self, packet: Packet);
    fn schedule(&mut self);
    fn dequeue(&mut self);
}
// FIFO Scheduler structure
pub struct FifoScheduler {
    input_queues: Vec<VecDeque<Packet>>, // List of input queues
    output_ports: Vec<Option<(Packet, Instant)>>, // The output ports, each can hold at most 1 packet and its departure time
    served_packets_count: usize,
    last_serve_time: Instant,
    latency_log: File,
}
