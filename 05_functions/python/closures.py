if __name__ == "__main__":
    example_closure = lambda x: x

    s = example_closure("hello")
    n = example_closure(5)
    print(s, n)

    x = [1, 2, 3]

    equal_to_x = lambda z: z == x

    print("this is x", x)
    y = [1, 2, 3]
    assert equal_to_x(y)

    items = [1, 2, 3, 4, 5]
    plus_one = list(map(lambda x: x + 1, items))
    sum_all = sum(map(lambda x: x + 1, items))
    print(plus_one, sum_all)

    two_args = lambda x, y: x - y
    print(two_args(5, 3))
