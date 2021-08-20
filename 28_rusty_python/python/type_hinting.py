from typing import Iterator


def fib(n: int) -> Iterator[int]:
    a, b = 0, 1
    while a < n:
        yield a
        a, b = b, a + b


class BankAccount:
    def __init__(self, initial_balance: int = 0) -> None:
        self.balance = initial_balance

    def deposit(self, amount: int) -> None:
        self.balance += amount

    def withdraw(self, amount: int) -> None:
        self.balance -= amount

    def overdrawn(self) -> bool:
        return self.balance < 0


if __name__ == "__main__":
    print(list(fib(10)))
    my_account = BankAccount(15)
    my_account.withdraw(5)
    print(my_account.balance)
