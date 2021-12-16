

#[derive(Debug, PartialEq)]
struct Packet {
    version: usize,
    type_id: usize,
    content: PacketContent
}

impl Packet {
    fn version_sum(&self) -> usize {
        match self.content {
            PacketContent::Value(_) => self.version,
            PacketContent::SubPackets(ref p) => {
                p.iter().map(|x| x.version_sum()).sum::<usize>()
                    + self.version
            },
        }
    }

    fn eval(&self) -> usize {
        let mut subvalues;
        match &self.content {
            PacketContent::Value(x) => { return *x },
            PacketContent::SubPackets(ps) => {
                subvalues = ps.iter().map(|p| p.eval())
            }
        };

        match self.type_id {
            // sum
            0 => { subvalues.sum() },
            1 => { subvalues.product() },
            2 => { subvalues.min().unwrap() },
            3 => { subvalues.max().unwrap() },
            5 => {
                if subvalues.next().unwrap() > subvalues.next().unwrap() {
                    1
                } else {
                    0
                }
            },
            6 => {
                if subvalues.next().unwrap() < subvalues.next().unwrap() {
                    1
                } else {
                    0
                }
            },
            7 => {
                if subvalues.next().unwrap() == subvalues.next().unwrap() {
                    1
                } else {
                    0
                }
            },
            _ => panic!("Invalid type id")
        }
    }
}

#[derive(Debug, PartialEq)]
enum PacketContent {
    Value(usize),
    SubPackets(Vec<Packet>)
}


fn parse_packet_value(mut input: Vec<usize>) -> (Vec<usize>, PacketContent) {
    let mut result_bits: String = String::new();
    loop {
        let last = input.remove(0) == 0;
        result_bits.push_str(
            &input.drain(0..4).map(|x| x.to_string()).collect::<String>()
        );
        if last { break; }
    };

    (input, PacketContent::Value(usize::from_str_radix(&result_bits, 2).unwrap()))
}

fn parse_subpackets(mut input: Vec<usize>) -> (Vec<usize>, PacketContent) {
    let mut subpackets = vec![];
    let input = match input.remove(0) {
        // based on length of bytes
        0 => {
            let (mut input, bytelen) = parse_num(input, 15);
            let len_before = input.len();

            loop {
                let (input_, packet) = parse_packet(input);
                input = input_;
                subpackets.push(packet);

                if input.len() == len_before - bytelen { break input; }
            }
        },
        // Based on amount of subpackets
        1 => {
            let (mut input, count) = parse_num(input, 11);

            for _ in 0..count {
                let (input_, packet) = parse_packet(input);
                input = input_;
                subpackets.push(packet);
            }
            input
        }
        _ => panic!("Invalid length type id"),
    };

    (input, PacketContent::SubPackets(subpackets))
}

fn parse_num(mut input: Vec<usize>, len: usize) -> (Vec<usize>, usize) {
    let num = usize::from_str_radix(
        &input.drain(0..len)
            .map(|x| x.to_string())
            .collect::<String>(),
        2
    ).unwrap();
    (input, num)
}

fn parse_packet(mut input: Vec<usize>) -> (Vec<usize>, Packet) {
    let (input, version) = parse_num(input, 3);
    let (input, type_id) = parse_num(input, 3);

    let (input, content) = match type_id {
        4 => { parse_packet_value(input) },
        _ => { parse_subpackets(input) },
    };

    (input, Packet { version, type_id, content })
}

fn hex_to_bits(input: &str) -> Vec<usize> {
    input.chars()
        .map(|c| {
            match c {
                '0' => &[0,0,0,0],
                '1' => &[0,0,0,1],
                '2' => &[0,0,1,0],
                '3' => &[0,0,1,1],
                '4' => &[0,1,0,0],
                '5' => &[0,1,0,1],
                '6' => &[0,1,1,0],
                '7' => &[0,1,1,1],
                '8' => &[1,0,0,0],
                '9' => &[1,0,0,1],
                'A' => &[1,0,1,0],
                'B' => &[1,0,1,1],
                'C' => &[1,1,0,0],
                'D' => &[1,1,0,1],
                'E' => &[1,1,1,0],
                'F' => &[1,1,1,1],
                _ => panic!("invalid char")
            }
        })
        .flat_map(|x| x.iter())
        .map(|x| *x)
        .collect()
}


fn main() {
    let mut input = std::fs::read_to_string("input/day16").unwrap();
    input.pop(); // remove newline
    let mut bits = hex_to_bits(&input);
    let (bits, p) = parse_packet(bits);
    println!("Part 1: {}", p.version_sum());
    println!("Part 2: {}", p.eval());
}


#[test]
fn test_example_value() {
    let mut bits = hex_to_bits("D2FE28");
    let (bits, p) = parse_packet(bits);
    assert_eq!(p.version, 6);
    assert_eq!(p.type_id, 4);
    assert_eq!(p.content, PacketContent::Value(2021));
}

#[test]
fn test_subpackets() {
    let mut bits = hex_to_bits("38006F45291200");
    let (bits, p) = parse_packet(bits);
    assert_eq!(p.version, 1);
    assert_eq!(p.type_id, 6);
    //assert_eq!(bits.len(), 0);
    // assert_eq!(p.content, PacketContent::Value(2021));
}


#[test]
fn test_subpackets_2() {
    let mut bits = hex_to_bits("EE00D40C823060");
    let (bits, p) = parse_packet(bits);
    assert_eq!(p.version, 7);
    assert_eq!(p.type_id, 3);
    // assert_eq!(bits.len(), 0);
}


#[test]
fn test_sum_first() {
    let mut bits = hex_to_bits("8A004A801A8002F478");
    let (bits, p) = parse_packet(bits);
    assert_eq!(p.version_sum(), 16);
}

#[test]
fn test_sum_last() {
    let mut bits = hex_to_bits("A0016C880162017C3686B18A3D4780");
    let (bits, p) = parse_packet(bits);
    assert_eq!(p.version_sum(), 31);
}

#[test]
fn test_part2() {
    let mut bits = hex_to_bits("9C0141080250320F1802104A08");
    let (bits, p) = parse_packet(bits);
    assert_eq!(p.eval(), 1);
}
