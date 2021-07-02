class A:
    def __init__(self, x, y):
        self.x = x
        self.y = y


class B:
    members = []

    def __init__(self, x):
        self.members.append(x)


if __name__ == '__main__':
    a = A(1, 2)
    a.x += 2
    print("a.x", a.x)
    b1 = B(1)
    b2 = B(2)
    print(b1.members, b2.members)
