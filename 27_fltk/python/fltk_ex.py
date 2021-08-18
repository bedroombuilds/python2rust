from fltk import *
import sys

COUNTER = 0


class Adder(Fl_Widget):
    def __new__(self):
        container = Fl_Pack(80, 50, 200, 20, "Adder Widget")
        button = Fl_Button(0, 0, 50, 20, "add 1")
        button.callback(self.add1)
        button2 = Fl_Button(0, 0, 50, 20, "add 2")
        button2.callback(self.add2)
        button3 = Fl_Button(0, 0, 80, 20, "subtract 3")
        button3.callback(self.sub3)
        container.end()
        container.type(FL_HORIZONTAL)

        container2 = Fl_Pack(80, 130, 200, 20, "file widget")
        button4 = Fl_Button(0, 0, 80, 20, "open file")
        button4.callback(self.open_file)
        container2.end()

    def add1(self):
        global COUNTER
        COUNTER += 1
        disp_frame.label(str(COUNTER))

    def add2(self):
        global COUNTER
        COUNTER += 2
        disp_frame.label(str(COUNTER))

    def sub3(self):
        global COUNTER
        COUNTER -= 3
        disp_frame.label(str(COUNTER))

    def open_file(self):
        fc = Fl_File_Chooser(".", "*.rs", Fl_File_Chooser.SINGLE, "Choose File")
        fc.show()
        while fc.visible():
            Fl.wait()
        self.filename = fc.value(1)


class NoEscWindow(Fl_Window):
    def __init__(self, xpos, ypos, width, height, label):
        Fl_Window.__init__(self, xpos, ypos, width, height, label)

    def handle(self, ev):
        if ev == FL_SHORTCUT and Fl.event_key() == FL_Escape:
            return 1
        return Fl_Window.handle(self, ev)


if __name__ == "__main__":
    win = NoEscWindow(200, 200, 300, 200, "Ordinal Events")
    adder = Adder()
    disp_frame = Fl_Box(0, 0, 200, 200, "0")
    win.end()
    win.show(len(sys.argv), sys.argv)
    Fl.run()
