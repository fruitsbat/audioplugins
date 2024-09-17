import subprocess

subprocess.run(
    [
        "cargo",
        "xtask",
        "bundle",
        "-p",
        "x_fader",
        "xy_fader",
    ]
)
