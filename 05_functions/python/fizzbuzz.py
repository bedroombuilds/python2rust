if __name__ == "__main__":
    # not possible as lambda
    def fizz_buzz(x):
        if x % 15 == 0:
            print("FizzBuzz")
        elif x % 3 == 0:
            print("Fizz")
        elif x % 5 == 0:
            print("Buzz")
        else:
            print(x)

    for i in range(1, 16):
        fizz_buzz(i)

    print("---")
    list(map(fizz_buzz, range(1, 16)))
