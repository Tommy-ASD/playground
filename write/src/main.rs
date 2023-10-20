use std::fmt;

struct DataPacket {
    id: u32,
    payload: Vec<u8>,
}

impl DataPacket {
    fn format_for_protocol(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Format the data packet according to the protocol's requirements.
        write!(f, "ID: {}, Payload Length: {}", self.id, self.payload.len())?;
        // Add more formatting specific to the protocol.
        // ...
        Ok(())
    }
}

fn main() {
    let packet = DataPacket {
        id: 123,
        payload: vec![0, 1, 2, 3],
    };

    let formatted_data = format!(
        "{}",
        packet
            .format_for_protocol(&mut std::fmt::Formatter::new(Vec::new()))
            .unwrap()
    );
    println!("Formatted Data for Protocol: {}", formatted_data);
}
