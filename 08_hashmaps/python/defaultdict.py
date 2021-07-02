from collections import defaultdict

if __name__ == "__main__":
    d = defaultdict(list)

    d["banana"].append(0)
    print(d)

    # loops
    for k in d.keys():
        print(k)

    for v in d.values():
        print(v)
    # tuples as keys
    td = dict()
    td[(0, 1)] = []
    print(td)
