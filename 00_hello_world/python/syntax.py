#!/usr/bin/env python3
"docstring for module."

def multiply(x: int, y: int) -> int:
    "docstring for function."
    # one line comment
    return x * y
    """multiline comment explaining
    that type hints using annotations are not
    enforced but can be used by third party tools
    such as type checkers, IDEs, linters, etc.
    """


if __name__ == "__main__":
    z = multiply(5, 6)
    print("result: {}".format(z))
    txt = f"result: {z}" # python >= 3.6
    print(txt)
