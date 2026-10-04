"""Build the standalone English-first landing page."""

import argparse
import json
import shutil
from pathlib import Path

from jinja2 import Environment, StrictUndefined

SITE = Path(__file__).resolve().parent


def render(template, messages):
    if set(messages) != {'en', 'ja'} or messages['en'].keys() != messages['ja'].keys():
        raise ValueError('English and Japanese translation keys must match')
    if any(not isinstance(value, str) for copy in messages.values() for value in copy.values()):
        raise ValueError('Translations must be strings')
    engine = Environment(undefined=StrictUndefined, autoescape=True, keep_trailing_newline=True)
    page = engine.from_string(template)
    pages = {language: page.render(copy=copy) for language, copy in messages.items()}
    return pages['en']


def build(out):
    messages = json.loads((SITE / 'messages.json').read_text(encoding='utf-8'))
    html = render((SITE / 'template.html').read_text(encoding='utf-8'), messages)
    out.mkdir(parents=True, exist_ok=True)
    (out / 'index.html').write_text(html, encoding='utf-8')
    # Classic scripts keep the downloaded preview usable through file://.
    (out / 'messages.js').write_text('const messages = ' + json.dumps(messages, ensure_ascii=False, indent=2) + ';\n', encoding='utf-8')
    for name in ('style.css', 'app.js', 'language-preference.js'):
        shutil.copyfile(SITE / name, out / name)
    print(f'Built {out / "index.html"}')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out', type=Path, default=SITE / 'dist')
    build(parser.parse_args().out.resolve())
