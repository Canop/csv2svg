use {
    crate::*,
    anyhow::*,
    std::io,
    svg::{
        node::{self, *},
        Document,
    },
};

pub struct Graph {
    tbl: Tbl,
    width: usize,
    height: usize,
    gr: IntRect,
    projector: Projector,
    scale: Scale,
}

impl Graph {
    pub fn new(tbl: Tbl) -> Self {
        let width = 600;
        let height = 400;
        let x_seq = &tbl.x_seq();
        let (y_min, y_max) = tbl.y_min_max();
        let scale = Scale::new(y_min, y_max);
        let sr = IntRect::new(x_seq.min, scale.max, x_seq.max - x_seq.min, -scale.range());
        let (top, right, bottom, left) = (26, 50, 85, 70);
        let gr = IntRect::new(
            left,
            top,
            width as i64 - (left + right),
            height as i64 - (top + bottom),
        );
        let projector = Projector::new(&sr, &gr);
        Self {
            tbl,
            width,
            height,
            gr,
            projector,
            scale,
        }
    }
    fn legend_group(&self) -> node::element::Group {
        let mut group = node::element::Group::new();
        let mut x = 0;
        let y = 10;
        let w = self.width / self.tbl.y_seqs_count();
        for (seq_idx, y_seq) in self.tbl.y_seqs().enumerate() {
            let square = node::element::Rectangle::new()
                .set("x", x + 4)
                .set("y", 10)
                .set("width", 8)
                .set("height", 8)
                .set("fill", COLORS[seq_idx]);
            group.append(square);
            let label = element::Text::new()
                .set("x", x + 14)
                .set("y", y + 7)
                .set("fill", LEGEND_COLOR)
                .set("font-size", 10)
                .add(node::Text::new(&y_seq.header));
            group.append(label);
            x += w;
        }
        group
    }
    fn y_scale_group(&self) -> node::element::Group {
        let mut group = node::element::Group::new();
        let x_seq = &self.tbl.x_seq();
        for tick in &self.scale.ticks {
            let data = element::path::Data::new()
                .move_to(self.projector.project_point((x_seq.min, *tick)))
                .horizontal_line_to(self.projector.project_x(x_seq.max));
            let path = element::Path::new()
                .set("fill", "none")
                .set("stroke", TICK_LINE_COLOR)
                .set("stroke-width", 1)
                .set("opacity", 0.5)
                .set("stroke-dasharray", "10 7")
                .set("d", data);
            group.append(path);
            let tick_label = element::Text::new()
                .set("x", self.gr.left + self.gr.width + 2)
                .set("y", self.projector.project_y(*tick) + 2)
                .set("fill", TICK_LABEL_COLOR)
                .set("text-anchor", "left")
                .set("font-size", 10)
                .add(node::Text::new(tick.to_string()));
            group.append(tick_label);
        }
        group
    }
    fn x_ticks_group(&self) -> node::element::Group {
        let mut group = node::element::Group::new();
        let x_seq = &self.tbl.x_seq();
        let y = self.gr.bottom();
        struct Tick {
            idx: usize,
            x: i64,
            tx: i64,
            skip_label: bool,
        }
        let mut ticks = Vec::new();
        for idx in 0..x_seq.len() {
            let x = self.projector.project_x(x_seq.ival[idx].unwrap());
            let tx = x;
            ticks.push(Tick {
                idx,
                x,
                tx,
                skip_label: false,
            });
        }
        // we improve the ticks position to avoid overlap
        // and we draw them
        let l = ticks.len();
        let m = 10;
        let (mut a, mut b) = (self.gr.left + m, self.gr.right() - m);
        for i in 1..ticks.len() / 2 {
            let tx = ticks[i].tx.max(a);
            if tx < b {
                a = tx + m;
                ticks[i].tx = tx;
            } else {
                ticks[i].skip_label = true;
            }
            let idx = l - i - 1;
            let tx = ticks[idx].tx.min(b);
            if tx > a {
                b = tx - m;
                ticks[idx].tx = tx;
            } else {
                ticks[idx].skip_label = true;
            }
        }
        for (idx, tick) in ticks.iter().enumerate() {
            if ticks.len() < 20 || idx == 0 || idx == ticks.len() - 1 {
                let data = element::path::Data::new()
                    .move_to((tick.x, self.gr.top))
                    .vertical_line_to(y);
                let path = element::Path::new()
                    .set("fill", "none")
                    .set("stroke", TICK_LINE_COLOR)
                    .set("stroke-width", 1)
                    .set("stroke-dasharray", "1 3")
                    .set("opacity", 0.5)
                    .set("d", data);
                group.append(path);
            }
            if !tick.skip_label {
                let tick_label = element::Text::new()
                    .set("x", tick.tx)
                    .set("y", y + 8)
                    .set("fill", TICK_LABEL_COLOR)
                    .set("text-anchor", "end")
                    .set("font-size", 8)
                    .set("transform", format!("rotate(-45 {} {})", tick.tx, y + 8))
                    .add(node::Text::new(x_seq.raw[tick.idx].as_ref().unwrap()));
                group.append(tick_label);
                let data = element::path::Data::new()
                    .move_to((tick.x, y - 3))
                    .vertical_line_to(y)
                    .line_to((tick.tx, y + 5));
                let path = element::Path::new()
                    .set("fill", "none")
                    .set("stroke", TICK_LINE_COLOR)
                    .set("stroke-width", 1)
                    .set("opacity", 0.5)
                    .set("d", data);
                group.append(path);
            }
        }
        group
    }
    fn curbs_group(&self) -> node::element::Group {
        let mut group = node::element::Group::new();
        let x_seq = &self.tbl.x_seq();
        for (seq_idx, y_seq) in self.tbl.y_seqs().enumerate() {
            let mut points = x_seq
                .ival
                .iter()
                .map(|ox| ox.unwrap()) // x sequence is guaranteed without hole
                .zip(y_seq.ival.iter())
                .filter_map(|(x, oy)| oy.map(|y| (x, y)))
                .map(|p| self.projector.project_point(p));
            let path = element::Path::new()
                .set("fill", "none")
                .set("stroke", COLORS[seq_idx])
                .set("stroke-width", 2)
                .set("opacity", 0.8)
                .set("stroke-linejoin", "round")
                .set("d", points_data(&mut points));
            group.append(path);
        }
        group
    }
    fn graph_group(&self) -> node::element::Group {
        let mut graph =
            node::element::Group::new().set("font-family", "Arial, Helvetica, sans-serif");
        graph.append(self.y_scale_group());
        graph.append(self.x_ticks_group());
        graph.append(self.curbs_group());
        graph.append(self.legend_group());
        graph
    }
    pub fn build_svg(&self) -> Document {
        let (width, height) = (self.width as i64, self.height as i64);
        Document::new()
            .set("viewBox", (0, 0, width, height))
            .set("style", DOCUMENT_STYLE)
            .add(self.graph_group())
    }
    pub fn write_svg<W: io::Write>(&self, mut writer: W) -> Result<()> {
        let document = self.build_svg();
        svg::write(&mut writer, &document)?;
        writer.write_all(b"\n")?;
        Ok(())
    }
}

fn points_data(points: &mut dyn Iterator<Item = (i64, i64)>) -> element::path::Data {
    let mut data = element::path::Data::new().move_to(points.next().unwrap());
    for point in points {
        data = data.line_to(point);
    }
    data
}
