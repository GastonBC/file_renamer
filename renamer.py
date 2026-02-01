from pathlib import Path
import os
import streamlit as st
import utils

def rename_files(dir, pattern, new_pattern, dry_run=True):
    for file_path in dir.iterdir():

        # Only process files
        if file_path.is_dir():
            continue

        filename = file_path.name

        if dry_run:
            try: 
                formatted = utils.reformat_string(filename, pattern, new_pattern)
                destination = dir / formatted
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

    dry_run = st.toggle("Dry Run", value=True)
    run_button = st.button("Execute Renaming", type="primary")

if run_button:
    st.subheader("Output")
    # Calling your function
    try:
        # Assuming your function prints or returns results
        results = rename_files(Path(folder), pat, new_pat, dry_run)
        
        if results:
            st.code(results)
        else:
            st.success("Task completed")
    except Exception as e:
        st.error(f"Error: {e}")