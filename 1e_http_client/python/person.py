URL = "https://pipl.ir/v1/getPerson"


def summary(data):
    p = data['person']['personal']
    return f"{p['name']} {p['last_name']}, {p['age']}, {p['country']}"
