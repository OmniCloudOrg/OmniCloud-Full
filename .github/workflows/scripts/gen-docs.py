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
            desc = "Documentation from this directory"
            
            # Try to get a description from the README if available
            readme_path = os.path.join(source_dir, "README.md")
            if os.path.exists(readme_path):
                try:
                    with open(readme_path, "r", encoding="utf-8") as readme_file:
                        content = readme_file.read()
                        # Try to extract a brief description from the beginning
                        desc_match = re.search(r'^# .+?\n\n(.+?)(?=\n\n|\Z)', content, re.DOTALL)
                        if desc_match:
                            desc = desc_match.group(1).strip()
                            # Truncate if too long
                            if len(desc) > 100:
                                desc = desc[:97] + "..."
                except Exception:
                    pass  # Continue if readme can't be read
            
            home_file.write(f"| **[{dir_name}](#{dir_name.lower()})** | {desc} |\n")
        
        home_file.write("\n---\n\n")
        
        # Process each source directory with enhanced formatting
        for source_dir in source_dirs:
            print(f"Processing directory: {source_dir}")
            home_file.write(f"<h2 id='{source_dir.lower()}'>{source_dir.capitalize()}</h2>\n\n")
            
            # Get all markdown files with their paths relative to the source dir
            md_files = []
            subdirectory_files = {}  # Map to track files by their parent directory
            
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
                        
                        # Track files by their immediate parent directory
                        parent_dir = os.path.basename(root)
                        if parent_dir not in subdirectory_files and rel_path:
                            subdirectory_files[parent_dir] = []
                        if rel_path:
                            subdirectory_files[parent_dir].append((full_path, rel_path))
            
            # Process directory structure more intelligently
            # Identify parent-child relationships
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
            
            # Get all directories for structure
            directories = set(directory_hierarchy.keys())
            
            # First, get top-level directories (no parent or parent is empty)
            top_level_dirs = []
            for dir_path in sorted(directory_hierarchy.keys()):
                if not directory_hierarchy[dir_path]['parent']:
                    top_level_dirs.append(dir_path)
            
            # Build a table of contents that reflects the actual directory structure
            if directories:
                home_file.write("### Directory Structure\n\n")
                
                # Create a collapsible directory tree with better formatting
                home_file.write("<details>\n")
                home_file.write("<summary>Click to expand directory structure</summary>\n\n")
                
                # Build directory tree using more appealing formatting
                home_file.write("```\n")
                home_file.write(f"{source_dir}/\n")
                
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
                home_file.write("</details>\n\n")
            
            # Create a path to files map for grouped display
            path_to_files = {}
            for file_path, rel_path in sorted(md_files):
                if rel_path not in path_to_files:
                    path_to_files[rel_path] = []
                path_to_files[rel_path].append(file_path)
            
            # First show files in the root of this directory
            if "" in path_to_files:
                home_file.write("<div class='content-section'>\n\n")
                home_file.write("#### Main Documents\n\n")
                
                # Skip if only contains a README which is used for the directory description
                non_readme_files = [f for f in path_to_files[""] if os.path.basename(f).lower() != "readme.md"]
                
                if non_readme_files:
                    home_file.write("| Document | Description |\n")
                    home_file.write("|:---------|:------------|\n")
                    
                    for file_path in sorted(path_to_files[""]):
                        try:
                            filename = os.path.basename(file_path)
                            with open(file_path, "r", encoding="utf-8") as f:
                                content = f.read()
                            
                            title = extract_title(content, filename[:-3])
                            wiki_filename = sanitize_filename(title)
                            if not wiki_filename:
                                wiki_filename = sanitize_filename(filename[:-3])
                            
                            # Try to extract a brief description
                            desc = "Documentation"
                            desc_match = re.search(r'^# .+?\n\n(.+?)(?=\n\n|\Z)', content, re.DOTALL)
                            if desc_match:
                                desc = desc_match.group(1).strip()
                                # Truncate if too long
                                if len(desc) > 100:
                                    desc = desc[:97] + "..."
                            
                            home_file.write(f"| [{title}]({wiki_filename}) | {desc} |\n")
                        except Exception as e:
                            print(f"Error processing file for table: {file_path}: {str(e)}")
                else:
                    home_file.write("*No documents at root level*\n")
                
                home_file.write("\n</div>\n\n")
            
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
            
            # Process each subdirectory
            for rel_path in sorted([p for p in path_to_files.keys() if p != ""]):
                subdir_name = os.path.basename(rel_path)
                readme_file = None
                
                # Check if this directory has a README
                for file_path in path_to_files[rel_path]:
                    if os.path.basename(file_path).lower() == "readme.md":
                        readme_file = file_path
                        break
                
                home_file.write(f"<div class='content-section' id='{sanitize_filename(subdir_name)}'>\n\n")
                home_file.write(f"#### {subdir_name.capitalize()}\n\n")
                
                # If there's a README, include its description
                if readme_file:
                    try:
                        with open(readme_file, "r", encoding="utf-8") as f:
                            content = f.read()
                        # Try to extract a brief description from the beginning
                        desc_match = re.search(r'^# .+?\n\n(.+?)(?=\n\n|\Z)', content, re.DOTALL)
                        if desc_match:
                            home_file.write(f"{desc_match.group(1).strip()}\n\n")
                    except Exception:
                        pass  # Continue if readme can't be read
                
                home_file.write("| Document | Description |\n")
                home_file.write("|:---------|:------------|\n")
                
                for file_path in sorted(path_to_files[rel_path]):
                    try:
                        with open(file_path, "r", encoding="utf-8") as f:
                            content = f.read()
                        
                        title = extract_title(content, os.path.basename(file_path)[:-3])
                        wiki_filename = sanitize_filename(title)
                        if not wiki_filename:
                            wiki_filename = sanitize_filename(os.path.basename(file_path)[:-3])
                        
                        # Try to extract a brief description
                        desc = "Documentation"
                        desc_match = re.search(r'^# .+?\n\n(.+?)(?=\n\n|\Z)', content, re.DOTALL)
                        if desc_match:
                            desc = desc_match.group(1).strip()
                            # Truncate if too long
                            if len(desc) > 100:
                                desc = desc[:97] + "..."
                        
                        home_file.write(f"| [{title}]({wiki_filename}) | {desc} |\n")
                    except Exception as e:
                        print(f"Error processing file for table: {file_path}: {str(e)}")
                
                home_file.write("\n</div>\n\n")
            
            home_file.write("<hr>\n\n")  # Add separator between sections
    
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

details summary {
  cursor: pointer;
  color: #0366d6;
  font-weight: bold;
  padding: 8px;
  background-color: #f1f8ff;
  border-radius: 3px;
  margin-bottom: 8px;
}

details summary:hover {
  background-color: #dbedff;
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
</style>

[Return to Home](Home)
""")

    # Add custom CSS for better sidebar appearance
    with open(wiki_dir / "_Sidebar-custom.md", "w") as sidebar_style_file:
        sidebar_style_file.write("""<!-- Custom styling for sidebar -->
<style>
.wiki-sidebar .markdown-body h1 {
  font-size: 24px;
  margin-bottom: 16px;
  padding-bottom: 8px;
  border-bottom: 1px solid #eaecef;
}

.wiki-sidebar .markdown-body h2 {
  font-size: 18px;
  margin-top: 24px;
  margin-bottom: 8px;
  padding: 4px 0;
  background-color: #f1f8ff;
  padding-left: 8px;
  border-radius: 3px;
}

.wiki-sidebar .markdown-body ul {
  margin-bottom: 16px;
}

.wiki-sidebar .markdown-body li {
  margin: 3px 0;
}

.wiki-sidebar strong {
  color: #24292e;
}

.wiki-sidebar a {
  color: #0366d6;
}

.wiki-sidebar a:hover {
  text-decoration: underline;
}
</style>

[[include:_Sidebar]]
""")
        
    # Create a custom home page with better styling
    with open(wiki_dir / "_Home-custom.md", "w") as home_style_file:
        home_style_file.write("""<!-- Custom styling for homepage -->
<style>
.markdown-body h1 {
  font-size: 32px;
  margin-bottom: 16px;
  padding-bottom: 8px;
  border-bottom: 1px solid #eaecef;
}

.markdown-body h2 {
  font-size: 24px;
  margin-top: 24px;
  margin-bottom: 16px;
  padding-bottom: 8px;
  border-bottom: 1px solid #eaecef;
}

.markdown-body table {
  border: 1px solid #dfe2e5;
  border-radius: 3px;
  margin-bottom: 16px;
}

.markdown-body th {
  background-color: #f6f8fa;
  padding: 8px 13px;
  border-bottom: 1px solid #dfe2e5;
}

.markdown-body td {
  padding: 8px 13px;
  border-bottom: 1px solid #dfe2e5;
}

.markdown-body tr:last-child td {
  border-bottom: none;
}

.content-category {
  background-color: #f1f8ff;
  padding: 16px;
  border-radius: 3px;
  margin-bottom: 24px;
  border-left: 4px solid #0366d6;
}

.content-section {
  background-color: #f6f8fa;
  border: 1px solid #eaecef;
  border-radius: 3px;
  padding: 16px;
  margin-bottom: 16px;
}

.content-section h4 {
  margin-top: 0;
  color: #24292e;
  border-bottom: 1px solid #eaecef;
  padding-bottom: 8px;
  margin-bottom: 16px;
}
</style>

[[include:Home]]
""")
    
    # Create sidebar with proper directory hierarchy
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