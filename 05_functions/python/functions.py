def do_something(x):
    def do_some_other(x):
        return x * 3

    return do_some_other(x)


if __name__ == '__main__':
    print(do_something(3))
