use std::{collections::HashMap, fs, path::PathBuf};

use regex::{Regex, escape};

pub fn reformat_string(orig_str: &str, orig_format: &str, new_format: &str) -> Result<String, String>
{
    // 1. Identify the tag names (e.g., "tit", "ext")
    // We use a regex to find everything inside { }
    let tag_regex = Regex::new(r"\{(.+?)\}").unwrap();
    let tags: Vec<&str> = tag_regex
        .captures_iter(orig_format)
        .map(|cap| cap.get(1).unwrap().as_str())
        .collect();

    // 2. Escape the entire format string
    let escaped_format = escape(orig_format);

    let escaped_tag_regex = Regex::new(r"\\\{.+?\\\}").unwrap();
    let pattern_str = escaped_tag_regex.replace_all(&escaped_format, "(.+)").to_string();

    // Create the regex anchor it to start/end for accuracy
    let pattern = Regex::new(&format!("^{}$", pattern_str)).map_err(|_| "Invalid Pattern Generated".to_string())?;

    let caps = pattern
        .captures(orig_str)
        .ok_or_else(|| "Pattern not found".to_string())?;

    // 4. Map tags to captured values
    let mut reformat_dict = HashMap::new();
    for (i, tag) in tags.iter().enumerate()
    {
        // i+1 because capture 0 is the full match
        if let Some(val) = caps.get(i + 1)
        {
            reformat_dict.insert(*tag, val.as_str());
        }
    }

    // 5. Final replacement in new_format
    let mut result = new_format.to_string();
    for (tag, value) in reformat_dict
    {
        let placeholder = format!("{{{}}}", tag);
        result = result.replace(&placeholder, value);
    }

    Ok(result)
}

pub fn get_sorted_files(base_path: &PathBuf) -> Result<Vec<fs::DirEntry>, String>
{
    // Check if the directory exists/is readable
    let entries = match fs::read_dir(&base_path)
    {
        Ok(e) => e,
        Err(e) => return Err(format!("Invalid folder path: {}", e)),
    };

    // Prepare the list (Flatten and Sort)
    let mut file_entries: Vec<fs::DirEntry> = entries.flatten().collect();
    file_entries.sort_by_key(|entry| entry.file_name());

    return Ok(file_entries);
}

pub fn process_rename(
    folder: &str, 
    pat: &str, 
    new_pat: &str, 
    dry_run: bool) -> Vec<String>
    {
        let mut results = Vec::new();
        let base_path = PathBuf::from(folder);
        let file_entries = match get_sorted_files(&base_path)
        {
            Ok(entries) => entries,
            Err(e) =>
            {
                results.push(e);
                return results;
            }
        };

        for entry in file_entries
        {
            if entry.path().is_dir()
            {
                continue;
            }

            let name = entry.file_name().to_string_lossy().into_owned();

            let new_name = match reformat_string(&name, pat, new_pat)
            {
                Ok(e) => e,
                Err(_) =>
                {
                    results.push(format!("Unable to rename {}", name));
                    continue;
                }
            };

            if dry_run
            {
                results.push(format!("{}  ➜  {}", name, new_name));
            }
            else
            {
                let old_path = base_path.join(&name);
                let new_path = base_path.join(&new_name);

                match fs::rename(&old_path, &new_path)
                {
                    Ok(_) => results.push(format!("RENAMED: {} > {}", name, new_name)),
                    Err(e) => results.push(format!("ERROR: {}: {}", name, e)),
                }
            }
        }
        results
    }