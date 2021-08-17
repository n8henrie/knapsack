# knapsack

For Coursera's Discrete Optimization course.

My first time working with [PyO3](https://github.com/PyO3/pyo3) and
[maturin](https://github.com/PyO3/maturin) to build rust libraries that I can
call from python -- quite a treat!

## MacOS Setup

- https://github.com/PyO3/pyo3/issues/1800

Working well with homebrew python, but having trouble with pyenv (possibly
because I use a framework install) and the MacOS system python.

1. python3 -m venv .venv && source ./.venv/bin/activate
2. pip install --upgrade pip maturin
3. maturin develop
4. cargo test --no-default-features
5. ./solver.py ./data/ks_4_0

## Windows Setup

Seemed to be working fine
