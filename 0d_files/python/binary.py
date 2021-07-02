import sys
from pathlib import Path


def bytes_from_file(filename):
    return bytearray(Path(filename).read_bytes())


if __name__ == "__main__":
    filename = sys.argv[1]
    data = bytes_from_file(filename)
    print(f"data size {len(data)}bytes")

    user_home = Path.home()
    user_fonts = user_home.joinpath('Library', 'Fonts')
    print(f"user_home: {user_home} user_fonts: {user_fonts}")

    data = bytes_from_file(user_fonts)
