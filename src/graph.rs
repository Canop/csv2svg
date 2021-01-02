use {
    anyhow::*,
    crate::*,
    std::{
        io,
    },
    svg::{
        Document,
        node::{self, *},
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
        let (top, right, bottom, left) = (40, 50, 60, 50);
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
                .set("font-size", 12)
                .add(node::Text::new(tick.to_string()));
            group.append(tick_label);
        }
        group
    }
    fn curbs_group(&self) -> node::element::Group {
        let mut group = node::element::Group::new();
        let x_seq = &self.tbl.x_seq();
        for (seq_idx, y_seq) in self.tbl.y_seqs().enumerate() {
            let mut points = x_seq.ival.iter()
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
        let mut graph = node::element::Group::new();
        graph.append(self.y_scale_group());
        graph.append(self.curbs_group());
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
        writer.write(b"\n")?;
        Ok(())
    }
}

fn points_data(points: &mut dyn Iterator<Item=(i64, i64)>) -> element::path::Data {
    let mut data = element::path::Data::new()
        .move_to(points.next().unwrap());
    for point in points {
        data = data.line_to(point);
    }
    data
}
