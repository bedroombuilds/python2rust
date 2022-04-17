class User:
    @staticmethod
    def builder():
        return UserBuilder()

    def __init__(self, uid, email, first_name, last_name):
        self.uid = uid
        self.email = email
        self.first_name = first_name
        self.last_name = last_name

    def __str__(self):
        return f"User (uid: {self.uid}, email: {self.email}, first_name: {self.first_name}, last_name: {self.last_name})"


class UserBuilder:
    def id(self, uid):
        return UserBuilderHasId(uid)

    def email(self, email):
        return UserBuilderHasEmail(email)

class UserBuilderHasId:
    def __init__(self, uid):
        self.uid = uid

    def email(self, email):
        return UserBuilderHasIdEmail(self.uid, email)

class UserBuilderHasEmail:
    def __init__(self, email):
        self.email = email

    def id(self, uid):
        return UserBuilderHasIdEmail(uid, self.email)

class UserBuilderHasIdEmail:
    def __init__(self, uid, email):
        self._uid = uid
        self._email = email
        self._first_name = None
        self._last_name = None

    def build(self):
        assert self._uid is not None
        assert self._email is not None
        return User(self._uid, self._email, self._first_name, self._last_name)

    def first_name(self, name):
        self._first_name = name
        return self

    def last_name(self, name):
        self._last_name = name
        return self

if __name__ == "__main__":
    user = User.builder().id("42").email("bedroombuilds@example.com").first_name("Mike").build();
    print(user)

