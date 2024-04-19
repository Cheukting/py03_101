# PyO3 101 - Writing Python modules in Rust

## Preflight checklist

- [Install/ Update Rust](https://www.rust-lang.org/tools/install)
- Make sure having Python 3.8 or above (recommend 3.12)
- Make sure using virtual environment (recommend pyenv + virtualenv)

## Setting up

1. Set up virtual environment and install **maturin**

```
pyenv virtualenv 3.12.2 pyo3
pyenv activate pyo3
pip install maturin
```

2. Start a project

If you want to start from scratch and write your own Rust code, you can start a new project by

```
mkdir pyo3_101
cd pyo3_101
maturin init
```

or you can clone this repo with the code in this workshop included.

## Build settings

Note that we have 2 toml files:

- Cargo.toml
- pyproject.toml

I am sure pyproject.toml is familiar to most of you. It holds the Python library information of the Python library that we are building.

For Cargo.toml, it stores the build settings for compiling the Rust crate that can be access by Python.

Let's spend a few minute to inspect the two files.

## Different between Python and Rust

Python and Rust are totally different languages. Also, how Python works and how Rust works is completely different.

The Python code that we write is not compiled directly. We do not need to compile Python code 99.9% of the time. Instead, the code is interpreted by a Python interpreter. If you are using the standard CPython, it is interpreted in C by CPython and then executed.

On the other hand, Rust code needs to be compiled. So before you run your program written in Rust, you will need to compile it. (For details you can [see here](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html#building-and-running-a-cargo-project), but we will omit the details in this workshop.) We will also do that in this workshop, after the compilation, a Python module (written in Rust and compiled) is build and we can use that module in Python code.

## Terms used in Rust

### Crates

In Rust, we have crates. [From the Rust Book](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html): "A crate is the smallest amount of code that the Rust compiler considers at a time.". For simplicity sake, let's think of a similar counterpart in Python. It is like a single .py file. Although since Python and Rust is so different it works very differently, but for the structure in your code, you may say it is the "Python equivalent" for now.

### Marco

"Fundamentally, macros are a way of writing code that writes other code, which is known as metaprogramming." ([see here](https://doc.rust-lang.org/book/ch19-06-macros.html)). In my own word, it is a meta way to plug-in in reusable code. Instead of function, which is called during run time, macros are considered during the compilation. In Rust, there are 2 types of marcos and we will come across them shortly.

## Write our library code in Rust

We will look at how to write some Rust code to define a Python function and how to add it to our Python module.

In the workshop we do not assume you know how to write Rust and we are not aim to teach Rust. We will explain enough Rust code so you can start writing simple code to taste using PyO3. For resources to learn Rust, please check the [reference session at the end]().

If you use `maturin init` to start the project, you will see a lib.rs file being generated and have the following code written as a start.

```
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_101(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
```

Let's analyse this code before we move on.

First of all `use pyo3::prelude::*;` is kinda like `import` in Python, we are using modules from other crates and packages. This line is needed for using PyO3.

Then we have `#[pyfunction]` and `#[pymodule]`, they are *procedural macros*. They act like decorators in Python. It takes come code, modify it, and then give out some code. Kinda like decorators which takes a function, modify it, and they gives out a function.

Next, you will see the other type of marco as `wrap_pyfunction!`. It is *declarative macro*. The syntax is very similar to a function. You can [see here for details](https://doc.rust-lang.org/book/ch19-06-macros.html#declarative-macros-with-macro_rules-for-general-metaprogramming), however, for simplicity sake, we can think of it like a function for now.

Next, you may also notice that Rust are typed. For Python we can do duck typing which means that we do not have to declare the type of variables in advance. However, it is not the case for Rust. Since now more and more Python coders start typing their code, you may found it easier if you have the habit of typing in Python. We will touch on some Rust and Python types conversion later in this workshop.

You may notice there is `Ok` at the end of each function. In Rust, it is common practice to return a *Result* enum (`Ok` and `Err`) at the end of the function so that any error can be handled when returned. Again, we are skipping the details, which you can [refer to here](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html). Related to it, there are `?` operators which is used as a shorthand to return the `Err` early if error occurred ([see here](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator)).

Last, there are some minor syntax differences, for example defining function with `fn` instead and `def`. Existence of semicolons `;` and curly brankats `{}`, which are both absent in Python.

## How to build the library

In the workshop, we will use *maturin* to build the Rust crate into Python library. In the terminal, type `maturin --help` to see what command options we have.

Note that there are several command that we can build the library. We will use the `develop` command the first time so it will be installed on the virtual enviroment and we can test out the library as we go.

Let's try `maturin develop` and see what happened. It may take a while for the first time but at the end you will see:

```
✏️  Setting installed package as editable
🛠 Installed pyo3_101-0.1.0
```

You will also see that we now have a new folder `target/`, which stores build information, and `Cargo.lock`, which stores all the dependency information on the Rust side of things.

Now type `pip list`, you will see that we have the package installed locally. Let's try to run it with some Python code.

Although you can try it with the Python shell, we will create a `try.py` file so we can keep a record of the test runs.

Now put some test code in `try.py`:

```
from pyo3_101 import sum_as_string

sum = sum_as_string(1,2)
print(f"{sum} as type {type(sum)}")
```

and try running the file: `python try.py`

You can see it works as expected. Now we get the logistics out of the way, we can start developing a very simple "toy" package.

---


---
## Reference

- [The Rust Book](https://doc.rust-lang.org/book/title-page.html)
- [Teach-rs (GitHub repo)](https://github.com/tweedegolf/teach-rs)
- [The PyO3 user guide](https://pyo3.rs/)
