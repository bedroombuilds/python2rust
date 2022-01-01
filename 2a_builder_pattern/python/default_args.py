from dataclasses import dataclass

class User:
    def __init__(self, uid, email, first_name=None, last_name=None):
        self.id = uid
        self.email = email
        self.first_name = first_name
        self.last_name = last_name

    def complete(self):
        return self.last_name is not None \
            and self.id > 0 \
            and not self.email.empty() \
            and self.first_name is not None


@dataclass
class Channel:
    token: int = 0
    special_info: str = ''


class ChannelBuilder:
    def __init__(self):
        self.token = 0
        self.special_info = '42'

    def build(self):
        return Channel(self.token, self.special_info)


if __name__ == '__main__':
    bob = User(13, "bob@example.com")
    bob.first_name = "Bob"

    print("complete?", bob.complete())
    print("bob_the_builder", bob)

    cb = ChannelBuilder()
    cb.special_info = "84"
    c1 = cb.build()
    c2 = Channel(4321)
    print(c1, c2)
    print(c1.token, c2.special_info)
