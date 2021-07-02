if __name__ == "__main__":
    alice = "I like dogs"
    bob = alice.replace("dog", "cat")

    print("Alice says:", alice)
    print("Bob says:", bob)
    print("Bob says again:", bob)

    pangram = "the quick brown fox jumps over the lazy"
    print("Pangram:", pangram)

    print("Words in reverse")
    for word in reversed(pangram.split()):
        print(">", word)


    chars = [c for c in pangram]
    chars = sorted(set(chars))


    string = ""
    for c in chars:
        string += c
        string += ", "


    chars_to_trim = ' ,'
    trimmed_str = string.strip(chars_to_trim)
    print("Used    characters:", string)
    print("Trimmed characters:", trimmed_str)
