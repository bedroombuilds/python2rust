"demo of arrays."
from array import array
if __name__ == "__main__":
    xs = array('i', [1, 2, 3, 4, 5])

    # this is dumb, use `numpy.zeros((500,), dtype=numpy.uint64)` instead
    ys = array('Q', [0] * 500)

    print("first element of the array:", xs[0])
    print("second element of the array:", xs[1])

    print("small size:", len(xs))
    print("big size:", len(ys))

    print("a section of the array as a slice", xs[1:3])

    print(xs[5])
