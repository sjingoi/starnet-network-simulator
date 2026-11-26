#ifndef ORBIT_SIM_H
#define ORBIT_SIM_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef uint32_t Address;

// Opens the visualizer window. Blocks until the window is closed.
void start_visualizer(void);

// Sets the visualizer's simulation time (seconds).
void set_visualizer_time(double time_s);

double sim_latency_ms(Address source, Address target, double time_s);

double sim_packet_loss(Address source, Address target, double time_s);

Address next_hop(Address node, Address dest, double time_s);

#ifdef __cplusplus
}
#endif

#endif // ORBIT_SIM_H
