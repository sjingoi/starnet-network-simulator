#ifndef ORBIT_SIM_H
#define ORBIT_SIM_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef uint32_t Address;

// Opens the visualizer window. Blocks until the window is closed.
void start_visualizer(void);

// Sets the visualizer's simulation time (seconds).
void set_visualizer_time(double time_s);

// One-way latency (milliseconds) of the direct link from `source` to `target`
// at simulation time `time_s` (seconds). The two nodes must be neighbours.
double sim_latency_ms(Address source, Address target, double time_s);

// Packet loss of the direct link from `source` to `target` at simulation time
// `time_s` (seconds), as a fraction from 0.0 (no loss) to 1.0 (all packets lost).
// The two nodes must be neighbours.
double sim_packet_loss(Address source, Address target, double time_s);

// Routing decision: returns the neighbour of `node` that a packet headed for
// `dest` should be forwarded to at simulation time `time_s` (seconds).
Address next_hop(Address node, Address dest, double time_s);

// Writes up to `capacity` node addresses into `out` and returns the total number of nodes.
// Pass out = NULL to query the count first:
//   size_t n = get_all_nodes(NULL, 0);
//   Address *addrs = malloc(n * sizeof(Address));
//   get_all_nodes(addrs, n);
size_t get_all_nodes(Address *out, size_t capacity);

#ifdef __cplusplus
}
#endif

#endif // ORBIT_SIM_H
