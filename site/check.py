"""Local/CI entry: test, build, then check the generated page."""

import argparse
import json
import subprocess
import sys
from html.parser import HTMLParser
from pathlib import Path

from build import SITE, build


class Page(HTMLParser):
    def __init__(self):
        super().__init__()
        self.ids = []
        self.links = []
        self.assets = []
        self.keys = []

    def handle_starttag(self, tag, attrs):
        attrs = dict(attrs)
        if 'id' in attrs:
            self.ids.append(attrs['id'])
        if 'data-i18n' in attrs:
            self.keys.append(attrs['data-i18n'])
        if tag == 'a':
            self.links.append(attrs.get('href', ''))
        if tag in ('script', 'img'):
            self.assets.append(attrs.get('src', ''))
        if tag == 'link' and attrs.get('rel') == 'stylesheet':
            self.assets.append(attrs.get('href', ''))


def check(out):
    subprocess.run([sys.executable, str(SITE / 'test_build.py')], check=True)
    subprocess.run(['node', '--test', str(SITE / 'language-preference.test.cjs')], check=True)
    build(out)
    page = Page()
    page.feed((out / 'index.html').read_text(encoding='utf-8'))
    messages = json.loads((SITE / 'messages.json').read_text(encoding='utf-8'))
    assert len(page.ids) == len(set(page.ids)), 'Duplicate HTML ids'
    for key in page.keys:
        assert key in messages['en'] and key in messages['ja'], f'Missing message: {key}'
    for link in page.links:
        assert link, 'Empty link'
        if link.startswith('#'):
            assert link[1:] in page.ids, f'Missing fragment: {link}'
        else:
            assert link.startswith('https://github.com/ba0918/kotowari'), f'Unexpected destination: {link}'
    for asset in page.assets:
        assert (out / asset).is_file(), f'Missing asset: {asset}'
    for script in ('messages.js', 'language-preference.js', 'app.js'):
        subprocess.run(['node', '--check', str(out / script)], check=True)
    print(f'PASS: build, 7 contract tests, {len(page.keys)} text references, links, assets, JS syntax')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out', type=Path, default=SITE / 'dist')
    check(parser.parse_args().out.resolve())
