const { test } = require('node:test');
const assert = require('node:assert/strict');
const { readFileSync } = require('node:fs');
const vm = require('node:vm');

function preference(storage) {
  const context = vm.createContext({ localStorage: storage });
  vm.runInContext(readFileSync(`${__dirname}/language-preference.js`, 'utf8'), context);
  return vm.runInContext('siteLanguage', context);
}

test('first visit and invalid saved choices use English without writing', () => {
  for (const value of [null, '', 'fr']) {
    const writes = [];
    const language = preference({ getItem: () => value, setItem: (...args) => writes.push(args) });
    assert.equal(language.read(), 'en');
    assert.deepEqual(writes, []);
  }
});

test('only the shared key supplies a saved choice', () => {
  for (const value of ['ja', 'en']) {
    const reads = [];
    const language = preference({
      getItem: key => {
        reads.push(key);
        return value;
      }
    });
    assert.equal(language.read(), value);
    assert.deepEqual(reads, ['ba0918-language']);
  }
});

test('an explicit valid choice saves to the shared key', () => {
  const writes = [];
  const language = preference({ setItem: (...args) => writes.push(args) });
  language.save('ja');
  language.save('en');
  language.save('fr');
  assert.deepEqual(writes, [
    ['ba0918-language', 'ja'],
    ['ba0918-language', 'en']
  ]);
});

test('storage denial does not prevent reading a default or choosing a language', () => {
  const denied = () => {
    throw new Error('Storage denied');
  };
  const language = preference({ getItem: denied, setItem: denied });
  assert.equal(language.read(), 'en');
  assert.doesNotThrow(() => language.save('ja'));
});
