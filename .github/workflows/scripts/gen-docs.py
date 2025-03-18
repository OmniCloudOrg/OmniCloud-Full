#!/usr/bin/env python3
"""
GitHub Wiki Generator

This script generates a GitHub wiki from markdown files found in the docs and/or notes directories.
It creates an organized index on the homepage and preserves the directory structure.
"""

import os
import re
import shutil
from pathlib import Path

def sanitize_filename(title):
    """Convert a title to a valid filename."""
    if not title:
        return ""
    # Convert to lowercase, replace non-alphanumeric with dashes
    sanitized = re.sub(r"[^a-z0-9]", "-", title.lower())
    # Replace multiple dashes with a single dash
    sanitized = re.sub(r"-+", "-", sanitized)
    # Remove leading and trailing dashes
    return sanitized.strip("-")

def extract_title(content, default_name):
    """Extract title from markdown content or use default."""
    title_match = re.search(r"^# (.+)$", content, re.MULTILINE)
    if title_match:
        return title_match.group(1).strip()
    return default_name.replace("-", " ").replace("_", " ").title()

def process_internal_links(content, path_to_wiki):
    """Process markdown links to work correctly in wiki."""
    # Function to replace links in markdown
    def replace_link(match):
        link_text = match.group(1)
        link_target = match.group(2)
        
        # Remove .md extension if present
        if link_target.endswith('.md'):
            link_target = link_target[:-3]
            
        # Check if this link target is in our mapping
        if link_target in path_to_wiki:
            return f'[{link_text}]({path_to_wiki[link_target]})'
        
        # If it's a relative path that might be in our mapping
        for path, wiki_name in path_to_wiki.items():
            if path.endswith(link_target) or link_target.endswith(path):
                return f'[{link_text}]({wiki_name})'
                
        # Otherwise return the original link
        return match.group(0)
    
    # Find all markdown links and process them
    pattern = r'\[([^\]]+)\]\(([^)]+)\)'
    return re.sub(pattern, replace_link, content)

def main():
    """Main function to generate wiki content."""
    wiki_dir = Path("wiki")
    source_dirs = []
    total_files = 0
    processed_dirs = 0

    print("Starting Wiki Generator")

    # Find source directories
    for dir_name in ["docs", "notes", "Docs", "Notes"]:
        if os.path.isdir(dir_name):
            source_dirs.append(dir_name)
            processed_dirs += 1
            print(f"Found source directory: {dir_name}")

    if not source_dirs:
        print("No docs or notes directories found. Exiting.")
        return

    # Start creating Home.md with a prettier format
    with open(wiki_dir / "Home.md", "w") as home_file:
        home_file.write("# Wiki Index\n\n")
        home_file.write("*This wiki is auto-generated from repository markdown files*\n\n")
        
        # Add a table of contents
        home_file.write("## Table of Contents\n\n")
        
        # Create a table for top-level directories
        home_file.write("| Category | Description |\n")
        home_file.write("|:---------|:------------|\n")
        
        for source_dir in source_dirs:
            dir_name = source_dir.capitalize()
            home_file.write(f"| **[{dir_name}](#{dir_name.lower()})** | Documentation from the `{source_dir}` directory |\n")
        
        home_file.write("\n---\n\n")
        
        # Process each source directory with enhanced formatting
        for source_dir in source_dirs:
            print(f"Processing directory: {source_dir}")
            home_file.write(f"## {source_dir.capitalize()}\n\n")
            
            # Add a brief description based on README if available
            readme_path = os.path.join(source_dir, "README.md")
            if os.path.exists(readme_path):
                try:
                    with open(readme_path, "r", encoding="utf-8") as readme_file:
                        content = readme_file.read()
                        # Try to extract a brief description from the beginning
                        desc_match = re.search(r'^# .+?\n\n(.+?)(?=\n\n|\Z)', content, re.DOTALL)
                        if desc_match:
                            home_file.write(f"{desc_match.group(1).strip()}\n\n")
                except Exception:
                    pass  # Continue if readme can't be read
            
            # Get all markdown files with their paths relative to the source dir
            md_files = []
            for root, _, files in os.walk(source_dir):
                for file in files:
                    if file.endswith(".md") and not file.startswith((".", "_")):
                        rel_path = os.path.relpath(root, source_dir)
                        md_files.append((os.path.join(root, file), rel_path if rel_path != "." else ""))
            
            # Create a dictionary to store all files by their paths
            path_to_wiki = {}
            
            # First pass - collect file information and create mappings
            for file_path, rel_path in sorted(md_files):
                try:
                    # Read the file content
                    with open(file_path, "r", encoding="utf-8") as md_file:
                        content = md_file.read()
                    
                    # Extract title
                    title = extract_title(content, os.path.basename(file_path)[:-3])
                    
                    # Create wiki filename
                    wiki_filename = sanitize_filename(title)
                    if not wiki_filename:
                        wiki_filename = sanitize_filename(os.path.basename(file_path)[:-3])
                    
                    # Store mapping for link processing
                    original_path = os.path.relpath(file_path)
                    path_to_wiki[original_path] = wiki_filename
                    path_to_wiki[os.path.basename(file_path)] = wiki_filename
                    
                    total_files += 1
                    print(f"Indexed: {file_path}")
                    
                except Exception as e:
                    print(f"Error indexing {file_path}: {str(e)}")
            
            # Second pass - process content and fix links
            for file_path, rel_path in sorted(md_files):
                try:
                    # Read the file content
                    with open(file_path, "r", encoding="utf-8") as md_file:
                        content = md_file.read()
                    
                    # Process internal links using our file mapping
                    content = process_internal_links(content, path_to_wiki)
                    
                    # Extract title
                    title = extract_title(content, os.path.basename(file_path)[:-3])
                    
                    # Get wiki filename
                    wiki_filename = sanitize_filename(title)
                    if not wiki_filename:
                        wiki_filename = sanitize_filename(os.path.basename(file_path)[:-3])
                    
                    # Copy content to wiki
                    with open(wiki_dir / f"{wiki_filename}.md", "w", encoding="utf-8") as wiki_file:
                        wiki_file.write(content)
                    
                    print(f"Processed: {file_path} → {wiki_filename}.md")
                    
                except Exception as e:
                    print(f"Error updating content for {file_path}: {str(e)}")
            
            # Get all directories for structure
            directories = set()
            for _, rel_path in md_files:
                if rel_path:  # Skip root level
                    # Add all intermediate directories
                    parts = rel_path.split(os.sep)
                    for i in range(len(parts)):
                        directories.add(os.sep.join(parts[:i+1]))
            
            # Add directories to index first
            for dir_path in sorted(directories):
                # Calculate indent level (number of path separators)
                indent_level = dir_path.count(os.sep) + 1
                indent = "  " * indent_level
                dir_name = os.path.basename(dir_path)
                home_file.write(f"{indent}- **{dir_name}**\n")
            
                # Create a dictionary to store all files by their paths
            path_to_wiki = {}
            
            # Process each markdown file
            for file_path, rel_path in sorted(md_files):
                try:
                    # Read the file content
                    with open(file_path, "r", encoding="utf-8") as md_file:
                        content = md_file.read()
                    
                    # Extract title
                    title = extract_title(content, os.path.basename(file_path)[:-3])
                    
                    # Create wiki filename
                    wiki_filename = sanitize_filename(title)
                    if not wiki_filename:
                        wiki_filename = sanitize_filename(os.path.basename(file_path)[:-3])
                    
                    # Store mapping for link processing
                    original_path = os.path.relpath(file_path)
                    path_to_wiki[original_path] = wiki_filename
                    path_to_wiki[os.path.basename(file_path)] = wiki_filename
                    
                    # Calculate indent level
                    indent_level = 1  # Start with base indent
                    if rel_path:
                        indent_level += rel_path.count(os.sep) + 1
                    
                    # Add to index
                    indent = "  " * indent_level
                    home_file.write(f"{indent}- [{title}]({wiki_filename})\n")
                    
                    total_files += 1
                    print(f"Processed: {file_path}")
                    
                except Exception as e:
                    print(f"Error processing {file_path}: {str(e)}")
            
            # Now process content and fix links
            for file_path, rel_path in sorted(md_files):
                try:
                    # Read the file content
                    with open(file_path, "r", encoding="utf-8") as md_file:
                        content = md_file.read()
                    
                    # Process internal links using our file mapping
                    content = process_internal_links(content, path_to_wiki)
                    
                    # Extract title
                    title = extract_title(content, os.path.basename(file_path)[:-3])
                    
                    # Get wiki filename
                    wiki_filename = sanitize_filename(title)
                    if not wiki_filename:
                        wiki_filename = sanitize_filename(os.path.basename(file_path)[:-3])
                    
                    # Copy content to wiki
                    with open(wiki_dir / f"{wiki_filename}.md", "w", encoding="utf-8") as wiki_file:
                        wiki_file.write(content)
                    
                except Exception as e:
                    print(f"Error updating content for {file_path}: {str(e)}")
            
            home_file.write("\n")  # Add spacing after section
    
    # Update stats at the top
    with open(wiki_dir / "Home.md", "r") as f:
        content = f.read()
    
    # Replace the placeholder stats line
    content = re.sub(
        r"\*This wiki is auto-generated from repository markdown files\*",
        f"*Generated from {total_files} markdown files across {processed_dirs} directories*",
        content
    )
    
    with open(wiki_dir / "Home.md", "w") as f:
        f.write(content)

    # Add custom styling to improve appearance
    with open(wiki_dir / "_Sidebar.md", "w") as sidebar_file:
        sidebar_file.write("# Wiki\n\n")
        
        # Add a cleaner sidebar with emoji icons
        for source_dir in source_dirs:
            # Choose an appropriate icon based on directory name
            icon = "📄"
            if source_dir.lower() == "docs":
                icon = "📚"
            elif source_dir.lower() == "notes":
                icon = "📝"
            
            sidebar_file.write(f"## {icon} {source_dir.capitalize()}\n\n")
            
            # Add top-level directories first (sorted)
            top_dirs = []
            for dir_path in sorted(directories):
                if os.path.dirname(dir_path) == '':  # Only top-level dirs
                    dir_name = os.path.basename(dir_path)
                    top_dirs.append(dir_name)
            
            for dir_name in sorted(top_dirs):
                # Choose an appropriate icon based on directory name
                dir_icon = "📁"
                if dir_name.lower() in ["cli", "command", "commands"]:
                    dir_icon = "⌨️"
                elif dir_name.lower() in ["admin", "administration"]:
                    dir_icon = "🔧"
                elif dir_name.lower() in ["network", "networking"]:
                    dir_icon = "🌐"
                elif dir_name.lower() in ["storage", "data"]:
                    dir_icon = "💾"
                elif dir_name.lower() in ["integration", "api"]:
                    dir_icon = "🔌"
                
                sidebar_file.write(f"- {dir_icon} **{dir_name}**\n")
            
            # Find top-level files
            top_level_files = []
            for file_path, rel_path in md_files:
                if source_dir in file_path and not rel_path:  # Only top-level files
                    try:
                        with open(file_path, "r", encoding="utf-8") as f:
                            content = f.read()
                        title = extract_title(content, os.path.basename(file_path)[:-3])
                        wiki_filename = sanitize_filename(title)
                        if not wiki_filename:
                            wiki_filename = sanitize_filename(os.path.basename(file_path)[:-3])
                        top_level_files.append((title, wiki_filename))
                    except Exception as e:
                        print(f"Error processing file for sidebar: {file_path}: {str(e)}")
            
            # Add top-level files (sorted by title)
            for title, wiki_filename in sorted(top_level_files):
                doc_icon = "📄"
                if "readme" in title.lower():
                    doc_icon = "ℹ️"
                sidebar_file.write(f"- {doc_icon} [{title}]({wiki_filename})\n")
            
            sidebar_file.write("\n")


    print(f"Wiki generation complete. Processed {total_files} files from {processed_dirs} directories.")

if __name__ == "__main__":
    main()