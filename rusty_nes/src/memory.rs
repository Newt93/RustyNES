// this module defines the Memory type, along with implementations for
// common memory operations such as fetching data

pub struct Memory {
    data: [u8; 65536],
}

impl Memory {
    pub fn new() -> Self {
        Memory { data: [0u8; 65536] }
    }

    pub fn read_word(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    pub fn read_word_pair(&self, addr: u16) -> u16 {
        // creating a word pair from two words
        ((self.data[addr.wrapping_add(1) as usize] as u16) << 8) | (self.data[addr as usize] as u16)
    }

    pub fn write_word(&mut self, addr: u16, data: u8) {
        self.data[addr as usize] = data;
    }

    pub fn write_word_pair(&mut self, addr: u16, data: u16) {
        self.data[addr as usize] = (data & 0x00ff) as u8;
        self.data[addr.wrapping_add(1) as usize] = (data >> 8) as u8;
    }

    pub fn flush(&mut self) {
        self.data = [0u8; 65536];
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_byte() {
        let mut mem = Memory::new();
        mem.write_word(0xaaaa, 54);
        assert_eq!(mem.read_word(0xaaaa), 54);
    }

    #[test]
    fn test_word() {
        let mut mem = Memory::new();
        mem.write_word_pair(0xbbbb, 2543);
        assert_eq!(mem.read_word_pair(0xbbbb), 2543);
    }
}
