import requests
import socket

def check_proxy(ip, port, timeout=5):
    try:
        with socket.create_connection((ip, port), timeout):
            return True
    except (socket.timeout, socket.error):
        return False

def download_proxy_list(url):
    response = requests.get(url)
    if response.status_code == 200:
        return response.text.splitlines()
    else:
        raise Exception(f"Ошибка при скачивании: {response.status_code}")

def validate_proxies(proxy_list, output_file):
    with open(output_file, 'w') as f:
        for line in proxy_list:
            line = line.strip()
            if not line:
                continue
            try:
                ip, port = line.split(':')
                if check_proxy(ip, int(port)):
                    print(f"Прокси активен: {line}")
                    f.write(line + '\n')  # Запись в файл в реальном времени
                else:
                    print(f"Прокси не активен: {line}")
            except ValueError:
                print(f"Некорректная строка: {line}")

url = 'https://raw.githubusercontent.com/TheSpeedX/SOCKS-List/master/socks4.txt'
output_file = 'active_proxies.txt'

proxy_list = download_proxy_list(url)
validate_proxies(proxy_list, output_file)

print("Проверка завершена, активные прокси записаны в файл.")
