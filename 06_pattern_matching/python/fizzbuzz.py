if __name__ == "__main__":
    # not possible as lambda
    def if_buzz(x):
        if x % 15 == 0:
            print("FizzBuzz")
        elif x % 3 == 0:
            print("Fizz")
        elif x % 5 == 0:
            print("Buzz")
        else:
            print(x)

    def fizz_buzz(x):
        match x % 3, x % 5:
            case 0, 0: print("FizzBuzz"),
            case 0, _: print("Fizz"),
            case _, 0: print("Buzz"),
            case _, _: print("{}", x)

    list(map(fizz_buzz, range(1, 24)))
