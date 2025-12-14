//! xdelta test helpers

pub struct VcdiffWindowBuilder {
    pub indicator: u8,
    pub source_data: Option<(u64, u64)>, // length, position
    pub target_window_length: u64,
    pub add_data: Vec<u8>,
    pub instructions: Vec<u8>,
    pub addresses: Vec<u8>,
    pub adler32: Option<u32>,
}

impl VcdiffWindowBuilder {
    pub fn new(target_len: u64) -> Self {
        Self {
            indicator: 0,
            source_data: None,
            target_window_length: target_len,
            add_data: Vec::new(),
            instructions: Vec::new(),
            addresses: Vec::new(),
            adler32: None,
        }
    }

    pub fn with_add(mut self, data: &[u8]) -> Self {
        self.add_data.extend_from_slice(data);
        self
    }

    pub fn with_instructions(mut self, inst: &[u8]) -> Self {
        self.instructions.extend_from_slice(inst);
        self
    }

    pub fn with_addresses(mut self, addr: &[u8]) -> Self {
        self.addresses.extend_from_slice(addr);
        self
    }

    pub fn build(&self) -> Vec<u8> {
        let mut win_header = Vec::new();

        // Indicator
        // Auto-set ADLER32 flag if checksum is present
        let mut indicator = self.indicator;
        if self.adler32.is_some() {
            indicator |= 0x04; // VCD_ADLER32
        }
        win_header.push(indicator);

        // Source data (if any)
        if let Some((len, pos)) = self.source_data {
            encode_7bit(&mut win_header, len);
            encode_7bit(&mut win_header, pos);
        }

        // We need to calculate header size to know delta_length.
        // But delta_length is PART of the header.
        // This is the tricky circular dependency.
        //
        // RFC 3284 Section 4.2:
        // "Length of the delta encoding ... includes the window header..."
        //
        // The header structure:
        // Win_Indicator (1)
        // [Source...]
        // Delta_Length (Varint)
        // Target_Length (Varint)
        // Delta_Indicator (1)
        // Add_Len (Varint)
        // Inst_Len (Varint)
        // Addr_Len (Varint)
        // [Adler32] (4)
        //
        // Data sections...

        let mut rest_of_header = Vec::new();
        encode_7bit(&mut rest_of_header, self.target_window_length);
        rest_of_header.push(0x00); // Delta Indicator (assuming 0 for tests)
        encode_7bit(&mut rest_of_header, self.add_data.len() as u64);
        encode_7bit(&mut rest_of_header, self.instructions.len() as u64);
        encode_7bit(&mut rest_of_header, self.addresses.len() as u64);
        if let Some(adler) = self.adler32 {
            rest_of_header.extend_from_slice(&adler.to_be_bytes());
        }

        let data_len = self.add_data.len() + self.instructions.len() + self.addresses.len();

        // Calculate Delta Length
        // Delta Length = (Size of Delta Length Varint) + Size(Rest of Header) +
        // Size(Data) + Size(Win_Indicator + Source) Wait, "Length of the delta
        // encoding... includes the window header" Let X be the value of
        // Delta_Length. Size(Window) = Size(Indicator) + Size(Source) +
        // Size(Varint(X)) + Size(RestOfHeader) + DataLen. X = Size(Window) -
        // (Start of Window to Start of DeltaLength?)? No, X is the length of
        // the encoding *of the target window*. Usually, X = TotalBytes -
        // BytesBeforeX? No.
        //
        // Let's assume Delta_Length DOES NOT include itself or bytes before it.
        // RFC: "Length of the delta encoding of the target window. This includes the
        // window header, the three length fields, and the data sections." "This
        // includes the window header". The "window header" starts at
        // "Win_Indicator". So Delta_Length = Total Window Size.
        // But Delta_Length is a variable length integer.
        // Let T = Total Size.
        // T = Size(Indicator) + Size(Source) + Size(Varint(T)) + Size(RestOfHeader) +
        // DataLen. We need to find T such that T fits in Size(Varint(T)) +
        // constant.
        //
        // Iterate T?

        let constant_part = win_header.len() + rest_of_header.len() + data_len;
        let mut t = constant_part as u64; // Start guess
        loop {
            let varint_size = varint_len(t);
            let total = constant_part as u64 + varint_size as u64;
            if total == t {
                break;
            }
            t = total;
        }

        let delta_length = t;

        // Construct final
        encode_7bit(&mut win_header, delta_length);
        win_header.extend_from_slice(&rest_of_header);

        win_header.extend_from_slice(&self.add_data);
        win_header.extend_from_slice(&self.instructions);
        win_header.extend_from_slice(&self.addresses);

        win_header
    }
}

pub fn encode_7bit(buf: &mut Vec<u8>, mut value: u64) {
    let mut groups = Vec::new();
    if value == 0 {
        groups.push(0);
    } else {
        while value > 0 {
            groups.push((value & 0x7F) as u8);
            value >>= 7;
        }
    }

    groups.reverse();

    for (i, byte) in groups.iter().enumerate() {
        let mut b = *byte;
        if i < groups.len() - 1 {
            b |= 0x80;
        }
        buf.push(b);
    }
}
fn varint_len(value: u64) -> usize {
    if value == 0 {
        return 1;
    }
    let mut len = 0;
    let mut v = value;
    while v > 0 {
        len += 1;
        v >>= 7;
    }
    len
}

pub fn prepend_header(window: &[u8]) -> Vec<u8> {
    let mut patch = Vec::new();
    patch.extend_from_slice(&[0xD6, 0xC3, 0xC4, 0x00]); // Magic + Version
    patch.push(0x00); // Header Indicator
    patch.extend_from_slice(window);
    eprintln!("Generated patch: {:02x?}", patch);
    patch
}
