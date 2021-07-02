import threading

def thread_plus(name):
    global data, lock
    with lock:
        for _ in range(100_000):
            data += 1

if __name__ == "__main__":
    data = 0
    lock = threading.Lock()
    threads = list()
    for index in range(10):
        x = threading.Thread(target=thread_plus, args=(index,))
        threads.append(x)
        x.start()

    for h in threads:
        h.join()
    print(data)
