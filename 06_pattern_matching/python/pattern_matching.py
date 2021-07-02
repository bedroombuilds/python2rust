class Dog:
  def __init__(self, name):
    self.name = name

class Cat:
  def __init__(self, age):
    self.age = age






def classify(pet):
  match pet:
    case Cat(age=age):
      print("A cat that's {} years old".format(age))
    case Dog:
      print("A dog named {}".format(pet.name))

def number(x):
    match x:
        case 1: print("one"),
        case 2 | 3: print("two or three"),
        case x if x >=4: print("four or bigger"), # range matching not yet possible
        case _: print("anything"),



if __name__ == "__main__":
    for x in range(0, 6):
        number(x)
    classify(Dog("Fido"))
    classify(Cat(3))
