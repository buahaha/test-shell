import os
import subprocess


subprocess.run("pwsh -c docker run --rm -it kalilinux/kali-rolling:latest".split(" "))