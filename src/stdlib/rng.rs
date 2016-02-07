const BIT_MASK: u64 = 0xffff_ffff_0000_0000;

pub struct Rng {
    state: u64
}

impl Rng {
    pub fn new() -> Rng {
        Rng {
            state: BIT_MASK
        }
    }

    pub fn seed_rnd(&mut self, seed: i32) {
        self.state = (seed as u64) | BIT_MASK;
    }

    pub fn rand(&mut self, low: i32, high: i32) -> i32 {
        // xorshift* prng
        let mut x = self.state;
        x ^= x >> 12;
        x ^= x >> 25;
        x ^= x >> 27;
        x *= 2685821657736338717;
        self.state = x;

        let range = high - low + 1;

        ((x as i32) % range) + low
    }

    pub fn rnd(&mut self, low: f32, high: f32) -> f32 {
        (self.rand(0, 100) as f32 / 100.0) * (high - low) + low
    }
}
