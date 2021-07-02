class D:
    def __init__(self, x, y, z=None):
        self.x = x
        self.y = y
        self.z = z if z is not None else 0

    def __str__(self):
        return f"D({self.x}, {self.y}, {self.z})"

    def __add__(self, rh):
        return D(self.x + rh.x, self.y + rh.y, self.z + rh.z)

    def __eq__(self, rh):
        return self.x == rh.x and self.y == rh.y and self.z == rh.z


if __name__ == "__main__":
    d1 = D(1, 2)
    d2 = D(1, 2, None)
    print(d1, d2)
    print(d1 + d2)
    assert d1 == d2
