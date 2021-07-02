import numpy

if __name__ == "__main__":
    a = numpy.matrix([[2.0, 1.0, 2.0], [-2.0, 2.0, 1.0], [1.0, 2.0, -2.0]])
    e, vecs = numpy.linalg.eig(a)
    print("eigenvalues =", e)
    print("V =", vecs)
    a_c = a.astype(complex)
    av = a_c.dot(vecs)
    print("AV =", av)
