# LEGB rule: Local, Enclosing, Global, and Built-in

global_variable = 1

def f():
    global global_variable
    local_variable = 2
    def subf():
        print("local", local_variable)
        print("global", global_variable)
        enclosed_variable = 3
        #local_variable = 5
    subf()
    #print("enclosed", enclosed_variable)
    global_variable = 9
    local_variable = 1
    print("global", global_variable)
    print("local", local_variable)
    # example use of built-in scope
    blah = int("42")


if __name__ == '__main__':
    f()
