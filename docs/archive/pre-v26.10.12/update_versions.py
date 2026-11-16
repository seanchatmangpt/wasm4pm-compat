import os
import glob
import re

files_to_check = []
for ext in ["*.md", "*.toml"]:
    files_to_check.extend(glob.glob(f"**/{ext}", recursive=True))

for file in files_to_check:
    if ".git" in file or "target" in file or "node_modules" in file:
        continue
    try:
        with open(file, "r") as f:
            content = f.read()
        
        # Replace 26.6.23, 26.6.14, and 26.6.24 with 26.6.25
        new_content = re.sub(r'26\.6\.(23|14|24)', '26.6.25', content)
        
        if new_content != content:
            with open(file, "w") as f:
                f.write(new_content)
            print(f"Updated {file}")
    except Exception as e:
        pass
