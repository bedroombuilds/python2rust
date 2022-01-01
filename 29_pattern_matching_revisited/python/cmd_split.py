#!/usr/bin/env python3.10
import sys

def quit_game():
    sys.exit(0)

class Room:
    def __init__(self, objs=None):
        self.objects = objs
        if objs is None:
            self.objects = set(["banana", "chair", "piano"])
    def describe(self):
        print("the room is dark and small, it contains", self.objects);
    def neighbor(self, direction):
        match direction:
            case "left":
                print("changed to room", direction)
                return Room(set(["knife", "bread", "butter"]))
            case "north":
                print("changed to room", direction)
                return Room(set(["polar-bear", "ice", "snow"]))
            case "right" |"front" |"back":
                print("No Room", direction)
            case _:
                print("unknown direction")
        return self

class Character:
    def __init__(self):
        self.backpack = set()
    def take(self, obj, room):
        if obj in room.objects:
            self.backpack.add(obj)
            room.objects.remove(obj)
        else:
            print("not found", obj)

    def drop(self, obj, room):
        if obj in self.backpack:
            self.backpack.remove(obj)
            room.objects.add(obj)
        else:
            print("not in backpack", obj)


if __name__ == "__main__":
    current_room = Room()
    character = Character()
    while True:
        command = input("What are you doing next? ")
        match command.split():
            case ["quit"]:
                print("Goodbye!")
                quit_game()
            case ["look"]:
                current_room.describe()
            case ["look", "backpack"]:
                print("in backpack", character.backpack)
            case ["take", obj]:
                character.take(obj, current_room)
            case ["drop", *objects]:
                for obj in objects:
                    character.drop(obj, current_room)
            case ["go", direction]:
                current_room = current_room.neighbor(direction)
            case ["go", ("north" | "south" | "east" | "west") as direction]:
                current_room = current_room.neighbor(direction)
            case ["go", chant] if chant in ["team", "wildcats"]:
                print(f"YEAH, GOOOOO {chant.upper()}!!")
            case _:
                print("didn't understand:", command)
