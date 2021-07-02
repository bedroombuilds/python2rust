def double_first(vec):
    try:
        first = vec[0]
        parsed = int(first)
        return parsed * 2
    except IndexError:
        print("no first item")
    except ValueError:
        print("invalid first item")


if __name__ == '__main__':
    numbers = ["42", "93", "18"]
    empty = []
    strings = ["tofu", "93", "18"]

    print(double_first(numbers))
    print(double_first(empty))
    print(double_first(strings))
