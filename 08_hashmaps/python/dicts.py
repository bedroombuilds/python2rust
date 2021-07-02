
if __name__ == '__main__':
    literal = dict([("key", "value"), ("blah", "blubb")])
    literal = {
        "key": "value",
        "blah": "blubb",
    }
    print(literal)

    # normal init
    mutable = dict()
    mutable["one"] = 1
    mutable["two"] = 2
    del (mutable["one"])
    print(mutable.get("one"))
    print(mutable.get("two"))

    # add kv
    mutable["three"] = 3
    for k, v in mutable.items():
        print(k, v)
