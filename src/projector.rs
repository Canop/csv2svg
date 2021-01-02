
pub struct IntRect {
    pub left: i64,
    pub top: i64,
    pub width: i64,
    pub height: i64,
}
impl IntRect {
    pub fn new<I: Into<i64>>(left: I, top: I, width: I, height: I) -> Self {
        Self {
            left: left.into(),
            top: top.into(),
            width: width.into(),
            height: height.into(),
        }
    }
}

pub struct Projector {
    rx: f64,
    ry: f64,
    sx: i64,
    sy: i64,
    dx: i64,
    dy: i64,
}

impl Projector {
    pub fn new(src: &IntRect, dst: &IntRect) -> Self {
        let rx = (dst.width as f64) / (src.width as f64);
        let ry = (dst.height as f64) / (src.height as f64);
        let sx = src.left;
        let sy = src.top;
        let dx = dst.left;
        let dy = dst.top;
        Self { rx, ry, sx, dx, sy, dy }
    }
    pub fn project_x(&self, x: i64) -> i64 {
        self.dx + ( ((x - self.sx) as f64) * self.rx ) as i64
    }
    pub fn project_y(&self, y: i64) -> i64 {
        self.dy + ( ((y - self.sy) as f64) * self.ry ) as i64
    }
    pub fn project_point(&self, p: (i64, i64)) -> (i64, i64) {
        (
            self.project_x(p.0),
            self.project_y(p.1),
        )
    }
}
