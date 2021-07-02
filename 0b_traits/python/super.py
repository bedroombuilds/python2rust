from abc import ABCMeta, abstractmethod


class Person:
    def __init__(self, name):
        self._name = name

    def name(self):
        return self._name


class Student(Person):
    def __init__(self, name):
        super().__init__(name)

    def university(self):
        raise NotImplementedError


class Programmer(metaclass=ABCMeta):
    @abstractmethod
    def fav_language(self):
        pass


class CompSciStudent(Student):
    def __init__(self, name):
        super().__init__(name)

    def git_username(self):
        raise NotImplementedError


class CollegeStudent(Student):
    def __init__(self, name):
        super().__init__(name)

    def university(self):
        return "Community College"


class RustProgrammer(Person, Programmer):
    def fav_language(self):
        return "Rust"


class SuperCompSciStudent(Student, Programmer):
    def __init__(self, name):
        super().__init__(name)

    def university(self):
        return "Alma Mater"

    def fav_language(self):
        return "Rust and Python"

    def git_username(self):
        return self.name().lower()


def make_person(rnd):
    if 0 <= rnd <= 3:
        return StreetPerson("Bob")
    elif 4 <= rnd <= 6:
        return CollegeStudent("Bobby")
    elif 7 <= rnd <= 9:
        return RustProgrammer("Robert")
    else:
        return SuperCompSciStudent("Bert")


def comp_sci_student_greeting(student):
    return "My name is {} and I attend {}. My favorite language is {}. My Git username is {}".format(
        student.name(), student.university(), student.fav_language(), student.git_username())


if __name__ == "__main__":
    p = make_person(10)
    cs = CollegeStudent("Bob")
    print(cs.name(), cs.university())
    print(comp_sci_student_greeting(p))
