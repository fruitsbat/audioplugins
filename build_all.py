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
        "-p",
        "xyz_fader",
        "-p",
        "crossfader_gui",
        "--release"
    ]
)
