import re
import sys

from iso_8601 import is_8601_date
RE = None


def is_proper_date(line):
    global RE
    if RE is None:
        RE = re.compile(r"^(\d{4})-(\d{2})-(\d{2})$")
    return RE.match(line)


if __name__ == "__main__":
    print("type 'quit' to quit")
    for line in sys.stdin:
        line = line.strip()

        if line == "quit":
            print("Good bye")
            break

        yn = " " if is_proper_date(line) else " not "

        print(f"{line} is{yn}a date")
