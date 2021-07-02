"demo of lists."
if __name__ == "__main__":

    xs = [1, 2, 3]
    print("Initial list:", xs)

    print("Push 4 into the list")
    xs.append(4)
    print("list:", xs)

    print("lists length:", len(xs))

    print("Second element:", xs[1])

    print("Pop last element:", xs.pop())

    print("Fourth element:", xs[3])

    print("Contents of xs:")
    for x in xs:
        print(">", x)
    """
    # enumerate
    # this
    for i, x in enumerate(xs):
        print(f"In position {i} we have value {x}")

    for i, x in enumerate(xs):
        xs[i] = x * 3
    # more idiomatic
    xs = [x * 3 for x in xs]
    print("Updated list:", xs)

    collected_iterator = [x for x in range(0, 10)]
    print("Collected (0..10) into:", collected_iterator)
    """
