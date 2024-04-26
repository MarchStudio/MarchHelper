import socket
import time

broadcast_address = "255.255.255.255"
port = 12345
message = input("要广播的信息")

# fake_source_ip = "127.0.0.1"  # 伪造的源IP地址用于测试

while True:
    with socket.socket(socket.AF_INET, socket.SOCK_DGRAM) as sock:
        # sock.bind((fake_source_ip, 0))  # 绑定伪造的源IP地址
        sock.setsockopt(socket.SOL_SOCKET, socket.SO_BROADCAST, 1)
        sock.sendto(message.encode(), (broadcast_address, port))
    time.sleep(0.1)
