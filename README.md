# DR-FIFO Implementation in Rust
## Goal
Create a simulator that implements the Dynamic Ranking Push-In-First-Out (DR-PIFO) scheduling algorithm for network packets flowing through four ingress and four egress pipes.
# Features

## Packet Generation
Simulate traffic generation from four ingress pipes with variable packet sizes, arrival times, and priorities.
## DR-PIFO Scheduler
Implement the DR-PIFO algorithm in Rust, including dynamic ranking updates and error detection for departure order.
## Output
Simulate packet forwarding to four egress pipes based on the scheduler's decisions.
## Statistics
Collect and display statistics like packet count, latency, and throughput for each ingress and egress pipe.
## Configurability
Allow users to configure parameters like packet size distribution, traffic arrival rates, and scheduling policies.