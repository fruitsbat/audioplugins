import subprocess

subprocess.run(
    [
        "cargo",
        "xtask",
        "bundle",
        "-p",
        "x_fader",
        "-p",
        "xy_fader",
        "--release"
    ]
)
