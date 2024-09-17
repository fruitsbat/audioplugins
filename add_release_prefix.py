import os
import sys

directory = os.path.join("target", "bundled")
prefix = sys.argv[0]

for filename in os.listdir(directory):
    old_file = os.path.join(directory)
    new_file = os.path.join(directory, prefix + filename)
    os.rename(old_file, new_file)

print(f"Prefix '{prefix}' added to release files")
