use rand::RngCore;

pub struct MockRng {
    values: Vec<u64>,
    current: usize,
}

impl MockRng {
    pub fn new(values: Vec<u64>) -> Self {
        MockRng { values, current: 0 }
    }
}

impl RngCore for MockRng {
    fn next_u32(&mut self) -> u32 {
        let val = self.values[self.current] as u32;
        self.current += 1;
        val
    }

    fn next_u64(&mut self) -> u64 {
        let val = self.values[self.current];
        self.current += 1;
        val
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        for byte in dest.iter_mut() {
            *byte = self.next_u32() as u8; // Or any other way to fill bytes
        }
    }
}