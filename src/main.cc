#include <ns3/attribute.h>
#include <ns3/core-module.h>
#include <ns3/csma-helper.h>
#include <ns3/net-device-container.h>
#include <ns3/network-module.h>
#include <ns3/point-to-point-module.h>
#include <ns3/string.h>
#include <ns3/internet-module.h>
#include <ns3/csma-module.h>
#include <ns3/internet-apps-module.h>

using namespace ns3;

int main() {
    Time::SetResolution(ns3::Time::US);

    NodeContainer nodes;
    nodes.Create(3);

    CsmaHelper csma;
    csma.SetChannelAttribute("DataRate", StringValue("1Mbps"));
    csma.SetChannelAttribute("Delay", StringValue("2ms"));
    NetDeviceContainer devices = csma.Install(nodes);

    InternetStackHelper network_stack;
    network_stack.Install(nodes);

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