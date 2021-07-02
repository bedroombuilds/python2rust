class D:
    def __init__(self, x, y, z=None):
        self.x = x
        self.y = y
        self.z = z if z is not None else 0

    def __str__(self):
        return f"D({self.x}, {self.y}, {self.z})"


if __name__ == "__main__":
    d = D(1, 2)
    print(d)
