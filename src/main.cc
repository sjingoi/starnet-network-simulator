#include <ns3/applications-module.h>
#include <ns3/core-module.h>
#include <ns3/internet-module.h>
#include <ns3/network-module.h>
#include <ns3/csma-module.h>
#include <ns3/internet-apps-module.h>
#include <orbit-sim.h>
#include <ns3/traffic-control-layer.h>

#include "ipv4-sat-routing.h"

using namespace ns3;

void InstallStack(Ptr<Node> node)
{
    node->AggregateObject(CreateObject<ArpL3Protocol>());
    Ptr<Ipv4L3Protocol> ipv4 = CreateObject<Ipv4L3Protocol>();
    node->AggregateObject(ipv4);
    node->AggregateObject(CreateObject<Icmpv4L4Protocol>());
    node->AggregateObject(CreateObject<TrafficControlLayer>());

    node->AggregateObject(CreateObject<UdpL4Protocol>());
    node->AggregateObject(CreateObject<TcpL4Protocol>());

    Ptr<Ipv4SatRouting> rt = CreateObject<Ipv4SatRouting>();
    ipv4->SetRoutingProtocol(rt);

    Ptr<ArpL3Protocol> arp = node->GetObject<ArpL3Protocol>();
    Ptr<TrafficControlLayer> tc = node->GetObject<TrafficControlLayer>();
    arp->SetTrafficControl(tc);
}

int main() {
    // start_visualizer();
    Time::SetResolution(ns3::Time::US);

    NodeContainer nodes;
    nodes.Create(3);

    CsmaHelper csma;
    csma.SetChannelAttribute("DataRate", StringValue("1Mbps"));
    csma.SetChannelAttribute("Delay", StringValue("2ms"));
    NetDeviceContainer devices = csma.Install(nodes);

    InstallStack(nodes.Get(0));
    InstallStack(nodes.Get(1));
    InstallStack(nodes.Get(2));

    // InternetStackHelper network_stack;
    // network_stack.Install(nodes);

    Ipv4AddressHelper address;
    address.SetBase("10.1.1.0", "255.255.255.0");
    Ipv4InterfaceContainer interfaces = address.Assign(devices);

    PingHelper ping = PingHelper(interfaces.GetAddress(1));
    ping.SetAttribute("Count", UintegerValue(6));
    ping.SetAttribute("Interval", TimeValue(Seconds(1)));
    
    ApplicationContainer app = ping.Install(nodes.Get(0));
    app.Start(Seconds(1.0));
    app.Stop(Seconds(20.0));

    csma.EnablePcapAll("my-sim", true);
    
    Simulator::Run();
    Simulator::Destroy();
    return 0;
}