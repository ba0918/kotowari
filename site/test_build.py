import unittest

from jinja2 import UndefinedError

from build import render


class BuildContract(unittest.TestCase):
    def test_static_english_is_escaped(self):
        self.assertEqual(render('{{ copy.title }}', {'en': {'title': '<safe>'}, 'ja': {'title': '日本語'}}), '&lt;safe&gt;')

    def test_missing_translation_is_rejected(self):
        with self.assertRaises(ValueError):
            render('{{ copy.title }}', {'en': {'title': 'Hello'}, 'ja': {}})

    def test_unknown_template_key_is_rejected(self):
        with self.assertRaises(UndefinedError):
            render('{{ copy.absent }}', {'en': {'title': 'Hello'}, 'ja': {'title': 'こんにちは'}})


if __name__ == '__main__':
    unittest.main()
