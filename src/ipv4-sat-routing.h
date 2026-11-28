/*
 * Copyright (c) 2006 Georgia Tech Research Corporation
 *
 * SPDX-License-Identifier: GPL-2.0-only
 *
 * Author: George F. Riley<riley@ece.gatech.edu>
 *         Gustavo Carneiro <gjc@inescporto.pt>
 */

#ifndef IPV4_SAT_ROUTING_H
#define IPV4_SAT_ROUTING_H

#include <ns3/ipv4-routing-protocol.h>
#include <ns3/ptr.h>
#include <ns3/socket.h>

#include <stdint.h>

namespace ns3
{

class Packet;
class NetDevice;
class Ipv4Interface;
class Ipv4Address;
class Ipv4Header;
class Ipv4RoutingTableEntry;
class Ipv4MulticastRoutingTableEntry;
class Node;

class Ipv4SatRouting : public Ipv4RoutingProtocol
{
  public:
    /**
     * @brief The interface Id associated with this class.
     * @return type identifier
     */
    static TypeId GetTypeId();

    Ipv4SatRouting();
    ~Ipv4SatRouting() override;

    Ptr<Ipv4Route> RouteOutput(Ptr<Packet> p,
                               const Ipv4Header& header,
                               Ptr<NetDevice> oif,
                               Socket::SocketErrno& sockerr) override;

    bool RouteInput(Ptr<const Packet> p,
                    const Ipv4Header& header,
                    Ptr<const NetDevice> idev,
                    const UnicastForwardCallback& ucb,
                    const MulticastForwardCallback& mcb,
                    const LocalDeliverCallback& lcb,
                    const ErrorCallback& ecb) override;

    void NotifyInterfaceUp(uint32_t interface) override;
    void NotifyInterfaceDown(uint32_t interface) override;
    void NotifyAddAddress(uint32_t interface, Ipv4InterfaceAddress address) override;
    void NotifyRemoveAddress(uint32_t interface, Ipv4InterfaceAddress address) override;
    void SetIpv4(Ptr<Ipv4> ipv4) override;
    void PrintRoutingTable(Ptr<OutputStreamWrapper> stream,
                           Time::Unit unit = Time::S) const override;

  protected:
    void DoDispose() override;
};

} // Namespace ns3

#endif /* IPV4_SAT_ROUTING_H */
