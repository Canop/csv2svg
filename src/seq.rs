use {crate::*, anyhow::*, chrono::DateTime};

#[derive(Debug)]
pub struct Seq {
    pub header: String,
    pub nature: Nature,
    pub raw: Vec<Option<String>>,
    pub ival: Vec<Option<i64>>,
    pub min: i64,
    pub max: i64,
}

impl Seq {
    pub fn new(raw_col: RawCol) -> Result<Self> {
        let RawCol { header, cells: raw } = raw_col;
        let mut ival = vec![None; raw.len()];
        let mut nature = None;
        let mut min_max: Option<(i64, i64)> = None;
        for (x, cell) in raw.iter().enumerate() {
            if let Some(s) = cell {
                let v = match nature {
                    Some(Nature::Date(_)) => {
                        if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
                            dt.timestamp_millis()
                        } else if let Ok(int) = s.parse::<i64>() {
                            // we change the seq nature
                            nature = Some(Nature::Integer);
                            int
                        } else {
                            bail!("cell can't be used: {:?}", s);
                        }
                    }
                    Some(Nature::Integer) => {
                        if let Ok(int) = s.parse::<i64>() {
                            int
                        } else {
                            bail!("cell can't be used: {:?}", s);
                        }
                    }
                    None => {
                        if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
                            nature = Some(Nature::Date(dt.offset().clone()));
                            dt.timestamp_millis()
                        } else if let Ok(int) = s.parse::<i64>() {
                            nature = Some(Nature::Integer);
                            int
                        } else {
                            bail!("cell can't be used: {:?}", s);
                        }
                    }
                };
                ival[x] = Some(v);
                min_max = Some(min_max.map_or((v, v), |mm| (mm.0.min(v), mm.1.max(v))));
            }
        }
        nature
            .map(|nature| {
                let (min, max) = min_max.unwrap();
                Self {
                    header,
                    nature,
                    raw,
                    ival,
                    min,
                    max,
                }
            })
            .ok_or_else(||anyhow!("empty column"))
    }
    pub fn is_full_and_increasing(&self) -> bool {
        for idx in 1..self.ival.len() {
            match (self.ival.get(idx - 1), self.ival.get(idx)) {
                (Some(a), Some(b)) if a < b => {} // ok
                _ => {
                    return false;
                }
            }
        }
        true
    }
    pub fn len(&self) -> usize {
        self.raw.len()
    }
}
