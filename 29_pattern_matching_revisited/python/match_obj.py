#!/usr/bin/env python3.10
from dataclasses import dataclass
from enum import Enum

class Button(Enum):
    LEFT = 0
    RIGHT = 1

@dataclass
class Click:
    position: tuple
    button: Button

if __name__ == "__main__":
    events = [Button.LEFT,
            Click((2, 4), Button.LEFT),
            Click((32, 4), Button.RIGHT),
            {"text": "blah", "color": "blue"},
            {"sleep": "long"},
            {"sleep": 1.3},
            Click((0, 4), Button.RIGHT),
            Click((0, 4), Button.LEFT),
        ]

    for event in events:
        match event:
            case Button.LEFT as b:
                print("button found", b)
            case Click((x, y), button=Button.RIGHT):
                print("right click", x, y)
            case Click((0, y)):
                print("clicked on x-axis y:", y)
            case Click((x, y)):
                print("Click", x, y)
            case {"text": message, "color": str(c)}:
                print("showing", message, "in color", c)
            case {"sleep": float(t)}:
                print(f"sleeping {t}secs")
