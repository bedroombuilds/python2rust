import sys


if __name__ == "__main__":


    with open(sys.argv[1]) as fh:
        count = 0


        for line in fh:
            elems = line.split(',')
            if elems[1] == "Tanzania":
                count += 1



    print(f"found {count} sales for Tanzania")
