
use {
    num_traits::Num,
};

pub struct Rect<N: Num + Copy> {
    pub left: N,
    pub top: N,
    pub width: N,
    pub height: N,
}
impl<N: Num + Copy> Rect<N> {
    pub fn new<I: Into<N>>(left: I, top: I, width: I, height: I) -> Self {
        Self {
            left: left.into(),
            top: top.into(),
            width: width.into(),
            height: height.into(),
        }
    }
    pub fn bottom(&self) -> N {
        self.top + self.height
    }
}

pub type IntRect = Rect<i64>;
