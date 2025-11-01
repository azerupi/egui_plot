use egui::{epaint::TextShape, Color32, Stroke};

use super::{LineStyle, PlotItem, PlotItemBase, PlotPoint};

/// The axis to which the span is attached.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpanAxis {
    /// The span is attached to the X axis.
    X,
    /// The span is attached to the Y axis.
    Y,
}

/// The position of the name within the span.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Position {
    /// The name is at the start of the span.
    Start,
    /// The name is at the end of the span.
    End,
}

/// A span on either the X or Y axis.
#[derive(Clone, Debug, PartialEq)]
pub struct Span {
    base: PlotItemBase,
    pub(super) start: f64,
    pub(super) end: f64,
    pub(super) axis: SpanAxis,
    pub(super) fill: Color32,
    pub(super) start_border_stroke: Stroke,
    pub(super) start_border_style: LineStyle,
    pub(super) end_border_stroke: Stroke,
    pub(super) end_border_style: LineStyle,
    pub(super) name_position: Position,
}

impl Span {
    pub fn new(
        name: impl Into<String>,
        start: impl Into<f64>,
        end: impl Into<f64>,
        axis: SpanAxis,
    ) -> Self {
        Self {
            base: PlotItemBase::new(name.into()),
            start: start.into(),
            end: end.into(),
            axis,
            fill: Color32::TRANSPARENT,
            start_border_stroke: Stroke::NONE,
            start_border_style: LineStyle::Solid,
            end_border_stroke: Stroke::NONE,
            end_border_style: LineStyle::Solid,
            name_position: Position::Start,
        }
    }

    /// Set the fill color of the span.
    #[inline]
    pub fn fill(mut self, color: impl Into<Color32>) -> Self {
        self.fill = color.into();
        self
    }

    /// Set the stroke of the border at the start of the span.
    #[inline]
    pub fn start_border_stroke(mut self, stroke: impl Into<Stroke>) -> Self {
        self.start_border_stroke = stroke.into();
        self
    }

    /// Set the style of the border at the start of the span.
    #[inline]
    pub fn start_border_style(mut self, style: LineStyle) -> Self {
        self.start_border_style = style;
        self
    }

    /// Set the stroke of the border at the end of the span.
    #[inline]
    pub fn end_border_stroke(mut self, stroke: impl Into<Stroke>) -> Self {
        self.end_border_stroke = stroke.into();
        self
    }

    /// Set the style of the border at the end of the span.
    #[inline]
    pub fn end_border_style(mut self, style: LineStyle) -> Self {
        self.end_border_style = style;
        self
    }

    /// Set the position of the name.
    #[inline]
    pub fn name_position(mut self, position: Position) -> Self {
        self.name_position = position;
        self
    }

    /// Name of this plot item.
    ///
    /// This name will show up in the plot legend, if legends are turned on.
    ///
    /// Setting the name via this method does not change the item's id, so you can use it to
    /// change the name dynamically between frames without losing the item's state. You should
    /// make sure the name passed to [`Self::new`] is unique and stable for each item, or
    /// set unique and stable ids explicitly via [`Self::id`].
    #[inline]
    pub fn name(mut self, name: impl ToString) -> Self {
        self.base.name = name.to_string();
        self
    }

    /// Highlight this plot item, typically by scaling it up.
    ///
    /// If false, the item may still be highlighted via user interaction.
    #[inline]
    pub fn highlight(mut self, highlight: bool) -> Self {
        self.base.highlight = highlight;
        self
    }

    /// Allowed hovering this item in the plot. Default: `true`.
    #[inline]
    pub fn allow_hover(mut self, hovering: bool) -> Self {
        self.base.allow_hover = hovering;
        self
    }

    /// Sets the id of this plot item.
    ///
    /// By default the id is determined from the name passed to [`Self::new`], but it can be
    /// explicitly set to a different value.
    #[inline]
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.base.id = id.into();
        self
    }
}

impl PlotItem for Span {
    fn shapes(&self, ui: &egui::Ui, transform: &super::PlotTransform, shapes: &mut Vec<egui::Shape>) {
        let bounds = transform.bounds();
        let (min, max) = match self.axis {
            SpanAxis::X => (
                PlotPoint::new(self.start, bounds.min[1]),
                PlotPoint::new(self.end, bounds.max[1]),
            ),
            SpanAxis::Y => (
                PlotPoint::new(bounds.min[0], self.start),
                PlotPoint::new(bounds.max[0], self.end),
            ),
        };

        let rect = egui::Rect::from_min_max(
            transform.position_from_point(&min),
            transform.position_from_point(&max),
        );

        if self.fill != Color32::TRANSPARENT {
            shapes.push(egui::Shape::rect_filled(rect, 0.0, self.fill));
        }

        if self.start_border_stroke != Stroke::NONE {
            let points = match self.axis {
                SpanAxis::X => vec![
                    transform.position_from_point(&PlotPoint::new(self.start, bounds.min[1])),
                    transform.position_from_point(&PlotPoint::new(self.start, bounds.max[1])),
                ],
                SpanAxis::Y => vec![
                    transform.position_from_point(&PlotPoint::new(bounds.min[0], self.start)),
                    transform.position_from_point(&PlotPoint::new(bounds.max[0], self.start)),
                ],
            };
            self.start_border_style.style_line(
                points,
                self.start_border_stroke.into(),
                self.base.highlight,
                shapes,
            );
        }

        if self.end_border_stroke != Stroke::NONE {
            let points = match self.axis {
                SpanAxis::X => vec![
                    transform.position_from_point(&PlotPoint::new(self.end, bounds.min[1])),
                    transform.position_from_point(&PlotPoint::new(self.end, bounds.max[1])),
                ],
                SpanAxis::Y => vec![
                    transform.position_from_point(&PlotPoint::new(bounds.min[0], self.end)),
                    transform.position_from_point(&PlotPoint::new(bounds.max[0], self.end)),
                ],
            };
            self.end_border_style.style_line(
                points,
                self.end_border_stroke.into(),
                self.base.highlight,
                shapes,
            );
        }

        if !self.base.name.is_empty() {
            let font_id = egui::TextStyle::Body.resolve(ui.style());
            let color = Color32::WHITE;

            let (max_width, pos) = match self.axis {
                SpanAxis::X => (rect.width(), rect.left_center()),
                SpanAxis::Y => (rect.height(), rect.center_top()),
            };

            let mut layout_job = egui::text::LayoutJob::simple(
                self.base.name.clone(),
                font_id.clone(),
                color,
                max_width,
            );
            layout_job.halign = egui::Align::LEFT;
            let galley = ui.fonts_mut(|f| f.layout_job(layout_job));

            if max_width > galley.size().x {
                let text_pos = if let Position::End = self.name_position {
                    match self.axis {
                        SpanAxis::X => rect.right_center() - egui::vec2(galley.size().x, 0.0),
                        SpanAxis::Y => rect.center_bottom() - egui::vec2(0.0, galley.size().y),
                    }
                } else {
                    pos
                };
                shapes.push(TextShape::new(text_pos, galley, color).into());
            } else {
                let truncated_name = format!("{}...", self.base.name.chars().next().unwrap());
                let galley = ui.fonts_mut(|f| {
                    f.layout_no_wrap(truncated_name, font_id, color)
                });
                if max_width > galley.size().x {
                    let text_pos = if let Position::End = self.name_position {
                        match self.axis {
                            SpanAxis::X => rect.right_center() - egui::vec2(galley.size().x, 0.0),
                            SpanAxis::Y => rect.center_bottom() - egui::vec2(0.0, galley.size().y),
                        }
                    } else {
                        pos
                    };
                    shapes.push(TextShape::new(text_pos, galley, color).into());
                }
            }
        }
    }

    fn initialize(&mut self, _x_range: std::ops::RangeInclusive<f64>) {
        // Noop
    }

    fn name(&self) -> &str {
        &self.base.name
    }

    fn color(&self) -> Color32 {
        self.fill
    }

    fn highlight(&mut self) {
        self.base.highlight = true;
    }

    fn highlighted(&self) -> bool {
        self.base.highlight
    }

    fn geometry(&self) -> super::PlotGeometry<'_> {
        super::PlotGeometry::None
    }

    fn bounds(&self) -> super::PlotBounds {
        let mut bounds = super::PlotBounds::NOTHING;
        match self.axis {
            SpanAxis::X => {
                bounds.min[0] = self.start;
                bounds.max[0] = self.end;
            }
            SpanAxis::Y => {
                bounds.min[1] = self.start;
                bounds.max[1] = self.end;
            }
        }
        bounds
    }

    fn base(&self) -> &PlotItemBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut PlotItemBase {
        &mut self.base
    }
}
