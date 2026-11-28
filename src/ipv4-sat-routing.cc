//
// Copyright (c) 2006 Georgia Tech Research Corporation
//
// SPDX-License-Identifier: GPL-2.0-only
//
// Author: George F. Riley<riley@ece.gatech.edu>
//         Gustavo Carneiro <gjc@inescporto.pt>

#include "ipv4-sat-routing.h"

#include <ns3/packet.h>
#include <ns3/node.h>

#include <iostream>

namespace ns3
{

NS_OBJECT_ENSURE_REGISTERED(Ipv4SatRouting);

TypeId
Ipv4SatRouting::GetTypeId()
{
    static TypeId tid = TypeId("ns3::Ipv4SatRouting")
                            .SetParent<Ipv4RoutingProtocol>()
                            .SetGroupName("Internet")
                            .AddConstructor<Ipv4SatRouting>();
    return tid;
}

Ipv4SatRouting::Ipv4SatRouting()
{
    std::cout << "Ipv4SatRouting::Ipv4SatRouting called" << std::endl;
    // Constructor
}

Ptr<Ipv4Route>
Ipv4SatRouting::RouteOutput(Ptr<Packet> p,
                               const Ipv4Header& header,
                               Ptr<NetDevice> oif,
                               Socket::SocketErrno& sockerr)
{
    std::cout << "Ipv4SatRouting::RouteOutput called" << std::endl;
    // Add route output logic
    return Create<Ipv4Route>();
}

bool
Ipv4SatRouting::RouteInput(Ptr<const Packet> p,
                              const Ipv4Header& ipHeader,
                              Ptr<const NetDevice> idev,
                              const UnicastForwardCallback& ucb,
                              const MulticastForwardCallback& mcb,
                              const LocalDeliverCallback& lcb,
                              const ErrorCallback& ecb)
{
    std::cout << "Ipv4SatRouting::RouteInput called" << std::endl;
    // Add route input logic
    return true;
}

Ipv4SatRouting::~Ipv4SatRouting()
{
    std::cout << "Ipv4SatRouting::~Ipv4SatRouting called" << std::endl;
    NS_LOG_FUNCTION(this);
}

void
Ipv4SatRouting::DoDispose()
{
    std::cout << "Ipv4SatRouting::DoDispose called" << std::endl;
    // Add dispose logic
    Ipv4RoutingProtocol::DoDispose();
}

void
Ipv4SatRouting::NotifyInterfaceUp(uint32_t i)
{
    std::cout << "Ipv4SatRouting::NotifyInterfaceUp called" << std::endl;
    // Add notify interface up logic
}

void
Ipv4SatRouting::NotifyInterfaceDown(uint32_t i)
{
    std::cout << "Ipv4SatRouting::NotifyInterfaceDown called" << std::endl;
    // Add notify interface down logic
}

void
Ipv4SatRouting::NotifyAddAddress(uint32_t interface, Ipv4InterfaceAddress address)
{
    std::cout << "Ipv4SatRouting::NotifyAddAddress called" << std::endl;
    // Add notify add address logic
}

void
Ipv4SatRouting::NotifyRemoveAddress(uint32_t interface, Ipv4InterfaceAddress address)
{
    std::cout << "Ipv4SatRouting::NotifyRemoveAddress called" << std::endl;
    // Add notify add address logic
}

void
Ipv4SatRouting::SetIpv4(Ptr<Ipv4> ipv4)
{
    std::cout << "Ipv4SatRouting::SetIpv4 called" << std::endl;
    // Add set upv4 logic
}

// Formatted like output of "route -n" command
void
Ipv4SatRouting::PrintRoutingTable(Ptr<OutputStreamWrapper> stream, Time::Unit unit) const
{
    std::cout << "Ipv4SatRouting::PrintRoutingTable called" << std::endl;
    // Add printing routing table
}

} // namespace ns3
