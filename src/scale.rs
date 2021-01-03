#[derive(Debug)]
pub struct Scale {
    pub min: i64,
    pub max: i64,
    pub ticks: Vec<i64>,
}
impl Scale {
    pub fn new(mut min: i64, mut max: i64) -> Self {
        debug_assert!(min < max);
        if max < min + 3 {
            max += 2;
            min -= 2;
        }
        if min > 0 && (max - min) * 4 > max {
            min = 0;
        }
        let l = ((max - min) as f64).log10().floor() as u32;
        let d = 10i64.pow(l);
        min = (min / d) * d;
        let mut tick = min;
        let mut ticks = vec![tick];
        loop {
            tick += d;
            ticks.push(tick);
            if tick > max {
                break;
            }
        }
        max = ticks[ticks.len() - 1];
        Self { min, max, ticks }
    }
    pub fn range(&self) -> i64 {
        self.max - self.min
    }
}
