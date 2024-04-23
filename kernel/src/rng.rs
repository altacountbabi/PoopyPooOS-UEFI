struct Xoshiro256 {
    state: [u64; 4],
}

impl Xoshiro256 {
    fn new(seed: u64) -> Xoshiro256 {
        let mut state = [0; 4];
        // Initialize the state with the seed
        state[0] = seed;
        for i in 1..4 {
            state[i] = seed.wrapping_add(i as u64).rotate_left(17).wrapping_mul(0x9E3779B97F4A7C15);
        }
        Xoshiro256 { state }
    }

    fn next_u64(&mut self) -> u64 {
        let result_starstar = self.state[1].wrapping_mul(5).rotate_left(7).wrapping_mul(9);
        let t = self.state[1] << 17;

        let result = result_starstar.wrapping_mul(9);
        let state = &mut self.state;
        let state_0 = state[0];
        let state_2 = state[2];
        state[2] ^= state_0;
        state[3] ^= state[1];
        state[1] ^= state_2;
        state[0] ^= state[3];
        state[2] ^= t;
        state[3] = state[3].rotate_left(45);
        result
    }
}

pub fn rand(min: u64, max: u64) -> Option<u64> {
    if min >= max {
        return None;
    }

    let seed = 42; /* {
        let time = rtc::read_rtc_time();
        (time.0 + time.1 - time.2) as u64
    }; */

    let mut a = Xoshiro256::new(seed);

    let rng = a.next_u64();
    let range = max - min;

    let random_number = rng % range + min;

    Some(random_number)
}