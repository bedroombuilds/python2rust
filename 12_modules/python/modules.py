import sys as mysys
from os import path as p, system
import os

# inline module not possible

if __name__ == "__main__":
    os.path.isfile("/etc/hosts")
    p.isdir("/etc/hosts")
    system("ls")
    mysys.exit(1)
