from pathlib import Path
import os
import streamlit as st

def reformat_string(orig_str, orig_format, new_format):
    import re
    # 1. Identify the tag names
    tags = re.findall(r'\{(.+?)\}', orig_format)

    # 2. Escape the entire format string so dots/parens are treated as literals
    # This turns "{track} ({formt}).{ext}" into "\{track\}\ \(\{formt\}\)\.\{ext\}"
    escaped_format = re.escape(orig_format)

    # 3. Replace the escaped tags with (.+)
    # We use (.+) instead of (.+?) here so the extension captures everything 
    # until the end of the string.
    pattern = re.sub(r'\\\{.+?\\\}', r'(.+)', escaped_format)
    
    match = re.match(pattern, orig_str)
    if not match:
        raise KeyError("Pattern not found")

    # 4. Map tags to the captured values
    reformat_dict = dict(zip(tags, match.groups()))

    return new_format.format(**reformat_dict)

def rename_files(dir, pattern, new_pattern, dry_run=True):
    for file_path in dir.iterdir():

        # Only process files
        if file_path.is_dir():
            continue

        filename = file_path.name
        formatted = reformat_string(filename, pattern, new_pattern)
        destination = dir / formatted

        if dry_run:
            try: 
                
                st.info(formatted)
            except KeyError:
                st.error(f"{filename} not formatted")
                continue

        # Runs the actual renaming. First part is dry run
        else:
            try:
                os.rename(str(file_path), str(destination))
                st.success(f"Renamed: {filename}")
            except Exception as e:
                st.error(f"Error renaming {filename}: {e}")



st.title("File Renamer Interface")

with st.container(border=True):
    folder = st.text_input("Folder Path", value="/home/gaston/Downloads")
    
    col1, col2 = st.columns(2)
    with col1:
        pat = st.text_input("Source Pattern", value="{title}.{extension}")
    with col2:
        new_pat = st.text_input("Target Pattern", value="{title}_old.{extension}")

    dry_run_b = st.toggle("Dry Run", value=True)
    run_button = st.button("Execute Renaming", type="primary")


if run_button:
    st.subheader("Output")
    try:
        results = rename_files(Path(folder), pat, new_pat, dry_run_b)
        if results:
            st.code(results)
    except Exception as e:
        st.error(f"Error: {e}")

st.subheader("Preview")
if folder and pat and new_pat:
    try:
        results = rename_files(Path(folder), pat, new_pat, dry_run=True)
        
        if results:
            st.code(results)
    except Exception as e:
        st.error(f"Error: {e}")

