use eframe::egui;
use rfd::FileDialog;
mod utils;

fn main() -> eframe::Result<()>
{
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([550.0, 700.0])
            .with_min_inner_size([500.0, 650.0])
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "Rust File Renamer",
        options,
        Box::new(|_cc| Ok(Box::<RenamerApp>::default())),
    )
}

struct FileItem
{
    original: String,
    new_name: String,
    selected: bool,
    frozen: bool,
}

struct RenamerApp
{
    folder: String,
    pat: String,
    new_pat: String,
    items: Vec<FileItem>,
}

impl Default for RenamerApp
{
    fn default() -> Self
    {
        let mut app = Self {
            folder: "/home/gaston/Downloads".to_string(),
            pat: "{title}.{extension}".to_string(),
            new_pat: "{title}_old.{extension}".to_string(),
            items: vec![],
        };

        app.reload_files();
        app
    }
}

impl RenamerApp
{
    fn reload_files(&mut self)
    {
        let raw_results = utils::process_rename(&self.folder, &self.pat, &self.new_pat, true);
        
        let mut new_items = Vec::new();
        for line in raw_results {
            let parts: Vec<&str> = line.split("  ➜  ").collect();
            if parts.len() == 2 {
                let orig = parts[0].to_string();
                let target = parts[1].to_string();

                if let Some(existing) = self.items.iter().find(|i| i.original == orig) {
                    new_items.push(FileItem {
                        original: orig,
                        new_name: if existing.frozen { existing.new_name.clone() } else { target },
                        selected: existing.selected,
                        frozen: existing.frozen,
                    });
                } else {
                    new_items.push(FileItem {
                        original: orig,
                        new_name: target,
                        selected: true,
                        frozen: false,
                    });
                }
            }
        }
        self.items = new_items;
    }

    fn apply_renaming(&mut self)
    {
        // Executes the actual rename operation via utils
        utils::process_rename(&self.folder, &self.pat, &self.new_pat, false);
        self.reload_files();
    }
}

impl eframe::App for RenamerApp
{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame)
    {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("File Renamer");
            ui.add_space(10.0);

            // --- Top Section: Folder Selection ---
            ui.horizontal(|ui| {
                ui.label("Target Folder:");
                if ui.text_edit_singleline(&mut self.folder).changed()
                {
                    self.reload_files();
                }
                if ui.button("Browse").clicked()
                {
                    if let Some(path) = FileDialog::new().pick_folder()
                    {
                        self.folder = path.display().to_string();
                        self.reload_files();
                    }
                }
            });

            ui.separator();

            // --- Middle Section: Patterns ---
            egui::Grid::new("renamer_grid")
                .num_columns(2)
                .spacing([10.0, 10.0])
                .show(ui, |ui| {
                    ui.label("Current Pattern:");
                    ui.label("New Pattern Template:");
                    ui.end_row();

                    let spacing_total = 20.0;
                    let column_width = (ui.available_width() - spacing_total).max(200.0) / 2.0;

                    let resp = ui.add(egui::TextEdit::singleline(&mut self.pat).min_size(egui::vec2(column_width, 0.0)));
                    if resp.changed()
                    {
                        self.reload_files();
                    }

                    let resp = ui.add(egui::TextEdit::singleline(&mut self.new_pat).min_size(egui::vec2(column_width, 0.0)));
                    if resp.changed()
                    {
                        self.reload_files();
                    }

                    ui.end_row();
                });

            ui.add_space(10.0);

            // --- Action Buttons: Freeze & Run ---
            ui.horizontal(|ui| {
                if ui.button("❄ Freeze Selected").clicked()
                {
                    for item in &mut self.items {
                        if item.selected {
                            item.frozen = true;
                        }
                    }
                }

                if ui.button("🚀 Apply Renaming").clicked()
                {
                    self.apply_renaming();
                }
            });

            ui.add_space(10.0);
            ui.label("Dry Run Preview & Selection:");

            // --- Bottom Section: File List with Checkboxes ---
            egui::ScrollArea::vertical()
                .max_height(400.0)
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    egui::Grid::new("results_grid")
                        .num_columns(4)
                        .spacing([10.0, 6.0])
                        .striped(true)
                        .show(ui, |ui| {
                            for item in &mut self.items
                            {
                                // Column 1: Checkbox for selection
                                ui.checkbox(&mut item.selected, "");

                                // Column 2: Original Name
                                ui.monospace(&item.original);

                                // Column 3: Arrow indicator
                                ui.label("➜");

                                // Column 4: New Name (Color-coded if frozen)
                                if item.frozen
                                {
                                    ui.horizontal(|ui| {
                                        ui.monospace(egui::RichText::new(&item.new_name).color(egui::Color32::LIGHT_BLUE));
                                        ui.label("(Frozen)");
                                    });
                                }
                                else
                                {
                                    ui.monospace(egui::RichText::new(&item.new_name).color(egui::Color32::LIGHT_GREEN));
                                }

                                ui.end_row();
                            }
                        });
                });
        });
    }
}
