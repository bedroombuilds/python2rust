#!/usr/bin/env python3.10

command = "bob key sword coin"

class Character:
    def drop(self, what, where):
        print("dropping", what, "into", where)

    def pickup(self, what):
        print("pickup", what)

character = Character()
current_room = "bedroom"

match command.split():
    case ["drop", *objects]:
        for obj in objects:
            character.drop(obj, current_room)
    case ["pickup", *objects]:
        for obj in objects:
            character.pickup(obj)
    case ["test"]:
        print("unknown")
