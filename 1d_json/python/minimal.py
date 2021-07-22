import json

if __name__ == "__main__":
    o_json = '["foo", {"bar": ["baz", null, 1.0, 2]}]';
    o = json.loads(o_json)
    print(o)

    s = json.dumps(['foo', {'bar': ('baz', None, 1.0, 2)}])
    print(s)

    o = json.loads(s)
    print(o)
