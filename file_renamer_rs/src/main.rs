use eframe::egui;
use rfd::FileDialog;
mod utils;

fn main() -> eframe::Result<()>
{
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 650.0])
            .with_min_inner_size([500.0, 650.0])
            .with_resizable(true), // Allow/Disallow dragging corners
        ..Default::default()
    };

    eframe::run_native(
        "Rust File Renamer",
        options,
        Box::new(|_cc| Ok(Box::<RenamerApp>::default())),
    )
}

struct RenamerApp
{
    folder: String,
    pat: String,
    new_pat: String,
    results: Vec<String>,
}

impl Default for RenamerApp
{
    fn default() -> Self
    {
        let mut app = Self {
            folder: "/home/gaston/Downloads".to_string(),
            pat: "{title}.{extension}".to_string(),
            new_pat: "{title}_old.{extension}".to_string(),
            results: vec![],
        };

        // Run once at the start
        app.run_logic(true);
        app
    }
}

impl RenamerApp
{
    fn run_logic(&mut self, dry_run: bool)
    {
        self.results = utils::process_rename(&self.folder, &self.pat, &self.new_pat, dry_run);
    }
}

impl eframe::App for RenamerApp
{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame)
    {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("File Renamer");
            ui.add_space(10.0); // Add a little breathing room

            // --- Top Section: Folder Selection ---
            ui.horizontal(|ui| {
                ui.label("Target Folder:");
                if ui.text_edit_singleline(&mut self.folder).changed()
                {
                    self.run_logic(true);
                }
                if ui.button("Browse").clicked()
                {
                    if let Some(path) = FileDialog::new().pick_folder()
                    {
                        self.folder = path.display().to_string();
                        self.run_logic(true);
                    }
                }
            });

            ui.separator();

            // --- Middle Section: The Grid ---
            // spacing([x, y]) defines the gap between columns and rows
            egui::Grid::new("renamer_grid")
                .num_columns(3) // Left widget, Vertical Line, Right widget
                .spacing([10.0, 10.0])
                .show(ui, |ui| {
                    // Row 1
                    ui.label("Current Pattern:");
                    ui.label("New Pattern Template:");
                    ui.end_row();

                    // Row 2. Defines column width by means of min textedit size
                    let spacing_total = 10.0 * 2.0;
                    let column_width = (484.0 - spacing_total) / 2.0;

                    let resp =
                        ui.add(egui::TextEdit::singleline(&mut self.pat).min_size(egui::vec2(column_width, 0.0)));

                    if resp.changed()
                    {
                        self.run_logic(true);
                    }

                    let resp =
                        ui.add(egui::TextEdit::singleline(&mut self.new_pat).min_size(egui::vec2(column_width, 0.0)));

                    if resp.changed()
                    {
                        self.run_logic(true);
                    }

                    ui.end_row();
                });

            ui.separator();

            // --- Bottom Section: Preview and Action ---
            if ui.button("🚀 Apply Renaming").clicked()
            {
                self.run_logic(false);
            }
            ui.add_space(10.0);

            ui.label("Dry Run Preview:");

            egui::ScrollArea::vertical()
                .max_height(650.0)
                .max_width(500.0)
                .auto_shrink([false; 2]) // Prevents the area from jumping around
                .show(ui, |ui| {
                    // Create a grid for the results
                    egui::Grid::new("results_grid")
                        .num_columns(3)
                        .spacing([10.0, 4.0]) // Wider horizontal spacing for clarity
                        .striped(true) // Adds alternating row colors like a real table
                        .show(ui, |ui| {
                            for line in &self.results
                            {
                                // Split the string back into parts
                                // (Assuming you kept the "  ➜  " separator)
                                let parts: Vec<&str> = line.split("  ➜  ").collect();

                                if parts.len() == 2
                                {
                                    // Column 1: Old Name
                                    ui.monospace(parts[0]);

                                    // Column 2: The Arrow
                                    ui.label(":");

                                    // Column 3: New Name (Highlighted)
                                    ui.group(|ui| {
                                        ui.monospace(egui::RichText::new(parts[1]).color(egui::Color32::LIGHT_GREEN));
                                    });
                                }
                                else
                                {
                                    // Handle the "Unable to rename" or error lines
                                    ui.label(egui::RichText::new(line).color(egui::Color32::LIGHT_RED));
                                    ui.label(""); // Empty cell for the arrow
                                    ui.label(""); // Empty cell for the new name
                                }
                                ui.end_row();
                            }
                        });
                });
        });
    }
}
