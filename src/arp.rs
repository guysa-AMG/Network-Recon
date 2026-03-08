use pnet::datalink::{self, NetworkInterface};
use pnet::packet::arp::{ArpOperations, ArpPacket, MutableArpPacket};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket, MutableEthernetPacket};
use pnet::packet::Packet;
use std::net::Ipv4Addr;

pub fn get_interface(iface_name: &str) -> Option<NetworkInterface> {
    let interfaces = datalink::interfaces();
    interfaces
        .into_iter()
        .find(|iface| iface.name == iface_name)
}

pub fn send_arp_request(
    interface: &NetworkInterface,
    source_ip: Ipv4Addr,
    source_mac: [u8; 6],
    target_ip: Ipv4Addr,
) {
    let (mut tx, _) = match datalink::channel(interface, Default::default()) {
        Ok(datalink::Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unknown channel type"),
        Err(e) => panic!("Error opening channel: {}", e),
    };

    let mut ethernet_buffer = [0u8; 42];
    let mut ethernet_packet = MutableEthernetPacket::new(&mut ethernet_buffer).unwrap();

    ethernet_packet.set_destination([0xff, 0xff, 0xff, 0xff, 0xff, 0xff].into());
    ethernet_packet.set_source(source_mac.into());
    ethernet_packet.set_ethertype(EtherTypes::Arp);

    let mut arp_buffer = [0u8; 28];
    let mut arp_packet = MutableArpPacket::new(&mut arp_buffer).unwrap();

    arp_packet.set_hardware_type(pnet::packet::arp::ArpHardwareTypes::Ethernet);
    arp_packet.set_protocol_type(EtherTypes::Ipv4);
    arp_packet.set_hw_addr_len(6);
    arp_packet.set_proto_addr_len(4);
    arp_packet.set_operation(ArpOperations::Request);
    arp_packet.set_sender_hw_addr(source_mac.into());
    arp_packet.set_sender_proto_addr(source_ip);
    arp_packet.set_target_hw_addr([0, 0, 0, 0, 0, 0].into());
    arp_packet.set_target_proto_addr(target_ip);

    ethernet_packet.set_payload(arp_packet.packet());

    tx.send_to(ethernet_packet.packet(), None);
}

pub fn receive_arp_responses(interface: &NetworkInterface) {
    let (_, mut rx) = match datalink::channel(interface, Default::default()) {
        Ok(datalink::Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unknown channel type"),
        Err(e) => panic!("Error opening channel: {}", e),
    };

    loop {
        match rx.next() {
            Ok(packet) => {
                let ethernet_packet = EthernetPacket::new(packet).unwrap();
                if ethernet_packet.get_ethertype() == EtherTypes::Arp {
                    let arp_packet = ArpPacket::new(ethernet_packet.payload()).unwrap();
                    if arp_packet.get_operation() == ArpOperations::Reply {
                        println!(
                            "ARP reply from: {} at {}",
                            arp_packet.get_sender_proto_addr(),
                            arp_packet.get_sender_hw_addr()
                        );
                    }
                }
            }
            Err(e) => {
                panic!("An error occurred while reading: {}", e);
            }
        }
    }
}
