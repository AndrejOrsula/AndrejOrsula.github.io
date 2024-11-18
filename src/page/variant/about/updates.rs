#[derive(Debug, Clone, Copy, PartialEq, typed_builder::TypedBuilder)]
pub struct UpdatesConfig {
    /// Font size for the updates heading
    #[builder(default = 36.0)]
    pub updates_font_size: f32,
    /// Radius of the circle for each update
    #[builder(default = 6.0)]
    pub circle_radius: f32,
    /// Width of the line connecting the circle to the text
    #[builder(default = 2.0)]
    pub line_width: f32,
    /// Vertical spacing between updates
    #[builder(default = 35.0)]
    pub vertical_spacing: f32,
    /// Length of the horizontal line connecting the circle to the text
    #[builder(default = 58.0)]
    pub horizontal_line_length: f32,
    /// Offset of the text from the horizontal line
    #[builder(default = 8.0)]
    pub text_line_offset: f32,
    /// Font size for the date text
    #[builder(default = 12.0)]
    pub font_size_date: f32,
    /// Font size for the title text
    #[builder(default = 17.0)]
    pub font_size_title: f32,
    /// Font size for the highlights text
    #[builder(default = 15.0)]
    pub font_size_highlight: f32,
}

impl Default for UpdatesConfig {
    fn default() -> Self {
        Self::builder().build()
    }
}

pub struct Updates {
    pub cfg: UpdatesConfig,
    entries: Vec<UpdateEntry>,
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct UpdateEntry {
    pub date: chrono::NaiveDate,
    pub title: String,
    pub highlights: String,
    pub description: String,
    pub url: String,
}

impl Updates {
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &UpdateEntry> {
        self.entries.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn parse(src: &str) -> Self {
        Self::parse_with_cfg(UpdatesConfig::default(), src)
    }

    pub fn parse_with_cfg(cfg: UpdatesConfig, src: &str) -> Self {
        let mut entries: Vec<UpdateEntry> = serde_yaml::from_str(src).unwrap();
        entries.sort_by(|a, b| b.date.cmp(&a.date));
        Self { cfg, entries }
    }

    pub fn show(&self, ui: &mut egui::Ui) {
        if self.is_empty() {
            return;
        }

        crate::utils::egui::centered_strong_heading_sized(
            ui,
            "Updates",
            self.cfg.updates_font_size,
        );

        ui.add_space(6.0 * ui.spacing().item_spacing.y);

        // Allocate space for the timeline to draw in
        let widget_rect = ui
            .allocate_space(egui::vec2(
                ui.available_width(),
                self.len() as f32 * self.cfg.vertical_spacing,
            ))
            .1;
        let painter = ui.painter();

        // Draw a single line from the top to the bottom of the timeline
        painter.line_segment(
            [
                egui::pos2(widget_rect.center().x, widget_rect.top()),
                egui::pos2(
                    widget_rect.center().x,
                    widget_rect.bottom() - self.cfg.vertical_spacing,
                ),
            ],
            egui::Stroke::new(self.cfg.line_width, ui.style().visuals.weak_text_color()),
        );

        for (index, update) in self.iter().enumerate() {
            // Determine layout: alternate sides (left/right) for text placement
            let is_left = index % 2 == 0;
            let side_sign = if is_left { -1.0 } else { 1.0 };
            let side_text_align = if is_left {
                egui::Align2::RIGHT_CENTER
            } else {
                egui::Align2::LEFT_CENTER
            };

            // Calculate circle position
            let y = widget_rect.top()
                + index as f32 * self.cfg.vertical_spacing
                + self.cfg.circle_radius;
            let circle_center = egui::pos2(widget_rect.center().x, y);

            // Calculate horizontal line position
            let horz_line_center = circle_center
                + egui::vec2(
                    side_sign * 0.5 * (self.cfg.horizontal_line_length + self.cfg.circle_radius),
                    0.0,
                );
            let horz_line_end =
                circle_center + side_sign * egui::vec2(self.cfg.horizontal_line_length, 0.0);

            // Calculate text position
            let entry_title_pos = egui::pos2(
                horz_line_end.x + side_sign * self.cfg.text_line_offset,
                circle_center.y,
            );
            let entry_highlight_pos = egui::pos2(
                entry_title_pos.x,
                entry_title_pos.y + ui.style().text_styles[&egui::TextStyle::Button].size,
            );
            let entry_date_pos =
                egui::pos2(horz_line_center.x, horz_line_center.y - self.cfg.line_width);

            // Draw the title, highlight and date text
            let rect_title = painter.text(
                entry_title_pos,
                side_text_align,
                &update.title,
                egui::FontId::proportional(self.cfg.font_size_title),
                ui.style().visuals.text_color(),
            );
            let rect_highlight = painter.text(
                entry_highlight_pos,
                side_text_align,
                &update.highlights,
                egui::FontId::proportional(self.cfg.font_size_highlight),
                ui.style().visuals.weak_text_color(),
            );
            let rect_date = painter.text(
                entry_date_pos,
                egui::Align2::CENTER_BOTTOM,
                update.date.format("%b %y").to_string(),
                egui::FontId::monospace(self.cfg.font_size_date),
                ui.style().visuals.weak_text_color(),
            );

            // Add hover overlay for the circle and text
            let rect_sense = rect_title
                .union(rect_highlight)
                .union(rect_date)
                .union(egui::Rect {
                    min: circle_center - egui::vec2(self.cfg.circle_radius, self.cfg.circle_radius),
                    max: circle_center + egui::vec2(self.cfg.circle_radius, self.cfg.circle_radius),
                });
            let res = {
                let res = ui
                    .interact(
                        rect_sense,
                        egui::Id::new(index),
                        egui::Sense::hover() | egui::Sense::click(),
                    )
                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                    .on_hover_text_at_pointer(format!(
                        "{}: {}",
                        update.date.format("%B %Y"),
                        update.description
                    ));
                crate::utils::egui::clickable_url(res, &update.url)
            };

            // Redraw text as highlighted if hovered
            if res.hovered() {
                painter.text(
                    entry_title_pos,
                    side_text_align,
                    &update.title,
                    egui::FontId::proportional(self.cfg.font_size_title),
                    ui.style().visuals.strong_text_color(),
                );
                painter.text(
                    entry_highlight_pos,
                    side_text_align,
                    &update.highlights,
                    egui::FontId::proportional(self.cfg.font_size_highlight),
                    ui.style().visuals.text_color(),
                );
                painter.text(
                    entry_date_pos,
                    egui::Align2::CENTER_BOTTOM,
                    update.date.format("%b %y").to_string(),
                    egui::FontId::monospace(self.cfg.font_size_date),
                    ui.style().visuals.text_color(),
                );
            }

            // Draw the circle and line for the entry
            let stroke_color = if res.hovered() {
                ui.style().visuals.text_color()
            } else {
                ui.style().visuals.weak_text_color()
            };
            let fill_color = if res.hovered() {
                ui.style().visuals.text_color()
            } else {
                ui.style().visuals.window_fill
            };
            painter.line_segment(
                [circle_center, horz_line_end],
                egui::Stroke::new(self.cfg.line_width, stroke_color),
            );
            painter.circle(
                circle_center,
                self.cfg.circle_radius,
                fill_color,
                egui::Stroke::new(self.cfg.line_width, stroke_color),
            );
        }
    }
}
