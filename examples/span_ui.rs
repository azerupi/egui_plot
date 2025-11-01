use eframe::{
    egui::{self, CentralPanel, Context, TopBottomPanel},
    App, Frame,
};
use egui_plot::{Line, Plot, PlotPoints};

#[derive(Default)]
pub struct SpanDemo {
    span_x: [f64; 2],
    span_y: [f64; 2],
}

impl SpanDemo {
    pub fn new() -> Self {
        Self {
            span_x: [2.0, 4.0],
            span_y: [2.0, 4.0],
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        let sin = (0..1000).map(|i| {
            let x = i as f64 * 0.01;
            [x, x.sin()]
        });
        let line = Line::new(PlotPoints::from_iter(sin));

        Plot::new("span_demo")
            .show(ui, |plot_ui| {
                plot_ui.line(line);

                plot_ui.span(
                    egui_plot::Span::new("span_x", self.span_x[0], self.span_x[1], egui_plot::items::SpanAxis::X)
                        .fill(egui::Color32::from_rgb(255, 0, 0).with_alpha(50))
                        .start_border_stroke(egui::Stroke::new(1.0, egui::Color32::RED))
                        .end_border_stroke(egui::Stroke::new(1.0, egui::Color32::RED)),
                );
                plot_ui.span(
                    egui_plot::Span::new("span_y", self.span_y[0], self.span_y[1], egui_plot::items::SpanAxis::Y)
                        .fill(egui::Color32::from_rgb(0, 0, 255).with_alpha(50))
                        .start_border_stroke(egui::Stroke::new(1.0, egui::Color32::BLUE))
                        .end_border_stroke(egui::Stroke::new(1.0, egui::Color32::BLUE)),
                );
            })
            .response
    }
}

impl App for SpanDemo {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });
        CentralPanel::default().show(ctx, |ui| {
            self.ui(ui);
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Span Demo",
        options,
        Box::new(|_cc| Box::new(SpanDemo::new())),
    )
}
