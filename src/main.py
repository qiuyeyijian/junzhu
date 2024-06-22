# -*- coding:gb2312 -*-
from bs4 import BeautifulSoup
import requests
import re
import os


def get_contents(target):
    req = requests.get(url=target)
    req.encoding = 'gb2312'
    html = req.text
    bf = BeautifulSoup(html, 'lxml')
    contents = str(bf.find_all('div', class_='novelbody')[0])

    pattern = re.compile('.*?<br/>')
    contents = re.findall(pattern, contents)

    for i in range(len(contents)):
        contents[i] = contents[i].replace('<br/>', '\n')
        contents[i] = contents[i].replace('<h2>', '')
        contents[i] = contents[i].replace('</h2>', '\n')
    return contents


class Downloader(object):
    def __init__(self, novelid):
        self.target = f'https://www.jjwxc.net/onebook.php?novelid={novelid}'
        self.chapters = []
        self.urls = []
        req = requests.get(url=self.target)
        req.encoding = 'gb2312'
        html = req.text
        table_bf = BeautifulSoup(html, 'lxml')
        table = table_bf.find_all('table', class_='cytable')
        tr_bf = BeautifulSoup(str(table[0]), 'lxml')
        a = tr_bf.find_all('a', itemprop='url')
        for item in a:
            self.chapters.append(item.string)
            self.urls.append(item.get('href'))

    def download(self, filename):
        filename = os.path.abspath(filename)
        if os.path.isfile(filename):
            os.remove(filename)
        print('Downloading...')
        with open(filename, "a+") as f:
            for i in range(len(self.chapters)):
                f.writelines(get_contents(self.urls[i]))
                print('\r', 'In process: %.3f%%' % float(i / len(self.chapters) * 100), end='', flush=True)
            print('\nDone!\n')
            f.close()


if __name__ == '__main__':
    Downloader("7434574").download("j.txt")

