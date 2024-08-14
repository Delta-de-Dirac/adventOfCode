use std::fs;


#[derive(Debug)]
enum PacketData {
    Literal(u64),
    Operation(Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    version:  u8,
    type_id:  u8,
    data:     PacketData,
    bit_size: usize,
}

fn bin_to_u64(bin: &[bool]) -> u64 {
    let mut acc = 0;
    for (i, &b) in bin.iter().rev().enumerate() {
        if i > 63 {panic!("Cannor parse [bool] to u64");}
        if b {acc += 2u64.pow(i as u32);}
    }
    return acc;
}

fn bin_to_u16(bin: &[bool]) -> u16 {
    let mut acc = 0;
    for (i, &b) in bin.iter().rev().enumerate() {
        if i > 15 {panic!("Cannor parse [bool] to u16");}
        if b {acc += 2u16.pow(i as u32);}
    }
    return acc;
}

fn bin_to_u8(bin: &[bool]) -> u8 {
    let mut acc = 0;
    for (i, &b) in bin.iter().rev().enumerate() {
        if i > 7 {panic!("Cannor parse [bool] to u8");}
        if b {acc += 2u8.pow(i as u32);}
    }
    return acc;
}

fn parse_packet(bin: &[bool]) -> Packet {
    let mut version  = 0;
    let mut type_id  = 0;
    let mut data = PacketData::Literal(0);
    let mut bit_size = 0;
    match bin.get(0..3) {
        Some(bin_version) => {
            version = bin_to_u8(bin_version);
        }
        None => {
            panic!("Couldn't find expeceted version");
        }
    }
    match bin.get(3..6) {
        Some(bin_type_id) => {
            type_id = bin_to_u8(bin_type_id);
        }
        None => {
            eprintln!("{bin:?}");
            panic!("Couldn't find expeceted type_id");
        }
    }
    match type_id {
        4u8 => {
            let mut cont_pos = 6;
            let mut lit_buffer : Vec<bool> = Vec::new();
            loop{
                match bin.get(cont_pos+1..cont_pos+5) {
                    Some(lit_data) => {
                        lit_buffer.extend_from_slice(lit_data);
                    }
                    None => {
                        panic!("Expected literal data");
                    }
                }
                if *bin.get(cont_pos).expect("Expected lit continue data") {
                    cont_pos += 5;
                    continue;
                }
                break;
            }
            data = PacketData::Literal(bin_to_u64(&lit_buffer));
            bit_size = cont_pos+5;


            return Packet {
                version : version,
                type_id: type_id,
                data: data,
                bit_size: bit_size,
            };
        },
        _op => {
            let mut start_pos = 0;
            let mut sub_packets : Vec<Packet> = Vec::new();
            match bin.get(6) {
                Some(true) => {
                    let len_bin = bin.get(7..18).expect("Couldn't find expeceted len 11 bits");
                    let len = bin_to_u16(len_bin);
                    start_pos = 18;
                    for _ in 0..len {
                        let subpacket = parse_packet(bin.get(start_pos..).expect("No data to parse subpacket"));
                        start_pos += subpacket.bit_size;
                        sub_packets.push(subpacket);
                    }
                    return Packet {
                        version : version,
                        type_id: type_id,
                        data: PacketData::Operation(sub_packets),
                        bit_size: start_pos,
                    };
                }
                Some(false) => {
                    let len_bin = bin.get(7..22).expect("Couldn't find expeceted len 15 bits");
                    let mut bits_to_read = bin_to_u16(len_bin);
                    start_pos = 22;
                    while bits_to_read > 0 {
                        let subpacket = parse_packet(bin.get(start_pos..).expect("No data to parse subpacket"));
                        start_pos += subpacket.bit_size;
                        bits_to_read -= subpacket.bit_size as u16;
                        sub_packets.push(subpacket);
                    }
                }
                None => {
                    panic!("Couldn't find expected len_id");
                }
            }
            return Packet {
                version : version,
                type_id: type_id,
                data: PacketData::Operation(sub_packets),
                bit_size: start_pos,
            };
        },
    }
}

fn sum_packets_versions(packet: &Packet) -> u64{
    let mut acc = packet.version as u64;
    match &packet.data {
        PacketData::Operation(sub_packets) => {
            for pac in sub_packets {
                acc += sum_packets_versions(&pac);
            }
        },
        _ => {},
    }
    return acc;
}

fn print_bin(bin: &[bool]) {
    for b in bin {
        match *b {
            true =>  {print!("1");},
            false => {print!("0");},
        }
    }
    print!("\n");
}

fn eval_packet(packet: &Packet) -> u64 {
    match &packet.data {
        PacketData::Literal(x) => {*x}
        PacketData::Operation(sub_packets) => {
            match &packet.type_id {
                0u8 => {
                    let mut acc = 0;
                    for pac in sub_packets{
                        acc += eval_packet(pac);
                    }
                    acc
                },
                1u8 => {
                    let mut acc = 1;
                    for pac in sub_packets{
                        acc *= eval_packet(pac);
                    }
                    acc
                },
                2u8 => {
                    let mut min = u64::MAX;
                    for pac in sub_packets{
                        let val = eval_packet(pac);
                        if val < min {min = val};
                    }
                    min
                },
                3u8 => {
                    let mut max = u64::MIN;
                    for pac in sub_packets{
                        let val = eval_packet(pac);
                        if val > max {max = val};
                    }
                    max
                },
                4u8 => {panic!("invalid type_id for data in packet");},
                5u8 => {
                    if sub_packets.len() != 2 {panic!("invalid number of subpackets in greater than opertaion");}
                    let first  = eval_packet(&sub_packets[0]);
                    let second = eval_packet(&sub_packets[1]);
                    if first > second {1} else {0}
                },
                6u8 => {
                    if sub_packets.len() != 2 {panic!("invalid number of subpackets in less than opertaion");}
                    let first  = eval_packet(&sub_packets[0]);
                    let second = eval_packet(&sub_packets[1]);
                    if first < second {1} else {0}
                },
                7u8 => {
                    if sub_packets.len() != 2 {panic!("invalid number of subpackets in equal opertaion");}
                    let first  = eval_packet(&sub_packets[0]);
                    let second = eval_packet(&sub_packets[1]);
                    if first == second {1} else {0}
                },
                _ => {panic!("invalid type_id");},
            }
        }
    }
}

fn main(){
    println!("Starting...");

    let file_name = "./input/day16.txt";

    let input_file : String = fs::read_to_string(file_name)
                        .unwrap_or_else(|err| {
                            println!("Cannot read \"{file_name}\"");
                            println!("{err}");
                            std::process::exit(1);
                        });

    let input_lines = input_file
    .trim()
    .split(|x| x == '\n')
    .collect::<Vec<&str>>();

    let hex_coded = *input_lines.get(0).expect("Can't read input");
    let mut bin : Vec<bool> = Vec::new();

    for h in hex_coded.chars() {
        let bin_str = format!("{:04b}", h.to_digit(16).expect("Can't parse input"));
        for b in bin_str.chars(){
            bin.push(b.to_digit(2).expect("") != 0);
        }
    }

    let bin = bin;

    let main_packet = parse_packet(&bin);

    let result = sum_packets_versions(&main_packet);

    println!("Result part 1: {result}");

    let result = eval_packet(&main_packet);

    println!("Result part 2: {result}");
}
