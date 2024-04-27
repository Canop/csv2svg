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
    hover: bool, // whether to build elements only visible on hover
}

impl Graph {
    pub fn new(tbl: Tbl) -> Self {
        let width = 800;
        let height = 500;
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
            hover: true,
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
            let label = element::Text::new(&y_seq.header)
                .set("x", x + 14)
                .set("y", y + 7)
                .set("fill", LEGEND_COLOR)
                .set("font-size", 10);
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
                .set("opacity", 0.4)
                .set("stroke-dasharray", "10 7")
                .set("d", data);
            group.append(path);
            let tick_label = element::Text::new(tick.to_string())
                .set("x", self.gr.left + self.gr.width + 2)
                .set("y", self.projector.project_y(*tick) + 2)
                .set("fill", TICK_LABEL_COLOR)
                .set("text-anchor", "left")
                .set("font-size", 8);
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
            vis: Visibility, // visibility when non hovered
        }
        let mut ticks = Vec::new();
        for idx in 0..x_seq.len() {
            let x = self.projector.project_x(x_seq.ival[idx].unwrap());
            ticks.push(Tick {
                idx,
                x,
                tx: x,
                vis: Visibility::Visible,
            });
        }
        // we improve the ticks position to avoid overlap
        let dots = ticks.iter().map(|t| t.x).collect();
        if let Some(dots) = unoverlap(dots, 10) {
            // we can show all ticks
            for (idx, dot) in dots.iter().enumerate() {
                if idx != 0 && idx != ticks.len() - 1 {
                    ticks[idx].tx = *dot;
                    ticks[idx].vis = Visibility::Faded;
                }
            }
        } else {
            // ticks will be shown only on hover
            for idx in 1..ticks.len() - 1 {
                ticks[idx].vis = Visibility::Invisible;
            }
        }
        // and we draw them
        for tick in ticks {
            let mut tick_group = node::element::Group::new().set("class", tick.vis.css_class());
            let data = element::path::Data::new()
                .move_to((tick.x, self.gr.top))
                .vertical_line_to(y);
            let hoverable_path = element::Path::new()
                .set("fill", "none")
                .set("stroke", TICK_LINE_COLOR)
                .set("stroke-width", 4)
                .set("opacity", 0)
                .set("d", data.clone());
            tick_group.append(hoverable_path);
            let path = element::Path::new()
                .set("fill", "none")
                .set("stroke", TICK_LINE_COLOR)
                .set("stroke-width", 1)
                .set("stroke-dasharray", "1 3")
                .set("opacity", 0.5)
                .set("d", data);
            tick_group.append(path);
            // the opt_group may be hidden or faded when not hovered, depending
            // on tick.vis
            let mut tick_opt_group = node::element::Group::new().set("class", "opt");
            let data = element::path::Data::new()
                .move_to((tick.x, y - 3))
                .vertical_line_to(y)
                .line_to((tick.tx, y + 7));
            let path = element::Path::new()
                .set("fill", "none")
                .set("stroke", TICK_LINE_COLOR)
                .set("stroke-width", 1)
                .set("opacity", 0.5)
                .set("d", data);
            tick_opt_group.append(path);
            let tick_label = element::Text::new(
                    x_seq.raw[tick.idx].as_ref().unwrap()
                )
                .set("x", tick.tx + 1)
                .set("y", y + 9)
                .set("fill", TICK_LABEL_COLOR)
                .set("text-anchor", "end")
                .set("font-size", 8)
                .set(
                    "transform",
                    format!("rotate(-45 {} {})", tick.tx + 1, y + 9),
                );
            tick_opt_group.append(tick_label);
            tick_group.append(tick_opt_group);
            group.append(tick_group);
        }
        group
    }
    fn curbs_group(&self) -> node::element::Group {
        let mut group = node::element::Group::new();
        let x_seq = &self.tbl.x_seq();
        for (seq_idx, y_seq) in self.tbl.y_seqs().enumerate() {
            let mut points_group = node::element::Group::new();
            let mut curve_data = element::path::Data::new();
            let mut started = false;
            for idx in 0..y_seq.len() {
                let p = (
                    x_seq.raw.get(idx),
                    x_seq.ival.get(idx),
                    y_seq.raw.get(idx),
                    y_seq.ival.get(idx),
                );
                if let (Some(Some(raw_x)), Some(Some(x)), Some(Some(raw_y)), Some(Some(y))) = p {
                    let (x, y) = self.projector.project_point((*x, *y));
                    let label = format!("{}, {}", raw_x, raw_y);
                    if started {
                        curve_data = curve_data.line_to((x, y));
                    } else {
                        curve_data = curve_data.move_to((x, y));
                        started = true;
                    }
                    if self.hover {
                        let mut point_group = node::element::Group::new().set("class", "inv");
                        let circle = node::element::Circle::new()
                            .set("fill", COLORS[seq_idx])
                            .set("cx", x)
                            .set("cy", y)
                            .set("opacity", 0)
                            .set("r", 8);
                        point_group.append(circle);
                        let mut point_opt_group = node::element::Group::new().set("class", "opt");
                        let point_label_shadow = element::Text::new(&label)
                            .set("x", x - 5)
                            .set("y", y - 10)
                            .set("stroke", "#222")
                            .set("stroke-width", 5)
                            .set("text-anchor", "end")
                            .set("font-size", 8);
                        point_opt_group.append(point_label_shadow);
                        let circle = node::element::Circle::new()
                            .set("fill", COLORS[seq_idx])
                            .set("cx", x)
                            .set("cy", y)
                            .set("r", 4);
                        point_opt_group.append(circle);
                        let point_label = element::Text::new(label)
                            .set("x", x - 5)
                            .set("y", y - 10)
                            .set("fill", TICK_LABEL_COLOR)
                            .set("text-anchor", "end")
                            .set("font-size", 8);
                        point_opt_group.append(point_label);
                        point_group.append(point_opt_group);
                        points_group.append(point_group);
                    }
                }
            }
            let curve = element::Path::new()
                .set("fill", "none")
                .set("stroke", COLORS[seq_idx])
                .set("stroke-width", 3)
                .set("opacity", 0.8)
                .set("stroke-linejoin", "round")
                .set("d", curve_data);
            group.append(curve);
            group.append(points_group);
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
            //.add(node::element::Style::new(SVG_STYLE))
            .add(self.graph_group())
    }
    pub fn write_svg<W: io::Write>(&self, mut writer: W) -> Result<()> {
        let document = self.build_svg();
        svg::write(&mut writer, &document)?;
        writer.write_all(b"\n")?;
        Ok(())
    }
}
