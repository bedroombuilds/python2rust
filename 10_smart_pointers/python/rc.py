import sys

if __name__ == "__main__":
    rc_examples = "Rc examples"
    print("refcount of rc_examples", sys.getrefcount(rc_examples))

    def scope():
        print("rc created")

        rc_a = rc_examples
        print("refcount of rc_a", sys.getrefcount(rc_a))

        def subscope():
            rc_b = rc_a
            print("refcount of rc_b", sys.getrefcount(rc_b))
            print("refcount of rc_a", sys.getrefcount(rc_a))
            print("refcount of rc_examples", sys.getrefcount(rc_examples))

        subscope()
        print("refcount of rc_examples after del rc_b", sys.getrefcount(rc_examples))

    scope()
    print("refcount of rc_examples after del rc_a", sys.getrefcount(rc_examples))
