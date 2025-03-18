#!/usr/bin/env python3
"""
GitHub Wiki Generator

This script generates a GitHub wiki from markdown files found in the docs and/or notes directories.
It creates an organized index with a clean file tree structure on the homepage and preserves
the directory hierarchy in the sidebar.
"""

import os
import re
import shutil
import datetime
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

def get_file_last_modified(file_path):
    """Get the last modified date of a file."""
    try:
        mtime = os.path.getmtime(file_path)
        return datetime.datetime.fromtimestamp(mtime)
    except:
        return datetime.datetime.now()

def main():
    """Main function to generate wiki content."""
    wiki_dir = Path("wiki")
    source_dirs = []
    total_files = 0
    processed_dirs = 0
    recently_updated = []  # Track recently updated files

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

    # Start creating Home.md focusing on the file tree
    with open(wiki_dir / "Home.md", "w") as home_file:
        home_file.write("# Wiki Index\n\n")
        home_file.write("*This wiki is auto-generated from repository markdown files*\n\n")
        
        # Create a table for top-level directories
        home_file.write("| Category | Description |\n")
        home_file.write("|:---------|:------------|\n")
        
        for source_dir in source_dirs:
            dir_name = source_dir.capitalize()
            home_file.write(f"| **[{dir_name}](#{dir_name.lower()})** | Documentation from `{source_dir}` directory |\n")
        
        home_file.write("\n---\n\n")
        
        # Process each source directory focusing on file structure
        for source_dir in source_dirs:
            print(f"Processing directory: {source_dir}")
            home_file.write(f"<h2 id='{source_dir.lower()}'>{source_dir.capitalize()}</h2>\n\n")
            
            # Get all markdown files with their paths relative to the source dir
            md_files = []
            
            for root, dirs, files in os.walk(source_dir):
                rel_path = os.path.relpath(root, source_dir)
                if rel_path == '.':
                    rel_path = ''
                
                # Skip hidden directories
                dirs[:] = [d for d in dirs if not d.startswith('.')]
                
                for file in files:
                    if file.endswith('.md') and not file.startswith(('.', '_')):
                        full_path = os.path.join(root, file)
                        md_files.append((full_path, rel_path))
                        
                        # Track file modification time for recently updated
                        last_modified = get_file_last_modified(full_path)
                        recently_updated.append((full_path, rel_path, last_modified))
            
            # Process directory structure - build hierarchy
            directory_hierarchy = {}
            # First, identify all directories
            for _, rel_path in md_files:
                if rel_path:
                    parts = rel_path.split(os.sep)
                    current_path = ""
                    for i, part in enumerate(parts):
                        parent_path = current_path
                        if current_path:
                            current_path = os.path.join(current_path, part)
                        else:
                            current_path = part
                            
                        if current_path not in directory_hierarchy:
                            directory_hierarchy[current_path] = {
                                'name': part,
                                'parent': parent_path,
                                'children': [],
                                'files': []
                            }
                        
                        # Add as child to parent
                        if parent_path and parent_path in directory_hierarchy and current_path not in directory_hierarchy[parent_path]['children']:
                            directory_hierarchy[parent_path]['children'].append(current_path)
            
            # Add files to their directories
            for file_path, rel_path in md_files:
                filename = os.path.basename(file_path)
                if rel_path in directory_hierarchy:
                    directory_hierarchy[rel_path]['files'].append((file_path, filename))
            
            # Get all directories
            directories = set(directory_hierarchy.keys())
            
            # First, get top-level directories (no parent or parent is empty)
            top_level_dirs = []
            for dir_path in sorted(directory_hierarchy.keys()):
                if not directory_hierarchy[dir_path]['parent']:
                    top_level_dirs.append(dir_path)
            
            # Create expanded directory tree - always visible by default
            home_file.write("### Directory Structure\n\n")
            
            # Build directory tree
            home_file.write("```\n")
            home_file.write(f"{source_dir}/\n")
            
            # Find README files for directories
            dir_to_readme = {}  # Map directories to their README files
            dir_to_wiki = {}    # Map directories to their wiki files
            
            for file_path, rel_path in md_files:
                filename = os.path.basename(file_path)
                if filename.lower() == "readme.md":
                    try:
                        with open(file_path, "r", encoding="utf-8") as f:
                            content = f.read()
                        title = extract_title(content, os.path.basename(file_path)[:-3])
                        wiki_filename = sanitize_filename(title)
                        if not wiki_filename:
                            wiki_filename = sanitize_filename(os.path.basename(file_path)[:-3])
                        
                        if rel_path:
                            # This is a README for a subdirectory
                            dir_to_readme[rel_path] = file_path
                            dir_to_wiki[rel_path] = wiki_filename
                        else:
                            # Root README
                            dir_to_readme[source_dir] = file_path
                            dir_to_wiki[source_dir] = wiki_filename
                    except Exception as e:
                        print(f"Error processing README mapping: {file_path}: {str(e)}")
            
            # Helper function to recursively print directory tree
            def print_directory_tree(dir_path, prefix=""):
                dir_info = directory_hierarchy[dir_path]
                
                # Print files in this directory
                files = [f[1] for f in dir_info['files']]
                sorted_files = sorted(files)
                
                for i, file in enumerate(sorted_files):
                    is_last_file = (i == len(sorted_files) - 1) and not dir_info['children']
                    file_prefix = "└── " if is_last_file else "├── "
                    home_file.write(f"{prefix}{file_prefix}{file}\n")
                
                # Print subdirectories
                children = sorted(dir_info['children'])
                for i, child in enumerate(children):
                    is_last = i == len(children) - 1
                    child_name = directory_hierarchy[child]['name']
                    
                    if is_last:
                        home_file.write(f"{prefix}└── {child_name}/\n")
                        # Recursively print the child directory
                        print_directory_tree(child, prefix + "    ")
                    else:
                        home_file.write(f"{prefix}├── {child_name}/\n")
                        # Recursively print the child directory
                        print_directory_tree(child, prefix + "│   ")
            
            # Print top-level directories
            for i, dir_path in enumerate(sorted(top_level_dirs)):
                dir_name = directory_hierarchy[dir_path]['name']
                is_last = i == len(top_level_dirs) - 1
                
                if is_last:
                    home_file.write(f"└── {dir_name}/\n")
                    print_directory_tree(dir_path, "    ")
                else:
                    home_file.write(f"├── {dir_name}/\n")
                    print_directory_tree(dir_path, "│   ")
            
            home_file.write("```\n\n")
            
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
            
            # Create an interactive file browser with links
            home_file.write("### Document Browser\n\n")
            
            # Recursive function to print directory structure with links
            def print_linked_tree(dir_path, indent=0):
                indent_str = "  " * indent
                dir_name = directory_hierarchy[dir_path]['name'] if dir_path in directory_hierarchy else os.path.basename(dir_path)
                
                # Check if directory has a README to link to
                if dir_path in dir_to_wiki:
                    wiki_link = dir_to_wiki[dir_path]
                    home_file.write(f"{indent_str}- **[{dir_name}]({wiki_link})**\n")
                else:
                    home_file.write(f"{indent_str}- **{dir_name}**\n")
                
                # Print files in this directory
                if dir_path in directory_hierarchy:
                    # Sort files to ensure predictable order
                    files = sorted(directory_hierarchy[dir_path]['files'], key=lambda x: x[1].lower())
                    
                    for file_path, filename in files:
                        if filename.lower() != "readme.md":  # Skip READMEs as they're linked to the directory
                            try:
                                with open(file_path, "r", encoding="utf-8") as f:
                                    content = f.read()
                                title = extract_title(content, os.path.basename(file_path)[:-3])
                                wiki_filename = sanitize_filename(title)
                                if not wiki_filename:
                                    wiki_filename = sanitize_filename(os.path.basename(file_path)[:-3])
                                
                                home_file.write(f"{indent_str}  - [{title}]({wiki_filename})\n")
                            except Exception as e:
                                print(f"Error creating link for file: {file_path}: {str(e)}")
                    
                    # Print subdirectories recursively
                    for child in sorted(directory_hierarchy[dir_path]['children']):
                        print_linked_tree(child, indent + 1)
            
            # Print each top-level directory
            for dir_path in sorted(top_level_dirs):
                print_linked_tree(dir_path)
            
            home_file.write("\n<hr>\n\n")  # Add separator between sections
    
    # Add custom styling for better appearance
    with open(wiki_dir / "custom-styling.md", "w") as style_file:
        style_file.write("""# Custom Styling

<!-- This file adds some custom styling to make the wiki more pleasant to read -->

<style>
.content-section {
  margin-bottom: 30px;
  padding: 15px;
  border-radius: 5px;
  background-color: #f8f9fa;
  border: 1px solid #eaecef;
}

.markdown-body table {
  width: 100%;
  border-collapse: collapse;
}

.markdown-body table th {
  background-color: #eaecef;
  text-align: left;
  padding: 8px;
}

.markdown-body table td {
  padding: 8px;
  border-top: 1px solid #eaecef;
}

.markdown-body table tr:nth-child(2n) {
  background-color: #f6f8fa;
}

.markdown-body hr {
  height: 2px;
  background-color: #dfe2e5;
  margin: 30px 0;
}

h2 {
  padding-bottom: 8px;
  border-bottom: 1px solid #eaecef;
  margin-top: 24px;
}

h3 {
  color: #24292e;
}

h4 {
  color: #24292e;
  font-size: 16px;
  margin-top: 24px;
  margin-bottom: 16px;
  border-left: 3px solid #0366d6;
  padding-left: 8px;
}

.recently-updated {
  background-color: #f1f8ff;
  padding: 10px;
  border-radius: 5px;
  margin-top: 10px;
  border-left: 3px solid #0366d6;
}

.recently-updated h4 {
  margin-top: 0;
  border-left: none;
  padding-left: 0;
}
</style>

[Return to Home](Home)
""")
    
    # Create sidebar with proper directory hierarchy
    with open(wiki_dir / "_Sidebar.md", "w") as sidebar_file:
        sidebar_file.write("# Wiki\n\n")
        
        # Add recently updated section to sidebar
        sidebar_file.write("## 🕒 Recently Updated\n\n")
        # Sort by most recent first
        recently_updated.sort(key=lambda x: x[2], reverse=True)
        # Show the 5 most recent files
        for i, (file_path, rel_path, timestamp) in enumerate(recently_updated[:5]):
            try:
                with open(file_path, "r", encoding="utf-8") as f:
                    content = f.read()
                title = extract_title(content, os.path.basename(file_path)[:-3])
                wiki_filename = sanitize_filename(title)
                if not wiki_filename:
                    wiki_filename = sanitize_filename(os.path.basename(file_path)[:-3])
                
                # Format the date
                date_str = timestamp.strftime("%Y-%m-%d")
                sidebar_file.write(f"- [{title}]({wiki_filename}) *({date_str})*\n")
            except Exception as e:
                print(f"Error adding recent file to sidebar: {file_path}: {str(e)}")
        
        sidebar_file.write("\n")
        
        # Add a cleaner sidebar with emoji icons
        for source_dir in source_dirs:
            # Choose an appropriate icon based on directory name
            icon = "📄"
            if source_dir.lower() == "docs":
                icon = "📚"
            elif source_dir.lower() == "notes":
                icon = "📝"
            
            sidebar_file.write(f"## {icon} {source_dir.capitalize()}\n\n")
            
            # Find README files for directories
            dir_to_readme = {}  # Map directories to their README files
            
            # First find README files for directories
            for file_path, rel_path in md_files:
                filename = os.path.basename(file_path)
                if filename.lower() == "readme.md":
                    if rel_path:
                        # This is a README for a subdirectory
                        dir_to_readme[rel_path] = file_path
                    else:
                        # Root README
                        dir_to_readme[source_dir] = file_path
            
            # Recursively add child directories and their files to the sidebar
            def add_child_directories(parent_path, indent="  "):
                if parent_path not in directory_hierarchy:
                    return
                
                # Get all children of this directory
                children = sorted(directory_hierarchy[parent_path]['children'], 
                                 key=lambda x: directory_hierarchy[x]['name'].lower())
                
                for child_path in children:
                    child_name = directory_hierarchy[child_path]['name']
                    
                    # Choose an appropriate icon
                    dir_icon = "📁"
                    if child_name.lower() in ["cli", "command", "commands"]:
                        dir_icon = "⌨️"
                    elif child_name.lower() in ["admin", "administration"]:
                        dir_icon = "🔧"
                    elif child_name.lower() in ["network", "networking"]:
                        dir_icon = "🌐"
                    elif child_name.lower() in ["storage", "data"]:
                        dir_icon = "💾"
                    elif child_name.lower() in ["integration", "api"]:
                        dir_icon = "🔌"
                    
                    # Check if this directory has a README
                    if child_path in dir_to_readme:
                        readme_path = dir_to_readme[child_path]
                        try:
                            with open(readme_path, "r", encoding="utf-8") as f:
                                content = f.read()
                            title = extract_title(content, os.path.basename(readme_path)[:-3])
                            wiki_filename = sanitize_filename(title)
                            if not wiki_filename:
                                wiki_filename = sanitize_filename(os.path.basename(readme_path)[:-3])
                            
                            # Add directory with link to its README
                            sidebar_file.write(f"{indent}- {dir_icon} **[{child_name}]({wiki_filename})**\n")
                        except Exception as e:
                            print(f"Error processing README for sidebar: {readme_path}: {str(e)}")
                            sidebar_file.write(f"{indent}- {dir_icon} **{child_name}**\n")
                    else:
                        # No README, just show the directory
                        sidebar_file.write(f"{indent}- {dir_icon} **{child_name}**\n")
                    
                    # Recursively add its children
                    add_child_directories(child_path, indent + "  ")
            
            # Add directories to sidebar with README links if available
            sidebar_file.write("**Directories:**\n")
            for dir_path in sorted(top_level_dirs):
                dir_name = directory_hierarchy[dir_path]['name']
                
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
                
                # Check if this directory has a README
                if dir_path in dir_to_readme:
                    readme_path = dir_to_readme[dir_path]
                    try:
                        with open(readme_path, "r", encoding="utf-8") as f:
                            content = f.read()
                        title = extract_title(content, os.path.basename(readme_path)[:-3])
                        wiki_filename = sanitize_filename(title)
                        if not wiki_filename:
                            wiki_filename = sanitize_filename(os.path.basename(readme_path)[:-3])
                        
                        # Add directory with link to its README
                        sidebar_file.write(f"- {dir_icon} **[{dir_name}]({wiki_filename})**\n")
                    except Exception as e:
                        print(f"Error processing README for sidebar: {readme_path}: {str(e)}")
                        sidebar_file.write(f"- {dir_icon} **{dir_name}**\n")
                else:
                    # No README, just show the directory
                    sidebar_file.write(f"- {dir_icon} **{dir_name}**\n")
                
                # Add all child directories for each top-level directory
                add_child_directories(dir_path)
                
            sidebar_file.write("\n")
            
            # Find files that aren't in any specific subdirectory
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
            if top_level_files:
                sidebar_file.write("**Documents:**\n")
                for title, wiki_filename in sorted(top_level_files, key=lambda x: x[0].lower()):
                    # Skip READMEs that are already linked to directories
                    readme_used = False
                    for dir_path in dir_to_readme:
                        if dir_path in directory_hierarchy:
                            readme_path = dir_to_readme[dir_path]
                            try:
                                with open(readme_path, "r", encoding="utf-8") as f:
                                    content = f.read()
                                readme_title = extract_title(content, os.path.basename(readme_path)[:-3])
                                readme_wiki_filename = sanitize_filename(readme_title)
                                if wiki_filename == readme_wiki_filename:
                                    readme_used = True
                                    break
                            except Exception:
                                pass
                    
                    if not readme_used:
                        doc_icon = "📄"
                        if "readme" in title.lower():
                            doc_icon = "ℹ️"
                        elif "cli" in title.lower() or "command" in title.lower():
                            doc_icon = "⌨️"
                        sidebar_file.write(f"- {doc_icon} [{title}]({wiki_filename})\n")
                sidebar_file.write("\n")

    # Update stats at the top
    with open(wiki_dir / "Home.md", "r") as f:
        content = f.read()
    
    # Replace the placeholder stats line
    content = re.sub(
        r"\*This wiki is auto-generated from repository markdown files\*",
        f"*Generated from {total_files} markdown files across {processed_dirs} directories*",
        content
    )
    
    # Add link to custom styling at the top
    content = content.replace("# Wiki Index\n\n", "# Wiki Index\n\n[Custom Styling](custom-styling)\n\n")
    
    with open(wiki_dir / "Home.md", "w") as f:
        f.write(content)

    print(f"Wiki generation complete. Processed {total_files} files from {processed_dirs} directories.")

if __name__ == "__main__":
    main()