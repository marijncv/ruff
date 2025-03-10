## Is Ruff compatible with Black?

Yes. Ruff is compatible with [Black](https://github.com/psf/black) out-of-the-box, as long as
the `line-length` setting is consistent between the two.

As a project, Ruff is designed to be used alongside Black and, as such, will defer implementing
stylistic lint rules that are obviated by autoformatting.

## How does Ruff compare to Flake8?

(Coming from Flake8? Try [`flake8-to-ruff`](https://pypi.org/project/flake8-to-ruff/) to
automatically convert your existing configuration.)

Ruff can be used as a drop-in replacement for Flake8 when used (1) without or with a small number of
plugins, (2) alongside Black, and (3) on Python 3 code.

Under those conditions, Ruff implements every rule in Flake8. In practice, that means Ruff
implements all of the `F` rules (which originate from Pyflakes), along with a subset of the `E` and
`W` rules (which originate from pycodestyle).

Ruff also re-implements some of the most popular Flake8 plugins and related code quality tools
natively, including:

* [autoflake](https://pypi.org/project/autoflake/) ([#1647](https://github.com/charliermarsh/ruff/issues/1647))
* [eradicate](https://pypi.org/project/eradicate/)
* [flake8-2020](https://pypi.org/project/flake8-2020/)
* [flake8-annotations](https://pypi.org/project/flake8-annotations/)
* [flake8-bandit](https://pypi.org/project/flake8-bandit/) ([#1646](https://github.com/charliermarsh/ruff/issues/1646))
* [flake8-blind-except](https://pypi.org/project/flake8-blind-except/)
* [flake8-boolean-trap](https://pypi.org/project/flake8-boolean-trap/)
* [flake8-bugbear](https://pypi.org/project/flake8-bugbear/)
* [flake8-builtins](https://pypi.org/project/flake8-builtins/)
* [flake8-commas](https://pypi.org/project/flake8-commas/)
* [flake8-comprehensions](https://pypi.org/project/flake8-comprehensions/)
* [flake8-datetimez](https://pypi.org/project/flake8-datetimez/)
* [flake8-debugger](https://pypi.org/project/flake8-debugger/)
* [flake8-django](https://pypi.org/project/flake8-django/) ([#2817](https://github.com/charliermarsh/ruff/issues/2817))
* [flake8-docstrings](https://pypi.org/project/flake8-docstrings/)
* [flake8-eradicate](https://pypi.org/project/flake8-eradicate/)
* [flake8-errmsg](https://pypi.org/project/flake8-errmsg/)
* [flake8-executable](https://pypi.org/project/flake8-executable/)
* [flake8-implicit-str-concat](https://pypi.org/project/flake8-implicit-str-concat/)
* [flake8-import-conventions](https://github.com/joaopalmeiro/flake8-import-conventions)
* [flake8-logging-format](https://pypi.org/project/flake8-logging-format/)
* [flake8-no-pep420](https://pypi.org/project/flake8-no-pep420)
* [flake8-pie](https://pypi.org/project/flake8-pie/)
* [flake8-print](https://pypi.org/project/flake8-print/)
* [flake8-pyi](https://pypi.org/project/flake8-pyi/)
* [flake8-pytest-style](https://pypi.org/project/flake8-pytest-style/)
* [flake8-quotes](https://pypi.org/project/flake8-quotes/)
* [flake8-raise](https://pypi.org/project/flake8-raise/)
* [flake8-return](https://pypi.org/project/flake8-return/)
* [flake8-self](https://pypi.org/project/flake8-self/)
* [flake8-simplify](https://pypi.org/project/flake8-simplify/) ([#998](https://github.com/charliermarsh/ruff/issues/998))
* [flake8-super](https://pypi.org/project/flake8-super/)
* [flake8-tidy-imports](https://pypi.org/project/flake8-tidy-imports/)
* [flake8-type-checking](https://pypi.org/project/flake8-type-checking/)
* [flake8-use-pathlib](https://pypi.org/project/flake8-use-pathlib/)
* [isort](https://pypi.org/project/isort/)
* [mccabe](https://pypi.org/project/mccabe/)
* [pandas-vet](https://pypi.org/project/pandas-vet/)
* [pep8-naming](https://pypi.org/project/pep8-naming/)
* [pydocstyle](https://pypi.org/project/pydocstyle/)
* [pygrep-hooks](https://github.com/pre-commit/pygrep-hooks) ([#980](https://github.com/charliermarsh/ruff/issues/980))
* [pyupgrade](https://pypi.org/project/pyupgrade/) ([#827](https://github.com/charliermarsh/ruff/issues/827))
* [yesqa](https://github.com/asottile/yesqa)

Note that, in some cases, Ruff uses different rule codes and prefixes than would be found in the
originating Flake8 plugins. For example, Ruff uses `TID252` to represent the `I252` rule from
flake8-tidy-imports. This helps minimize conflicts across plugins and allows any individual plugin
to be toggled on or off with a single (e.g.) `--select TID`, as opposed to `--select I2` (to avoid
conflicts with the isort rules, like `I001`).

Beyond the rule set, Ruff suffers from the following limitations vis-à-vis Flake8:

1. Ruff does not yet support structural pattern matching.
2. Flake8 has a plugin architecture and supports writing custom lint rules. (Instead, popular Flake8
   plugins are re-implemented in Rust as part of Ruff itself.)

There are a few other minor incompatibilities between Ruff and the originating Flake8 plugins:

* Ruff doesn't implement all the "opinionated" lint rules from flake8-bugbear.
* Depending on your project structure, Ruff and isort can differ in their detection of first-party
  code. (This is often solved by modifying the `src` property, e.g., to `src = ["src"]`, if your
  code is nested in a `src` directory.)

## How does Ruff compare to Pylint?

At time of writing, Pylint implements ~409 total rules, while Ruff implements 440, of which at least
89 overlap with the Pylint rule set (you can find the mapping in [#970](https://github.com/charliermarsh/ruff/issues/970)).

Pylint implements many rules that Ruff does not, and vice versa. For example, Pylint does more type
inference than Ruff (e.g., Pylint can validate the number of arguments in a function call). As such,
Ruff is not a "pure" drop-in replacement for Pylint (and vice versa), as they enforce different sets
of rules.

Despite these differences, many users have successfully switched from Pylint to Ruff, especially
those using Ruff alongside a [type checker](https://github.com/charliermarsh/ruff#how-does-ruff-compare-to-mypy-or-pyright-or-pyre),
which can cover some of the functionality that Pylint provides.

Like Flake8, Pylint supports plugins (called "checkers"), while Ruff implements all rules natively.
Unlike Pylint, Ruff is capable of automatically fixing its own lint violations.

Pylint parity is being tracked in [#970](https://github.com/charliermarsh/ruff/issues/970).

## How does Ruff compare to Mypy, or Pyright, or Pyre?

Ruff is a linter, not a type checker. It can detect some of the same problems that a type checker
can, but a type checker will catch certain errors that Ruff would miss. The opposite is also true:
Ruff will catch certain errors that a type checker would typically ignore.

For example, unlike a type checker, Ruff will notify you if an import is unused, by looking for
references to that import in the source code; on the other hand, a type checker could flag that you
passed an integer argument to a function that expects a string, which Ruff would miss. The
tools are complementary.

It's recommended that you use Ruff in conjunction with a type checker, like Mypy, Pyright, or Pyre,
with Ruff providing faster feedback on lint violations and the type checker providing more detailed
feedback on type errors.

## Which tools does Ruff replace?

Today, Ruff can be used to replace Flake8 when used with any of the following plugins:

* [flake8-2020](https://pypi.org/project/flake8-2020/)
* [flake8-annotations](https://pypi.org/project/flake8-annotations/)
* [flake8-bandit](https://pypi.org/project/flake8-bandit/) ([#1646](https://github.com/charliermarsh/ruff/issues/1646))
* [flake8-blind-except](https://pypi.org/project/flake8-blind-except/)
* [flake8-boolean-trap](https://pypi.org/project/flake8-boolean-trap/)
* [flake8-bugbear](https://pypi.org/project/flake8-bugbear/)
* [flake8-builtins](https://pypi.org/project/flake8-builtins/)
* [flake8-commas](https://pypi.org/project/flake8-commas/)
* [flake8-comprehensions](https://pypi.org/project/flake8-comprehensions/)
* [flake8-datetimez](https://pypi.org/project/flake8-datetimez/)
* [flake8-debugger](https://pypi.org/project/flake8-debugger/)
* [flake8-django](https://pypi.org/project/flake8-django/) ([#2817](https://github.com/charliermarsh/ruff/issues/2817))
* [flake8-docstrings](https://pypi.org/project/flake8-docstrings/)
* [flake8-eradicate](https://pypi.org/project/flake8-eradicate/)
* [flake8-errmsg](https://pypi.org/project/flake8-errmsg/)
* [flake8-executable](https://pypi.org/project/flake8-executable/)
* [flake8-implicit-str-concat](https://pypi.org/project/flake8-implicit-str-concat/)
* [flake8-import-conventions](https://github.com/joaopalmeiro/flake8-import-conventions)
* [flake8-logging-format](https://pypi.org/project/flake8-logging-format/)
* [flake8-no-pep420](https://pypi.org/project/flake8-no-pep420)
* [flake8-pie](https://pypi.org/project/flake8-pie/)
* [flake8-print](https://pypi.org/project/flake8-print/)
* [flake8-pytest-style](https://pypi.org/project/flake8-pytest-style/)
* [flake8-quotes](https://pypi.org/project/flake8-quotes/)
* [flake8-raise](https://pypi.org/project/flake8-raise/)
* [flake8-return](https://pypi.org/project/flake8-return/)
* [flake8-self](https://pypi.org/project/flake8-self/)
* [flake8-simplify](https://pypi.org/project/flake8-simplify/) ([#998](https://github.com/charliermarsh/ruff/issues/998))
* [flake8-super](https://pypi.org/project/flake8-super/)
* [flake8-tidy-imports](https://pypi.org/project/flake8-tidy-imports/)
* [flake8-type-checking](https://pypi.org/project/flake8-type-checking/)
* [flake8-use-pathlib](https://pypi.org/project/flake8-use-pathlib/)
* [mccabe](https://pypi.org/project/mccabe/)
* [pandas-vet](https://pypi.org/project/pandas-vet/)
* [pep8-naming](https://pypi.org/project/pep8-naming/)
* [pydocstyle](https://pypi.org/project/pydocstyle/)

Ruff can also replace [isort](https://pypi.org/project/isort/),
[yesqa](https://github.com/asottile/yesqa), [eradicate](https://pypi.org/project/eradicate/),
[pygrep-hooks](https://github.com/pre-commit/pygrep-hooks) ([#980](https://github.com/charliermarsh/ruff/issues/980)), and a subset of the rules
implemented in [pyupgrade](https://pypi.org/project/pyupgrade/) ([#827](https://github.com/charliermarsh/ruff/issues/827)).

If you're looking to use Ruff, but rely on an unsupported Flake8 plugin, feel free to file an Issue.

## What versions of Python does Ruff support?

Ruff can lint code for any Python version from 3.7 onwards, including Python 3.10 and 3.11.

Ruff does not support Python 2. Ruff _may_ run on pre-Python 3.7 code, although such versions
are not officially supported (e.g., Ruff does _not_ respect type comments).

Ruff is installable under any Python version from 3.7 onwards.

## Do I need to install Rust to use Ruff?

Nope! Ruff is available as [`ruff`](https://pypi.org/project/ruff/) on PyPI:

```shell
pip install ruff
```

Ruff ships with wheels for all major platforms, which enables `pip` to install Ruff without relying
on Rust at all.

## Can I write my own plugins for Ruff?

Ruff does not yet support third-party plugins, though a plugin system is within-scope for the
project. See [#283](https://github.com/charliermarsh/ruff/issues/283) for more.

## How does Ruff's import sorting compare to [isort](https://pypi.org/project/isort/)?

Ruff's import sorting is intended to be nearly equivalent to isort when used `profile = "black"`.
There are a few known, minor differences in how Ruff and isort break ties between similar imports,
and in how Ruff and isort treat inline comments in some cases (see: [#1381](https://github.com/charliermarsh/ruff/issues/1381),
[#2104](https://github.com/charliermarsh/ruff/issues/2104)).

Like isort, Ruff's import sorting is compatible with Black.

Ruff does not yet support all of isort's configuration options, though it does support many of
them. You can find the supported settings in the [API reference](https://beta.ruff.rs/docs/settings/#isort). For example, you can set
`known-first-party` like so:

```toml
[tool.ruff]
select = [
    # Pyflakes
    "F",
    # Pycodestyle
    "E",
    "W",
    # isort
    "I001"
]
src = ["src", "tests"]

[tool.ruff.isort]
known-first-party = ["my_module1", "my_module2"]
```

## Does Ruff support Jupyter Notebooks?

Ruff is integrated into [nbQA](https://github.com/nbQA-dev/nbQA), a tool for running linters and
code formatters over Jupyter Notebooks.

After installing `ruff` and `nbqa`, you can run Ruff over a notebook like so:

```shell
> nbqa ruff Untitled.ipynb
Untitled.ipynb:cell_1:2:5: F841 Local variable `x` is assigned to but never used
Untitled.ipynb:cell_2:1:1: E402 Module level import not at top of file
Untitled.ipynb:cell_2:1:8: F401 `os` imported but unused
Found 3 errors.
1 potentially fixable with the --fix option.
```

## Does Ruff support NumPy- or Google-style docstrings?

Yes! To enable specific docstring convention, add the following to your `pyproject.toml`:

```toml
[tool.ruff.pydocstyle]
convention = "google"  # Accepts: "google", "numpy", or "pep257".
```

For example, if you're coming from flake8-docstrings, and your originating configuration uses
`--docstring-convention=numpy`, you'd instead set `convention = "numpy"` in your `pyproject.toml`,
as above.

Alongside `convention`, you'll want to explicitly enable the `D` rule code prefix, like so:

```toml
[tool.ruff]
select = [
    "D",
]

[tool.ruff.pydocstyle]
convention = "google"
```

Setting a `convention` force-disables any rules that are incompatible with that convention, no
matter how they're provided, which avoids accidental incompatibilities and simplifies configuration.

## How can I tell what settings Ruff is using to check my code?

Run `ruff check /path/to/code.py --show-settings` to view the resolved settings for a given file.

## I want to use Ruff, but I don't want to use `pyproject.toml`. Is that possible?

Yes! In lieu of a `pyproject.toml` file, you can use a `ruff.toml` file for configuration. The two
files are functionally equivalent and have an identical schema, with the exception that a `ruff.toml`
file can omit the `[tool.ruff]` section header.

For example, given this `pyproject.toml`:

```toml
[tool.ruff]
line-length = 88

[tool.ruff.pydocstyle]
convention = "google"
```

You could instead use a `ruff.toml` file like so:

```toml
line-length = 88

[pydocstyle]
convention = "google"
```

Ruff doesn't currently support INI files, like `setup.cfg` or `tox.ini`.

## How can I change Ruff's default configuration?

When no configuration file is found, Ruff will look for a user-specific `pyproject.toml` or
`ruff.toml` file as a last resort. This behavior is similar to Flake8's `~/.config/flake8`.

On macOS, Ruff expects that file to be located at `/Users/Alice/Library/Application Support/ruff/ruff.toml`.

On Linux, Ruff expects that file to be located at `/home/alice/.config/ruff/ruff.toml`.

On Windows, Ruff expects that file to be located at `C:\Users\Alice\AppData\Roaming\ruff\ruff.toml`.

For more, see the [`dirs`](https://docs.rs/dirs/4.0.0/dirs/fn.config_dir.html) crate.

## Ruff tried to fix something, but it broke my code. What should I do?

Ruff's autofix is a best-effort mechanism. Given the dynamic nature of Python, it's difficult to
have _complete_ certainty when making changes to code, even for the seemingly trivial fixes.

In the future, Ruff will support enabling autofix behavior based on the safety of the patch.

In the meantime, if you find that the autofix is too aggressive, you can disable it on a per-rule or
per-category basis using the [`unfixable`](https://beta.ruff.rs/docs/settings/#unfixable) mechanic. For example, to disable autofix
for some possibly-unsafe rules, you could add the following to your `pyproject.toml`:

```toml
[tool.ruff]
unfixable = ["B", "SIM", "TRY", "RUF"]
```

If you find a case where Ruff's autofix breaks your code, please file an Issue!

## How can I disable Ruff's color output?

Ruff's color output is powered by the [`colored`](https://crates.io/crates/colored) crate, which
attempts to automatically detect whether the output stream supports color. However, you can force
colors off by setting the `NO_COLOR` environment variable to any value (e.g., `NO_COLOR=1`).

[`colored`](https://crates.io/crates/colored) also supports the the `CLICOLOR` and `CLICOLOR_FORCE`
environment variables (see the [spec](https://bixense.com/clicolors/)).
