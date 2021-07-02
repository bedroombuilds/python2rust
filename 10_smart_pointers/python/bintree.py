class Node:
    printed = set()

    def __init__(self, value):
        self.parent = None
        self.left = None
        self.right = None
        self.value = value

    def __repr__(self):
        try:
            if id(self) in self.printed:
                return f"-"
            self.printed.add(id(self))
            spaces = " " * len(self.printed)
            s = f"Node\n{spaces}{spaces}parent: {self.parent}, left: {self.left}, right: {self.right}, v: {self.value}\n"
            return s
        except RecursionError:
            return "-"


if __name__ == '__main__':
    root = Node(0)
    node = Node(1)
    node.parent = root
    root.left = node
    node2 = Node(2)
    root.parent = node2

    print("Root:", root)
    Node.printed = set()
    print("Parent left:", root.left.parent.value)
    Node.printed = set()
    print("ids: {} {}".format(id(node2), id(root.parent)))
    del node2
    #root.parent = None
    print("Parent:   ", root.parent)
