#!/usr/bin/env python3
# dpw@alamo
# 2024-01-20 04:19:51

import sys

import socket

port = 22200

def main(args: list) -> None:
    # print(f'{args}')

    msg = "status"
    if len(args) > 0:
        msg = args[0];

    host = socket.gethostbyname(socket.gethostname())
    client = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    client.sendto(msg.encode(), (host, port))

    response = client.recvfrom(1024)
    print(f'{response[0].decode()}')


if __name__ == '__main__':
    main(sys.argv[1:])

