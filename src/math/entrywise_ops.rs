pub trait EntrywiseAdd<Rhs = Self> {
    type Output;

    fn entrywise_add(self, rhs: Rhs) -> Self::Output;
}

pub trait EntrywiseSub<Rhs = Self> {
    type Output;

    fn entrywise_sub(self, rhs: Rhs) -> Self::Output;
}

pub trait EntrywiseMul<Rhs = Self> {
    type Output;

    fn entrywise_mul(self, rhs: Rhs) -> Self::Output;
}

pub trait EntrywiseDiv<Rhs = Self> {
    type Output;

    fn entrywise_div(self, rhs: Rhs) -> Self::Output;
}

pub trait EntrywisePow<Rhs = Self> {
    type Output;

    fn entrywise_pow(self, rhs: Rhs) -> Self::Output;
}

pub trait EntrywiseSqrt {
    type Output;

    fn entrywise_sqrt(self) -> Self::Output;
}

pub trait EntrywiseInv {
    type Output;

    fn entrywise_inv(self) -> Self::Output;
}

pub trait EntrywiseEX {
    type Output;

    fn entrywise_e_x(self) -> Self::Output;
}
