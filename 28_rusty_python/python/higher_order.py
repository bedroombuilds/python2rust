def HasColor(name, default=None, doc=None):
    propname = "__color_{}".format(name)
    colors = {
        "black": 0x00000000,
        "white": 0x00ffffff,
        "red": 0x00ff0000,
        "green": 0x0000ff00,
        "blue": 0x000000ff
    }

    def getter(self):
        return getattr(self, propname, default)

    def setter(self, value):
        if isinstance(value, int):
            if isinstance(value, bool):
                raise TypeError("integer colors must be ints")

            if value < 0 or value > 0x00ffffff:
                raise ValueError("integer colors must be 24-bit")

            setattr(self, propname, value)
        elif isinstance(value, str):
            if value not in colors:
                raise ValueError("unknown color '{}'".format(value))

            setattr(self, propname, colors[value])
        else:
            raise TypeError("color specifications must be ints or strs")

    class Inner:
        pass

    setattr(Inner, name, property(getter, setter, None, doc))
    return Inner


class Canvas(HasColor("foreground"), HasColor("background")):
    pass


class Button(Canvas, HasColor("border"), HasColor("text")):
    pass


if __name__ == "__main__":
    c = Canvas()
    c.foreground = 'red'

    b = Button()
    b.foreground = 'black'
    b.text = 'green'
    print(f"canvas fg: {c.foreground:06x}, button txt: {b.text:06X}, button fg: {b.foreground}")
