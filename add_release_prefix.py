import os
import sys
import time

directory = os.path.join("target", "bundled")
prefix = sys.argv[1]  # assuming prefix is passed as the first argument
max_retries = 5  # Number of retries for a locked file
delay = 2  # Delay in seconds between retries

# Iterate over files in the directory
for filename in os.listdir(directory):
    old_file = os.path.join(directory, filename)  # ensure you access the full file path
    new_file = os.path.join(directory, prefix + filename)  # add prefix to the filename
    
    retries = 0
    while retries < max_retries:
        try:
            os.rename(old_file, new_file)
            print(f"Renamed: {old_file} -> {new_file}")
            break
        except PermissionError:
            print(f"PermissionError: File '{old_file}' is being used. Retrying in {delay} seconds...")
            retries += 1
            time.sleep(delay)  # Wait before retrying
        except Exception as e:
            print(f"Error: {e}")
            break
    else:
        print(f"Failed to rename '{old_file}' after {max_retries} attempts.")

print(f"Prefix '{prefix}' added to release files.")