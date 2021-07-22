import urllib.request
import json
import time

from person import URL, summary

def fetch():
    request = urllib.request.Request(
        URL,
        headers={'User-Agent': 'Mozilla/5.0 (Macintosh; M1 Mac OS X 11)'})
    res = urllib.request.urlopen(request)
    return json.load(res)


def main():
    for _ in range(50):
        person = fetch()
        print(summary(person))


if __name__ == "__main__":
    start_time = time.time()
    main()
    print("--- %s seconds ---" % (time.time() - start_time))
