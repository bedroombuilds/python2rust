if __name__ == '__main__':

    x = 1
    print(x)
    # bind x again shadows old one from above
    x = 'i'
    print(x)

    # declare, init
    something = None
    x = 5
    print("x, something", x, something)
    something = x * 5
    print("x, something", x, something)

    # mutability
    y = 0
    y = y * 2 + x
    print(y)

    BLAH = 42  # only const by convention
    y *= BLAH
