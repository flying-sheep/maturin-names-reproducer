maturin divergent name reproducer
=================================

Reproducer for [PyO3/maturin#1520](https://github.com/PyO3/maturin/issues/1520).

Giving a rust crate and a Python distribution/module different names doesn’t seem to be really supported by maturin.

At the time of writing, I have the following, which I think mean what I write in the descriptions, but I might be wrong:

- <samp>Cargo.toml</samp>

  - `package.name = 'xdot'` to set the crate name for crates.io
  - `package.metadata.maturin.name = 'xdot_rs'` to set the dylib name used by maturin (defaults to `package.name` and needs to match the Python module name)

- <samp>pyproject.toml</samp>

  - `project.name = 'xdot-rs'` to set the distribution name for PyPI

- <samp>src/lib.rs</samp>

  - `#[pyo3(name = "xdot_rs")] pub fn pymodule...` to set the Python package name generated by maturin

Unfortunately the last one doesn’t seem to work: maturin generates a package named `xdot` in the .whl instead of `xdot_rs`, and the code it generates has `from .xdot import *` instead of `from .xdot_rs import *`:

```
./dist/xdot_rs-0.1.0-cp310-cp310-linux_x86_64.whl
├── xdot
│   ├── __init__.py
│   └── xdot_rs.cpython-310-x86_64-linux-gnu.so
└── xdot_rs-0.1.0.dist-info
    ├── METADATA
    ├── RECORD
    └── WHEEL
```

<samp>xdot/\_\_init\_\_.py</samp>:
```py
from .xdot import *

__doc__ = xdot.__doc__
if hasattr(xdot, "__all__"):
    __all__ = xdot.__all__
```
