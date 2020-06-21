use anyhow::Result;
use std::convert::TryInto;
use std::net::Ipv4Addr;
use std::ops::Range;

const SOURCE: Ipv4Addr = Ipv4Addr::new(10, 1, 1, 10);
const DESTINATION: Ipv4Addr = Ipv4Addr::new(10, 1, 1, 200);
const DESTINATION_PORT: u16 = 42069;
const IP_TOTAL_LENGTH: Range<usize> = 2..4;
const IP_PROTOCOL: usize = 9;
const IP_SOURCE_ADDRESS: Range<usize> = 12..16;
const IP_DESTINATION_ADDRESS: Range<usize> = 16..20;
const UDP_HEADER_LENGTH: usize = 8;
const UDP_DESTINATION_PORT_OFFSET: usize = 2;

pub fn udp(input: Vec<u8>) -> Result<Vec<u8>> {
    let mut result = Vec::new();
    let mut i = &input[..];

    while !i.is_empty() {
        let ihl = (i[0] & 0b00001111) as usize * 4;
        let length = u16::from_be_bytes(i[IP_TOTAL_LENGTH].try_into()?);

        let from_addr_bytes: [u8; 4] = i[IP_SOURCE_ADDRESS].try_into()?;
        let from_addr = Ipv4Addr::from(from_addr_bytes);

        let to_addr_bytes: [u8; 4] = i[IP_DESTINATION_ADDRESS].try_into()?;
        let to_addr = Ipv4Addr::from(to_addr_bytes);

        let mut ip_checksum: u16 = 0;
        for chunk in i[..ihl].chunks(2) {
            ip_checksum = ip_checksum.wrapping_add(u16::from_be_bytes(chunk.try_into()?));
        }
        ip_checksum = !ip_checksum;
        let port_offset = ihl + UDP_DESTINATION_PORT_OFFSET;
        let to_port = u16::from_be_bytes([i[port_offset], i[port_offset + 1]]);

        let mut udp_checksum: u16 = 0;

        udp_checksum =
            udp_checksum.wrapping_add(u16::from_be_bytes(from_addr_bytes[..2].try_into()?));
        udp_checksum =
            udp_checksum.wrapping_add(u16::from_be_bytes(from_addr_bytes[2..].try_into()?));
        udp_checksum =
            udp_checksum.wrapping_add(u16::from_be_bytes(to_addr_bytes[..2].try_into()?));
        udp_checksum =
            udp_checksum.wrapping_add(u16::from_be_bytes(to_addr_bytes[2..].try_into()?));
        udp_checksum = udp_checksum.wrapping_add(i[IP_PROTOCOL] as u16);
        udp_checksum = udp_checksum.wrapping_add((length as usize - ihl) as u16);

        let udp_packet = &i[ihl..length as usize];
        let mut chunks = udp_packet.chunks_exact(2);
        for chunk in &mut chunks {
            let before = udp_checksum;
            udp_checksum = udp_checksum.wrapping_add(u16::from_be_bytes(chunk.try_into()?));
            if before > udp_checksum {
                // I have no idea why this is necessary...
                udp_checksum = udp_checksum.wrapping_add(1);
            }
        }
        if !chunks.remainder().is_empty() {
            let chunk = [chunks.remainder()[0], 0];
            udp_checksum = udp_checksum.wrapping_add(u16::from_be_bytes(chunk));
        }
        udp_checksum = !udp_checksum;

        if from_addr == SOURCE
            && to_addr == DESTINATION
            && to_port == DESTINATION_PORT
            && ip_checksum == 0
            && udp_checksum == 0
        {
            result.extend_from_slice(&udp_packet[UDP_HEADER_LENGTH..]);
        }

        i = &i[length as usize..];
    }

    Ok(result)
}
