def ascii_capitalize(name):
    if name:
        first = name[0]
        if first.isascii():
            return first.upper() + name[1:]
        return name
    return ""


names = ["alice", "bob", "özgül", "ßad"]
for name in names:
    print(name, '->', name.capitalize())
    print(name, '->', ascii_capitalize(name))
    print("")
