#[derive(Debug)]
enum Data {
    Value(u64),
    Operator(u8, Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    version: u8,
    data: Data,
}

impl Packet {
    fn sum_version_numbers(&self) -> usize {
        (self.version as usize)
            + match &self.data {
                Data::Value(_) => 0,
                Data::Operator(_, packets) => packets.iter().map(|p| p.sum_version_numbers()).sum(),
            }
    }

    fn calculate_value(&self) -> u64 {
        match &self.data {
            Data::Value(v) => *v,
            Data::Operator(0, packets) => packets.iter().map(|p| p.calculate_value()).sum(),
            Data::Operator(1, packets) => packets.iter().map(|p| p.calculate_value()).product(),
            Data::Operator(2, packets) => {
                packets.iter().map(|p| p.calculate_value()).min().unwrap()
            }
            Data::Operator(3, packets) => {
                packets.iter().map(|p| p.calculate_value()).max().unwrap()
            }
            Data::Operator(5, packets) => {
                if packets[1].calculate_value() < packets[0].calculate_value() {
                    1
                } else {
                    0
                }
            }
            Data::Operator(6, packets) => {
                if packets[0].calculate_value() < packets[1].calculate_value() {
                    1
                } else {
                    0
                }
            }
            Data::Operator(7, packets) => {
                if packets[0].calculate_value() == packets[1].calculate_value() {
                    1
                } else {
                    0
                }
            }
            _ => panic!("not supported"),
        }
    }

    fn read(raw: &str) -> (Self, usize) {
        let version = u8::from_str_radix(&raw[0..=2], 2).unwrap();
        let type_id = u8::from_str_radix(&raw[3..=5], 2).unwrap();
        let mut rest = &raw[6..];
        let (read, data) = if type_id == 4 {
            // we need to read in groups of 5, with the first bit indicating if it is the last
            let mut data = 0;
            let mut read = 0;
            loop {
                read += 5;
                let slice = &rest[0..=4];
                for c in slice[1..].chars() {
                    data = match c {
                        '0' => data << 1,
                        '1' => (data << 1) + 1,
                        _ => panic!("not supported bit"),
                    };
                }
                if &slice[0..=0] == "0" {
                    break;
                }
                rest = &rest[5..];
            }
            (read, Data::Value(data))
        } else if &rest[0..=0] == "0" {
            let raw_count = &rest[1..=15];
            let count = usize::from_str_radix(&raw_count, 2).unwrap();
            let next_raw = &rest[16..=(count + 16)];
            let mut total_read = 0;
            let mut packets = Vec::new();
            while total_read < count {
                let (packet, read) = Packet::read(&next_raw[total_read..]);
                packets.push(packet);
                total_read += read;
            }
            (1 + 15 + count, Data::Operator(type_id, packets))
        } else if &rest[0..=0] == "1" {
            let raw_count = &rest[1..=11];
            let count = usize::from_str_radix(&raw_count, 2).unwrap();
            let next_raw = &rest[12..];
            let mut total_read = 0;
            let mut packets = Vec::new();
            while packets.len() < count {
                let (packet, read) = Packet::read(&next_raw[total_read..]);
                packets.push(packet);
                total_read += read;
            }
            (1 + 11 + total_read, Data::Operator(type_id, packets))
        } else {
            panic!("unsupported");
        };

        let packet = Self { version, data };

        (packet, read + 6)
    }
}

fn solve(input: String) -> (usize, u64) {
    let binary_representation: String = input
        .chars()
        .map(|c| u8::from_str_radix(&c.to_string(), 16).unwrap())
        .map(|c| format!("{:04b}", c))
        .collect();

    let (packet, _) = Packet::read(&binary_representation);
    (packet.sum_version_numbers(), packet.calculate_value())
}

fn main() {
    let input = std::env::args().skip(1).next().unwrap();
    let (p1, p2) = solve(input);
    println!("Part 1 = {}", p1);
    println!("Part 2 = {}", p2);
}
