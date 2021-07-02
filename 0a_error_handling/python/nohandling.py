def multiply(first: str, second: str):
    first = int(first)
    second = int(second)
    return first * second


if __name__ == '__main__':
    twenty = multiply("10", "2")
    print(f"double is {twenty}")

    tt = multiply("t", "2")
    print(f"double is {tt}")
