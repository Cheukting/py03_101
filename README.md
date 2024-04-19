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

or you can clone this repo with the code in this workshop included in the `ex_*` folders.

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

Note that there are several command that we can build the library. We will use the `develop` command so it will be installed on the virtual environment and we can test out the library as we go.

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
import pyo3_101 as p1

sum = p1.sum_as_string(1,2)
print(f"{sum} as type {type(sum)}")
```

and try running the file: `python try.py`

You can see it works as expected. Now we get the logistics out of the way, we can start developing a very simple "toy" package.

---

## Hello world

Now, let's add our own function called "say_hello", we will take a name as String and they return a Python String saying hello.

```
/// Take a name and say hello
#[pyfunction]
fn say_hello(name: String) -> PyResult<String> {
    Ok(format!("Hello {}, how are you today?", name))
}
```

Now try to save and type `maturin develop` in the terminal, you will see that our library is built but with a warning:

`warning: function `say_hello` is never used`

It is because we have to add our `say_hello` function to our Python module and it will not be available in the new Python package built. Let's fix it by adding it to the module:

```
/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_101(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(say_hello, m)?)?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
```

and build again with `maturin develop`. This time with no warning.

Now we can test the `say_hello` function in `try.py`:

```
# test say_hello
print(p1.say_hello("John"))
```

It works! Let say we will also take the name of the conference and welcoming people to that conference in `say_hello`:

```
/// Take name and conference to say hello
#[pyfunction]
fn say_hello(name: String, conf: String) -> PyResult<String> {
    Ok(format!("Hello {}, welcome to {}", name, conf))
}
```

So now we expect it to work if we develop again and update `try.py`:

```
# test say_hello
print(p1.say_hello("John", "PyCon"))
```

In Python we can pass in the arguments as either positional or keyword arguments, what if we do it like this:

```
# test say_hello
print(p1.say_hello(conf = "PyCon", name = "John"))
```

Do you think it still works? Let's try it now.

Before we move on to the next exercise, let's add one more thing. What if I want the default value if the name of the conference not provided to be "the conference"? We can use function signatures to do so:

```
/// Take a name and say hello
#[pyfunction]
#[pyo3(signature = (name, conf="the conference".to_string()))]
fn say_hello(name: String, conf: String) -> PyResult<String> {
    Ok(format!("Hello {}, welcome to {}", name, conf))
}
```

Try it now with just the `name` attribute:

```
# test say_hello
print(p1.say_hello(name = "John"))
```

Here `#[pyo3(signature = (...))]` is a marco provided my PyO3 to generate `__text_signature__` attribute for the Python object created. If you are curious, you can go to a Python shell to inspect:

```
>>> import pyo3_101 as p1
>>> dir(p1.say_hello)
['__call__', '__class__', '__delattr__', '__dir__', '__doc__', '__eq__', '__format__', '__ge__', '__getattribute__', '__getstate__', '__gt__', '__hash__', '__init__', '__init_subclass__', '__le__', '__lt__', '__module__', '__name__', '__ne__', '__new__', '__qualname__', '__reduce__', '__reduce_ex__', '__repr__', '__self__', '__setattr__', '__sizeof__', '__str__', '__subclasshook__', '__text_signature__']
>>> p1.say_hello.__text_signature__
'(name, conf=...)'
>>>
```

For more information about function signature, please refer to [the PyO3 user guide](https://pyo3.rs/).

---

## Reading a file

Now, we will read a registration list as a txt file and check if name is on that list. First of all, for using file io in Rust, we need to include some crates:

```
use std::fs::File;
use std::io::Read;
```

Then we can add a new function:

```
/// Give are gistration list and check if name is in it
#[pyfunction]
fn check_reg(filename: String, name: String) -> PyResult<String> {
    let mut file = File::open(filename).expect("File not exist");
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    if contents.contains(&name) {
        Ok("You are registered!".to_string())
    } else {
        Ok("Sorry you are not in our list!".to_string())
    }
}
```

Remember to add it to our module:

```
/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_101(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(say_hello, m)?)?;
    m.add_function(wrap_pyfunction!(check_reg, m)?)?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
```

Now, let's build and try it out:

```
# test check_reg
print(p1.check_reg("reg_list.txt", "John"))
```

Notice that we will now have an error:

`pyo3_runtime.PanicException: no file found: Os { code: 2, kind: NotFound, message: "No such file or directory" }`

This is expected as that file does not exist. In our Rust code:

```
let mut file = File::open(filename).expect("File not exist");
```

This state's that Rust will panic and terminate when the file cannot be opened. PyO3 will turn it into a PanicException in Python. In Python, if a file does not exist, we will get an FileNotFoundError instead. So let's see if we can make it raise a FileNotFoundError just like Python.

To understand how to do it, we need to have some understand of how errors are handled in Rust. You can see [this chapter of The Rust Book](https://doc.rust-lang.org/book/ch09-00-error-handling.html) for more information as in this workshop we will not go into details. This is what we will do, first we will include the `PyFileNotFoundError` provided by PyO3:

```
use pyo3::exceptions::PyFileNotFoundError;
```

Then for the function:

```
/// Give are gistration list and check if name is in it
#[pyfunction]
fn check_reg(filename: String, name: String) -> PyResult<String> {
    let file_result = File::open(filename);
    match file_result {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            if contents.contains(&name) {
                Ok("You are registered!".to_string())
            } else {
                Ok("Sorry you are not in our list!".to_string())
            }
        },
        Err(_) => {
            Err(PyFileNotFoundError::new_err("File not exist"))
        },
    }
}
```

Instead of throwing a panic if file cannot be open, we will catch the result and proceeded as normal if `Ok` and return a `PyFileNotFoundError` if `Err`.

Now try again, you will see the new error looks more familiar.

Next, let's create an empty file `reg_list.txt` in the working directory and see if it work as intended. After that we will add the name in the list (and few others) to check again.

If it is working as intended, we can move on to the next part of the workshop.

---




---
## Reference

- [The Rust Book](https://doc.rust-lang.org/book/title-page.html)
- [Teach-rs (GitHub repo)](https://github.com/tweedegolf/teach-rs)
- [The PyO3 user guide](https://pyo3.rs/)
