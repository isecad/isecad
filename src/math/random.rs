#![allow(dead_code)]

use std::collections::hash_map::*;
use std::fmt::*;
use std::hash::*;
use std::num::*;

pub const N: usize = 624;

const NM1: usize = N - 1;
const M: usize = 397;
const MM1: usize = M - 1;
const MMN: usize = ((M as isize) - (N as isize)) as usize;
const NMM: usize = N - M;
const U: Wrapping<u32> = Wrapping(2147483648);
const L: Wrapping<u32> = Wrapping(U.0 - 1);

const I: u32 = 19650218;
const J: u32 = 5489;

const A: Wrapping<u32> = Wrapping(4294901760);
const B: Wrapping<u32> = Wrapping(1812433253);
const C: Wrapping<u32> = Wrapping(65535);
const D: Wrapping<u32> = Wrapping(1664525);
const E: Wrapping<u32> = Wrapping(1566083941);
const F: [Wrapping<u32>; 2] = [Wrapping(0), Wrapping(2567483615)];
const G: Wrapping<u32> = Wrapping(2636928640);
const H: Wrapping<u32> = Wrapping(4022730752);

const X: f32 = 4294967296.0;

#[derive(Clone)]
pub struct Random {
    pub mt: [Wrapping<u32>; N],
    pub i: usize,
    pub last_normal: f32,
}

impl Debug for Random {
    /// Instead of showing contents of the [`Random::mt`], it shows its hash.
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        let mut hash = DefaultHasher::new();

        self.mt.hash(&mut hash);

        let hash = hash.finish();

        fmt.debug_struct("Random")
            .field("mt", &hash)
            .field("i", &self.i)
            .field("last_normal", &self.last_normal)
            .finish()
    }
}

impl Random {
    pub fn new(seed: u32) -> Random {
        let mut random: Random = Random {
            mt: [Wrapping(0); N],
            i: N,
            last_normal: 0.0,
        };

        let mut i: usize = 1;

        random.init(I);

        for _ in (1..=N).rev() {
            let a: Wrapping<u32> = random.mt[i - 1] ^ (random.mt[i - 1] >> 30);

            random.mt[i] = (random.mt[i] ^ (((((a & A) >> 16) * D) << 16) + (a & C) * D)) + Wrapping(seed);

            i += 1;

            if i >= N {
                random.mt[0] = random.mt[NM1];

                i = 1;
            }
        }

        for _ in (1..N).rev() {
            let a: Wrapping<u32> = random.mt[i - 1] ^ (random.mt[i - 1] >> 30);

            random.mt[i] = (random.mt[i] ^ (((((a & A) >> 16) * E) << 16) + (a & C) * E)) - Wrapping(i as u32);

            i += 1;

            if i >= N {
                random.mt[0] = random.mt[NM1];

                i = 1;
            }
        }

        random.mt[0] = U;

        random
    }

    fn init(&mut self, seed: u32) {
        self.mt[0] = Wrapping(seed);

        for i in 1..N {
            let a: Wrapping<u32> = self.mt[i - 1] ^ (self.mt[i - 1] >> 30);

            self.mt[i] = ((((a & A) >> 16) * B) << 16) + (a & C) * B + Wrapping(i as u32);
        }

        self.i = N;
    }

    fn u32(&mut self) -> u32 {
        let mut i: usize = self.i;

        if i >= N {
            if i > N {
                self.init(J);
            }

            for j in 0..NMM {
                let a: Wrapping<u32> = (self.mt[j] & U) | (self.mt[j + 1] & L);

                self.mt[j] = self.mt[j + M] ^ (a >> 1) ^ F[(a.0 & 1) as usize];
            }

            for j in NMM..NM1 {
                let a: Wrapping<u32> = (self.mt[j] & U) | (self.mt[j + 1] & L);

                self.mt[j] = self.mt[j + MMN] ^ (a >> 1) ^ F[(a.0 & 1) as usize];
            }

            let a: Wrapping<u32> = (self.mt[NM1] & U) | (self.mt[0] & L);

            self.mt[NM1] = self.mt[MM1] ^ (a >> 1) ^ F[(a.0 & 1) as usize];

            i = 0;
        }

        let mut a: Wrapping<u32> = self.mt[i];

        a ^= a >> 11;
        a ^= (a << 7) & G;
        a ^= (a << 15) & H;
        a ^= a >> 18;

        self.i = i + 1;

        a.0
    }

    pub fn f32(&mut self) -> f32 {
        self.u32() as f32 / X
    }

    pub fn uniform(&mut self, min: f32, max: f32) -> f32 {
        min + self.f32() * (max - min)
    }

    pub fn normal(&mut self, min: f32, max: f32) -> f32 {
        let mut current_normal: f32 = self.last_normal;

        self.last_normal = 0.0;

        if current_normal == 0.0 {
            let a: f32 = self.f32() * 2.0 * std::f32::consts::PI;
            let b: f32 = f32::sqrt(-2.0 * f32::ln(1.0 - self.f32()));

            self.last_normal = f32::sin(a) * b;
            current_normal = f32::cos(a) * b;
        }

        min + current_normal * max
    }
}
