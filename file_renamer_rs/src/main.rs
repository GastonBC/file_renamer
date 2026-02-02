use eframe::egui;
use regex::{Regex, escape};
use rfd::FileDialog;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

fn reformat_string(orig_str: &str, orig_format: &str, new_format: &str) -> Result<String, String> {
    // 1. Identify the tag names (e.g., "tit", "ext")
    // We use a regex to find everything inside { }
    let tag_regex = Regex::new(r"\{(.+?)\}").unwrap();
    let tags: Vec<&str> = tag_regex
        .captures_iter(orig_format)
        .map(|cap| cap.get(1).unwrap().as_str())
        .collect();

    // 2. Escape the entire format string
    let escaped_format = escape(orig_format);

    // 3. Replace the escaped tags with (.+)
    // In Rust, escaped braces look like \{ or \{
    // 3. Replace the escaped tags with (.+)
    // Use a new regex that specifically looks for escaped braces \{...\}
    let escaped_tag_regex = Regex::new(r"\\\{.+?\\\}").unwrap();
    let pattern_str = escaped_tag_regex
        .replace_all(&escaped_format, "(.+)")
        .to_string();

    // Create the regex anchor it to start/end for accuracy
    let pattern = Regex::new(&format!("^{}$", pattern_str))
        .map_err(|_| "Invalid Pattern Generated".to_string())?;

    let caps = pattern
        .captures(orig_str)
        .ok_or_else(|| "Pattern not found".to_string())?;

    // 4. Map tags to captured values
    let mut reformat_dict = HashMap::new();
    for (i, tag) in tags.iter().enumerate() {
        // i+1 because capture 0 is the full match
        if let Some(val) = caps.get(i + 1) {
            reformat_dict.insert(*tag, val.as_str());
        }
    }

    // 5. Final replacement in new_format
    let mut result = new_format.to_string();
    for (tag, value) in reformat_dict {
        let placeholder = format!("{{{}}}", tag);
        result = result.replace(&placeholder, value);
    }

    Ok(result)
}

fn main() -> eframe::Result<()> {
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

struct RenamerApp {
    folder: String,
    pat: String,
    new_pat: String,
    results: Vec<String>,
}

impl Default for RenamerApp {
    fn default() -> Self {
        Self {
            folder: "/home/gaston/Downloads".to_string(),
            pat: "{title}.{extension}".to_string(),
            new_pat: "{title}_old.{extension}".to_string(),
            results: vec![],
        }
    }
}

impl RenamerApp {
    fn run_logic(&mut self, dry_run: bool) {
        self.results.clear();
        let path = PathBuf::from(&self.folder);

        // 1. Guard: Check if the directory exists/is readable
        let entries = match fs::read_dir(&path) {
            Ok(e) => e,
            Err(_) => {
                self.results.push("Invalid folder path".to_string());
                return;
            }
        };

        // Prepare the list (Flatten and Sort)
        let mut file_entries: Vec<_> = entries.flatten().collect();
        file_entries.sort_by_key(|entry| entry.file_name());

        for entry in file_entries {
            if entry.path().is_dir() {
                continue;
            }

            let name = entry.file_name().to_string_lossy().into_owned();

            let new_name = match reformat_string(&name, &self.pat, &self.new_pat) {
                Ok(e) => e,
                Err(_) => {
                    self.results.push(format!("Unable to rename {}", name));
                    return;
                }
            };

            if dry_run {
                self.results.push(format!("{}  ➜  {}", name, new_name));
            } else {
                let old_path = path.join(&name);
                let new_path = path.join(&new_name);

                match fs::rename(&old_path, &new_path) {
                    Ok(_) => self.results.push(format!("✅ {} ➜ {}", name, new_name)),
                    Err(e) => self.results.push(format!("❌ Error {}: {}", name, e)),
                }
            }
        }
    }
}

// if !dry_run { self.results.push("Renaming complete.".to_string()); }

impl eframe::App for RenamerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("File Renamer");
            ui.add_space(10.0); // Add a little breathing room

            // --- Top Section: Folder Selection ---
            ui.horizontal(|ui| {
                ui.label("Target Folder:");
                ui.text_edit_singleline(&mut self.folder);
                if ui.button("Browse").clicked() {
                    if let Some(path) = FileDialog::new().pick_folder() {
                        self.folder = path.display().to_string();
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

                    ui.add(
                        egui::TextEdit::singleline(&mut self.pat)
                            .min_size(egui::vec2(column_width, 0.0)),
                    );
                    ui.add(
                        egui::TextEdit::singleline(&mut self.new_pat)
                            .min_size(egui::vec2(column_width, 0.0)),
                    );
                    ui.end_row();
                });

            ui.separator();

            // --- Bottom Section: Preview and Action ---
            if ui.button("🚀 Apply Renaming").clicked() {
                self.run_logic(false);
            }
            ui.add_space(10.0);
            self.run_logic(true); // Continuous dry run

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
                            for line in &self.results {
                                // Split the string back into parts
                                // (Assuming you kept the "  ➜  " separator)
                                let parts: Vec<&str> = line.split("  ➜  ").collect();

                                if parts.len() == 2 {
                                    // Column 1: Old Name
                                    ui.monospace(parts[0]);

                                    // Column 2: The Arrow
                                    ui.label(":");

                                    // Column 3: New Name (Highlighted)
                                    ui.group(|ui| {
                                        ui.monospace(
                                            egui::RichText::new(parts[1])
                                                .color(egui::Color32::LIGHT_GREEN),
                                        );
                                    });
                                } else {
                                    // Handle the "Unable to rename" or error lines
                                    ui.label(
                                        egui::RichText::new(line).color(egui::Color32::LIGHT_RED),
                                    );
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
