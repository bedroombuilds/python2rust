import json


class Person:
    def __init__(self, name, age, job, verified, parents):
        self.name = name
        self.age = age
        self.job = job
        self.verified = verified
        self.parents = parents

    def __str__(self):
        return ", ".join([f"{k}: {v}" for k, v in self.__dict__.items()])


class MyEncoder(json.JSONEncoder):
    def default(self, o):
        return o.__dict__


class MyDecoder(json.JSONDecoder):
    def decode(self, s):
        d = json.JSONDecoder.decode(self, s)
        return Person(**d)


if __name__ == '__main__':
    bob = Person(name="Bob", age=12, job=None, verified=True,
            parents=["Alice", "Carl"])
    bob_json = json.dumps(bob, cls=MyEncoder)
    print(bob_json)
    bob = json.loads(bob_json, cls=MyDecoder)
    print(bob)
