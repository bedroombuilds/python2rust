import socket
import ssl

HOST = "127.0.0.1"
HOSTNAME = 'michael.kefeder.at'
PORT = 65432
context = ssl.create_default_context()

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    with context.wrap_socket(s, server_hostname=HOSTNAME) as ssock:
        print(ssock.version())
        ssock.connect((HOST, PORT))
        ssock.sendall(b"Hello, world")
        data = ssock.recv(1024)

print("Received", repr(data))
