
pub(crate) struct Xor128 {
    x: u32,
}

impl Xor128 {
    pub fn new(seed: u32) -> Self {
        let mut ret = Xor128 { x: 2463534242 };
        if 0 < seed {
            ret.x ^= seed;
            ret.nexti();
        }
        ret.nexti();
        ret
    }

    pub fn nexti(&mut self) -> u32 {
        // T = (I + L^a)(I + R^b)(I + L^c)
        // a = 13, b = 17, c = 5
        let x1 = self.x ^ (self.x << 13);
        let x2 = x1 ^ (x1 >> 17);
        self.x = x2 ^ (x2 << 5);
        self.x
    }

    pub fn next(&mut self) -> f64 {
        self.nexti() as f64 / 0xffffffffu32 as f64
    }
}
