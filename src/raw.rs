use {
    anyhow::*,
    std::{
        io::Read,
    },
};


#[derive(Debug)]
pub struct RawCol {
    pub header: String,
    pub cells: Vec<Option<String>>,
}
impl RawCol {
    fn new(header: String) -> Self {
        Self {
            header,
            cells: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct RawTbl {
    /// by construction, all cols are guaranteed to have the same number
    /// of items (the "row count")
    pub cols: Vec<RawCol>,
}
impl RawTbl {
    pub fn read<R: Read>(r: R) -> Result<Self> {
        let mut csv_reader = csv::Reader::from_reader(r);
        let mut cols = Vec::new();
        for header in csv_reader.headers()? {
            cols.push(RawCol::new(header.to_string()));
        }
        if cols.is_empty() {
            bail!("empty table");
        }
        for record in csv_reader.records() {
            let record = record?;
            let mut cells = record.iter();
            for col in cols.iter_mut() {
                let cell = cells.next();
                col.cells.push(
                    cell.filter(|&s| s.chars().any(|c| !c.is_whitespace()))
                        .map(|s| s.to_string())
                );
            }
        }
        Ok(Self { cols })
    }
    /// return the number of rows
    pub fn row_count(&self) -> usize {
        self.cols[0].cells.len()
    }
}
