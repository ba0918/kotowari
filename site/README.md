# kotowari landing page

Public URL: https://ba0918.github.io/kotowari/

The Pages workflow builds and checks pull requests. It publishes only checked main
builds after changes to `site/` or the Pages workflow, or a manual main-branch run.
Only `site/dist` is uploaded. GitHub Pages uses the Actions build mode and the
`github-pages` environment restricts deployments to main.

## Update and check

Python 3.11+ and Node.js 20+:

```sh
python -m pip install --require-hashes -r site/requirements.txt
python site/check.py
```

`check.py` is the shared local/CI entry: contract tests, build, translation references,
local assets, fragment links and JavaScript syntax. The Pages workflow calls this same
command. Existing product workflows keep their own checks.
Build only: `python site/build.py`. Open `site/dist/index.html` directly, or serve
`site/dist` with a local static server. No CDN, web font or runtime fetch is required.

| Edit                                               | File                                                        |
| -------------------------------------------------- | ----------------------------------------------------------- |
| Semantic HTML and diagram structure                | `template.html`                                             |
| All translated text (plain strings, matching keys) | `messages.json`                                             |
| Layout, color and responsive rules                 | `style.css`                                                 |
| Applying the selected language                     | `app.js`                                                    |
| Reading/saving the language preference             | `language-preference.js`                                    |
| Jinja2 rendering and asset copying                 | `build.py`                                                  |
| Reproducible checks                                | `check.py`, `test_build.py`, `language-preference.test.cjs` |

Keep desktop rules with their component and responsive rules in the existing media
blocks. Edit the owning declaration rather than appending corrective CSS. Keep inline
text within its element so formatting does not add visible spaces. Jinja2 is the same
small rendering dependency used by kakoi/site; no custom template engine is introduced.
The build escapes all copy. Runtime translation uses `textContent`, never HTML insertion.

Language contract: read only valid `ba0918-language` values (`en`, `ja`), otherwise use
English. Reading never writes. Only an explicit button click saves. There is no legacy
key. If storage is denied, switching still works in the current page. The next navigation
or reload on the same origin inherits the saved choice; file URL persistence depends on
the browser. English is present without JavaScript.

For visual edits check both languages at 1440, 390 and 320px, a fresh visit, invalid/shared
preferences, reload persistence, storage denial, keyboard focus, primary links and all
section anchors. These browser checks supplement `check.py`.

## Content boundary

Reviewed main: `bc21216` on 2026-10-04; latest product release: `kotowari-v0.3.0`.
PR #4 is merged (2026-10-04): overview HTML and public Rust APIs are on main, but remain
under Unreleased. The main-only panel labels this difference. Stable guide links target
the v0.3.0 tag. The API guide is pinned to the reviewed main commit.

Sources: README, CHANGELOG, PROJECT, `docs/ir/core/{sources,coverage,test-discovery,
test-queries,overview-commands,library-api}.md`, `docs/guides/commands/{check,query,status}.md`,
`docs/guides/{marks,public-crate-api}.md`, `src/cli.rs`, and tests.
README's Rust-only sentence is stale: the IR, guides, implementation rules and v0.1.0
changelog describe the additional bundled languages. The LP follows those sources.
The command guide clarifies that exit 0 permits notices; the LP does not claim otherwise.

The connection diagram is an illustrative, abbreviated login example, not a screenshot,
CLI transcript or executable fixture. No product CLI was used to generate it. `check` links
specifications and tests; it does not execute tests or prove the implementation correct.
Semantic validity and review independence remain review responsibilities. No source translation or product feature is added.

Design/structure references: kakoi/site (current main, including the 5843305 refactor) and
kemi/site. Visual design is separate: gray-green surfaces, sans-serif typography and a
vertical trace diagram. Product code and the existing product workflows are unchanged.
