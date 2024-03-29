use {
    crate::*,
    anyhow::{bail, Result},
};

/// this table is garanteed to contain at least 2 sequences.
#[derive(Debug)]
pub struct Tbl {
    seqs: Vec<Seq>,
}

impl Tbl {
    pub fn from_raw(mut raw_tbl: RawTbl) -> Result<Self> {
        if raw_tbl.row_count() < 2 {
            bail!("two rows needed for a graph");
        }
        let mut seqs = Vec::new();
        for (col_idx, raw_col) in raw_tbl.cols.drain(..).enumerate() {
            match Seq::new(raw_col) {
                Ok(seq) => {
                    seqs.push(seq);
                }
                Err(e) => {
                    info!("column {} can't be used: {}", col_idx, e);
                }
            }
        }
        if seqs.len() < 2 {
            bail!("not enough usable columns")
        }
        Ok(Self { seqs })
    }
    pub fn from_seqs(seqs: Vec<Seq>) -> Result<Self> {
        if seqs.len() < 2 {
            bail!("not enough columns");
        }
        if !seqs[0].is_full_and_increasing() {
            bail!("first sequence must be full and increasing");
        }
        Ok(Self { seqs })
    }
    pub fn seqs_count(&self) -> usize {
        self.seqs.len()
    }
    pub fn y_seqs_count(&self) -> usize {
        self.seqs.len() - 1
    }
    pub fn dim(&self) -> (usize, usize) {
        (self.seqs_count(), self.seqs[0].len())
    }
    pub fn x_seq(&self) -> &Seq {
        &self.seqs[0]
    }
    pub fn y_seqs(&self) -> std::iter::Skip<std::slice::Iter<'_, seq::Seq>> {
        self.seqs.iter().skip(1)
    }
    pub fn y_min_max(&self) -> (i64, i64) {
        let mut y_seqs = self.y_seqs();
        let first_y = y_seqs.next().unwrap();
        y_seqs.fold((first_y.min, first_y.max), |(min, max), seq| {
            (min.min(seq.min), max.max(seq.max))
        })
    }
}
