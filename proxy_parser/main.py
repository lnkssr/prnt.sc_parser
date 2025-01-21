#!/usr/bin/env python3

import requests
import concurrent.futures

timeout = 0.5

def check_http_proxy(ip, port, timeout=timeout):
    proxies = {
        'http': f'http://{ip}:{port}',
        'https': f'http://{ip}:{port}'
    }
    try:
        response = requests.get('http://httpbin.org/ip', proxies=proxies, timeout=timeout)
        if response.status_code == 200:
            print(f"Прокси работает: {ip}:{port}")
            return f"{ip}:{port}"
    except requests.RequestException:
        pass
    return None

def download_proxy_list(url):
    response = requests.get(url)
    if response.status_code == 200:
        return response.text.splitlines()
    else:
        raise Exception(f"Ошибка при скачивании: {response.status_code}")

def validate_proxies(proxy_list, output_file, max_threads=500):
    active_proxies = []
    with concurrent.futures.ThreadPoolExecutor(max_threads) as executor:
        futures = {executor.submit(check_http_proxy, *line.strip().split(':')): line for line in proxy_list if line.strip()}
        for future in concurrent.futures.as_completed(futures):
            result = future.result()
            if result:
                active_proxies.append(result)
    
    with open(output_file, 'w') as f:
        for proxy in active_proxies:
            f.write(proxy + '\n')
    
    print("Проверка завершена, активные прокси записаны в файл.")
    return active_proxies

url = "https://raw.githubusercontent.com/TheSpeedX/PROXY-List/refs/heads/master/socks5.txt"
output_file = 'active_proxies.txt'

proxy_list = download_proxy_list(url)
active_proxies = validate_proxies(proxy_list, output_file)
print(f"Найдено {len(active_proxies)} активных прокси.")  
