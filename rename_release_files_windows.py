import os

# Define the directory where the files are located and the prefix to add
directory = '\target\bundled'  # Update with your directory
prefix = 'windows_'  # The prefix you want to add

# Loop through all files in the directory
for filename in os.listdir(directory):
    old_file = os.path.join(directory, filename)
    
    # Check if it's a file (not a directory or other type)
    if os.path.isfile(old_file):
        new_file = os.path.join(directory, prefix + filename)
        
        # Rename the file
        os.rename(old_file, new_file)

print(f"Prefix '{prefix}' added to all files.")
