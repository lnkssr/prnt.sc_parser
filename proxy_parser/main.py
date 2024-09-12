import requests

def check_http_proxy(ip, port, timeout=5):
    proxies = {
        'http': f'http://{ip}:{port}',
        'https': f'http://{ip}:{port}'
    }

    try:
        # Проверяем доступность сайта
        response = requests.get('http://google.com', proxies=proxies, timeout=timeout)
        if response.status_code == 200:
            print(f"Прокси работает. Ответ от httpbin.org: {response.text}")
            return True
        else:
            print(f"Прокси не работает. Код ответа: {response.status_code}")
    except requests.RequestException as e:
        print(f"Ошибка запроса: {e}")
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
                if check_http_proxy(ip, int(port)):
                    f.write(line + '\n')  # Запись в файл в реальном времени
                else:
                    print(f"Прокси не активен: {line}")
            except ValueError:
                print(f"Некорректная строка: {line}")

url = 'https://raw.githubusercontent.com/TheSpeedX/SOCKS-List/master/http.txt'
output_file = 'active_proxies.txt'

proxy_list = download_proxy_list(url)
validate_proxies(proxy_list, output_file)

print("Проверка завершена, активные прокси записаны в файл.")
