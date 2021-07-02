class Counter:
    def __init__(self):
        self.count = 0

    def __iter__(self):
        return self

    def __next__(self):
        if self.count < 5:
            self.count += 1
            return self.count
        raise StopIteration

    def __del__(self):
        print("destroyed at", self.count)


if __name__ == '__main__':
    counter = Counter()
    assert next(counter) == 1
    assert next(counter) == 2
    assert next(counter) == 3
    assert next(counter) == 4
    assert next(counter) == 5
    try:
        next(counter)
    except StopIteration:
        assert True

    count1 = Counter()
    count2 = Counter()
    next(count2)
    assert sum(filter(lambda x: x % 3 == 0, map(lambda t: t[0] * t[1], zip(count1, count2)))) == 18
    count1 = Counter()
    count2 = Counter()
    next(count2)
    assert sum((x for x in (a * b for a, b in zip(count1, count2)) if x % 3 == 0)) == 18
