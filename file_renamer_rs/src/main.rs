use eframe::egui;
use rfd::FileDialog;
mod utils;

fn main() -> eframe::Result<()>
{
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([550.0, 600.0])
            .with_min_inner_size([500.0, 500.0])
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "File Renamer",
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
                        selected: false,
                        frozen: false,
                    });
                }
            }
        }
        self.items = new_items;
    }

    fn apply_renaming(&mut self)
    {
        utils::process_rename(&self.folder, &self.pat, &self.new_pat, false);
        self.reload_files();
    }
}

impl eframe::App for RenamerApp
{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame)
    {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(4.0);

            // --- Top Input Fields (Grid Layout) ---
            egui::Grid::new("top_inputs")
                .num_columns(2)
                .spacing([10.0, 8.0])
                .show(ui, |ui| {
                    ui.label("Target Folder");
                    ui.horizontal(|ui| {
                        let available_w = ui.available_width();
                        ui.add(egui::TextEdit::singleline(&mut self.folder).desired_width(available_w - 75.0));
                        if ui.button("Browse").clicked()
                        {
                            if let Some(path) = FileDialog::new().pick_folder()
                            {
                                self.folder = path.display().to_string();
                                self.reload_files();
                            }
                        }
                    });
                    ui.end_row();

                    ui.label("Original Pattern");
                    let resp = ui.add(egui::TextEdit::singleline(&mut self.pat).desired_width(ui.available_width()));
                    if resp.changed()
                    {
                        self.reload_files();
                    }
                    ui.end_row();

                    ui.label("New Pattern");
                    let resp = ui.add(egui::TextEdit::singleline(&mut self.new_pat).desired_width(ui.available_width()));
                    if resp.changed()
                    {
                        self.reload_files();
                    }
                    ui.end_row();
                });

            ui.add_space(8.0);

            // --- Bottom Action Buttons (Pinned to bottom) ---
            let button_height = 30.0;
            let spacing = 8.0;
            let available_height = ui.available_height() - button_height - spacing;

            // --- Middle Table Section inside a framed group ---
            egui::Frame::group(ui.style())
                .inner_margin(4.0)
                .show(ui, |ui| {
                    egui::ScrollArea::vertical()
                        .max_height(available_height)
                        .auto_shrink([false; 2])
                        .show(ui, |ui| {
                            egui::Grid::new("results_grid")
                                .num_columns(3)
                                .spacing([12.0, 6.0])
                                .striped(true)
                                .show(ui, |ui| {
                                    // Header Row
                                    ui.label(egui::RichText::new("Old Value").strong());
                                    ui.label(egui::RichText::new("New Value").strong());
                                    ui.label(""); // spacer or select header
                                    ui.end_row();

                                    ui.separator();
                                    ui.separator();
                                    ui.separator();
                                    ui.end_row();

                                    for item in &mut self.items
                                    {
                                        ui.monospace(&item.original);

                                        if item.frozen
                                        {
                                            ui.horizontal(|ui| {
                                                ui.monospace(egui::RichText::new(&item.new_name).color(egui::Color32::LIGHT_BLUE));
                                                ui.label("(Frozen)");
                                            });
                                        }
                                        else
                                        {
                                            ui.monospace(egui::RichText::new(&item.new_name));
                                        }

                                        ui.checkbox(&mut item.selected, "");
                                        ui.end_row();
                                    }
                                });
                        });
                });

            ui.add_space(spacing);

            // --- Bottom Buttons Row ---
            ui.horizontal(|ui| {
                let btn_width = (ui.available_width() - 10.0) / 2.0;

                if ui.add_sized([btn_width, button_height], egui::Button::new("Mark Selected as Final")).clicked()
                {
                    for item in &mut self.items {
                        if item.selected {
                            item.frozen = true;
                        }
                    }
                }

                if ui.add_sized([btn_width, button_height], egui::Button::new("Apply New Values")).clicked()
                {
                    self.apply_renaming();
                }
            });
        });
    }
}
