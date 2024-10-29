# PyO3 101 - Writing Python modules in Rust

## Preflight checklist

- [Install/ Update Rust](https://www.rust-lang.org/tools/install)
- Make sure having Python 3.8 or above (recommend 3.12)
- Make sure using virtual environment (recommend using uv)

## Windows checklist

In this workshop we recommend using Unix OS (Mac or Linux). *If you use Windows, you may encounter problems with Rust and Maturin.* To minimise issues that you may encounter, please go through the extra checklist below:

- Install the [c++ build tools](https://visualstudio.microsoft.com/downloads/)
- [Check the `dll` files are linked correctly](https://pyo3.rs/v0.21.2/faq#im-trying-to-call-python-from-rust-but-i-get-status_dll_not_found-or-status_entrypoint_not_found)

## Setting up

1. Create a new working directory

```
mkdir pyo3_101
cd pyo3_101
```

2. Set up virtual environment and install **maturin**

```
uv venv .venv
source .venv/bin/activate
uv pip install maturin
python -m ensurepip --default-pip
```
*Note: the last command is needed as maturin cannot find pip otherwise*

2. Start a project

If you want to start from scratch and write your own module, you can start a new project by

```
maturin init
```

or you can clone this repo with the code in this workshop included in the `ex_*` folders as check points at the end of each session/ exercises.

## Build settings

Note that we have 2 toml files:

- Cargo.toml
- pyproject.toml

I am sure pyproject.toml is familiar to most of you. It holds the Python library information of the Python library that we are building.

For Cargo.toml, it stores the build settings for compiling the Rust crate that can be access by Python. Including the following:

* [package] — Defines a package.
* name — The name of the package.
* version — The version of the package.
* edition — The Rust edition.
* [lib] — Library target settings.
* name — The name of the target.
* crate-type — The crate types to generate.
* [dependencies] — Package library dependencies.

Let's spend a few minute to inspect the two files.

*(Please make sure you are using the right version os PyO3, in this workshop we are using version __0.21.2__)*

## Different between Python and Rust

Python and Rust are totally different languages. Also, how Python works and how Rust works is completely different.

The Python code that we write is not compiled directly. We do not need to compile Python code 99.9% of the time. Instead, the code is interpreted by a Python interpreter. If you are using the standard CPython, it is interpreted in C by CPython and then executed.

On the other hand, Rust code needs to be compiled. So before you run your program written in Rust, you will need to compile it. (For details you can [see here](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html#building-and-running-a-cargo-project), but we will omit the details in this workshop.) We will also do that in this workshop, after the compilation, a Python module (written in Rust and compiled) is build and we can use that module in Python code.

## Terms used in Rust

### Crates

In Rust, we have crates. [From the Rust Book](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html): "A crate is the smallest amount of code that the Rust compiler considers at a time.". For simplicity sake, let's think of a similar counterpart in Python. It is like a single .py file. Although since Python and Rust is so different it works very differently, but for the structure in your code, you may say it is the "Python equivalent" for now.

### Macros

"Fundamentally, macros are a way of writing code that writes other code, which is known as metaprogramming." ([see here](https://doc.rust-lang.org/book/ch19-06-macros.html)). In my own word, it is a meta way to plug-in in reusable code. Instead of function, which is called during run time, macros are considered during the compilation. In Rust, there are 2 types of macros and we will come across them shortly.

### Panic

"By default, these panics will print a failure message, unwind, clean up the stack, and quit..." ([see here](https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html)) When somethings goes wrong in our program during run time, a panic will occur and the program will be terminated. This is different from `Exceptions` in Python as in Python we can capture the exception and do something else. In Rust errors are handled differently, we will touch upon that a tiny bit in this workshop. For details, I would recommend you read [chapter 9 of the Rust Book](https://doc.rust-lang.org/book/ch09-00-error-handling.html).

## Write our library code in Rust

We will look at how to write some Rust code to define a Python function and how to add it to our Python module.

In the workshop we do not assume you know how to write Rust and we are not aim to teach Rust. We will explain enough Rust code so you can start writing simple code to taste using PyO3. For resources to learn Rust, please check the [reference session at the end](#reference).

If you use `maturin init` to start the project, you will see a `lib.rs` file being generated and have the following code written as a start.

```rust
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_101(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
```

Let's analyse this code before we move on.

First of all `use pyo3::prelude::*;` is kinda like `import` in Python, we are using modules from other crates and packages. This line is needed for using PyO3.

Then we have `#[pyfunction]` and `#[pymodule]`, they are **procedural macros**. They act like decorators in Python. It takes come code, modify it, and then give out some code. Kinda like decorators which takes a function, modify it, and they gives out a function.

Next, you will see the other type of marco as `wrap_pyfunction!`. It is **declarative macro**. The syntax is very similar to a function. You can [see here for details](https://doc.rust-lang.org/book/ch19-06-macros.html#declarative-macros-with-macro_rules-for-general-metaprogramming), however, for simplicity sake, we can think of it like a function for now.

Next, you may also notice that Rust are typed. For Python we can do duck typing which means that we do not have to declare the type of variables in advance. However, it is not the case for Rust. Since now more and more Python coders start typing their code, you may found it easier if you have the habit of typing in Python. We will touch on some Rust and Python types conversion later in this workshop.

You may notice there is `Ok` at the end of each function. In Rust, it is common practice to return a **Result** enum (`Ok` and `Err`) at the end of the function so that any error can be handled when returned. Again, we are skipping the details, which you can [refer to here](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html). Related to it, there are `?` operators which is used as a shorthand to return the `Err` early if error occurred ([see here](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator)).

Last, there are some minor syntax differences, for example defining function with `fn` instead and `def`. Existence of semicolons `;` and curly brackets `{}`, which are both absent in Python.

## How to build the library

In the workshop, we will use **maturin** to build the Rust crate into Python library. In the terminal, type `maturin --help` to see what command options we have.

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

```python
import pyo3_101 as p1

sum = p1.sum_as_string(1,2)
print(f"{sum} as type {type(sum)}")
```

and try running the file: `python try.py`

You can see it works as expected. Now we get the logistics out of the way, we can start developing a very simple "toy" package.

---

## Exercise 1 - Hello world

Now, let's add our own function called "say_hello", we will take a name as String and they return a Python String saying hello.

```rust
/// Take a name and say hello
#[pyfunction]
fn say_hello(name: String) -> PyResult<String> {
    Ok(format!("Hello {}, how are you today?", name))
}
```

Now try to save and type `maturin develop` in the terminal, you will see that our library is built but with a warning:

`warning: function `say_hello` is never used`

It is because we have to add our `say_hello` function to our Python module and it will not be available in the new Python package built. Let's fix it by adding it to the module:

```rust
/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_101(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(say_hello, m)?)?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
```

and build again with `maturin develop`. This time with no warning.

Now we can test the `say_hello` function in `try.py`:

```python
# test say_hello
print(p1.say_hello("John"))
```

It works! Let say we will also take the name of the conference and welcoming people to that conference in `say_hello`:

```rust
/// Take name and conference to say hello
#[pyfunction]
fn say_hello(name: String, conf: String) -> PyResult<String> {
    Ok(format!("Hello {}, welcome to {}", name, conf))
}
```

So now we expect it to work if we develop again and update `try.py`:

```python
# test say_hello
print(p1.say_hello("John", "PyCon"))
```

In Python we can pass in the arguments as either positional or keyword arguments, what if we do it like this:

```python
# test say_hello
print(p1.say_hello(conf = "PyCon", name = "John"))
```

Do you think it still works? Let's try it now.

Before we move on to the next exercise, let's add one more thing. What if I want the default value when the name of the conference not provided to be "the conference"? We can use function signatures to do so:

```rust
/// Take a name and say hello
#[pyfunction]
#[pyo3(signature = (name, conf="the conference".to_string()))]
fn say_hello(name: String, conf: String) -> PyResult<String> {
    Ok(format!("Hello {}, welcome to {}", name, conf))
}
```

Try it now with just the `name` attribute:

```python
# test say_hello
print(p1.say_hello(name = "John"))
```

Here `#[pyo3(signature = (...))]` is a macro provided by PyO3 to generate `__text_signature__` attribute for the Python object created. If you are curious, you can go to a Python shell to inspect:

```python
>>> import pyo3_101 as p1
>>> dir(p1.say_hello)
['__call__', '__class__', '__delattr__', '__dir__', '__doc__', '__eq__', '__format__', '__ge__', '__getattribute__', '__getstate__', '__gt__', '__hash__', '__init__', '__init_subclass__', '__le__', '__lt__', '__module__', '__name__', '__ne__', '__new__', '__qualname__', '__reduce__', '__reduce_ex__', '__repr__', '__self__', '__setattr__', '__sizeof__', '__str__', '__subclasshook__', '__text_signature__']
>>> p1.say_hello.__text_signature__
'(name, conf=...)'
>>>
```

For more information about function signature, please refer to [the PyO3 user guide](https://pyo3.rs/v0.21.2/function/signature).

---

## Exercise 2 - Reading a file and handling error

Now, we will read a registration list as a text file and check if name is on that list. First of all, for using file io in Rust, we need to include some crates:

```rust
use std::fs::File;
use std::io::Read;
```

Then we can add a new function:

```rust
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

```rust
/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_101(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(say_hello, m)?)?;
    m.add_function(wrap_pyfunction!(check_reg, m)?)?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
```

Now, let's build and try it out:

```python
# test check_reg
print(p1.check_reg("reg_list.txt", "John"))
```

Notice that we will now have an error:

`pyo3_runtime.PanicException: no file found: Os { code: 2, kind: NotFound, message: "No such file or directory" }`

This is expected as that file does not exist. As in our Rust code:

```rust
let mut file = File::open(filename).expect("File not exist");
```

This stated that Rust will panic and terminate when the file cannot be opened. PyO3 will turn it into a **PanicException** in Python. In Python, if a file does not exist, we will get an **FileNotFoundError** instead. So let's make it raising a **FileNotFoundError** just like Python.

To understand how to do it, we need to have some understand of how errors are handled in Rust. You can see [this chapter of The Rust Book](https://doc.rust-lang.org/book/ch09-00-error-handling.html) for more information as in this workshop we will not go into details. This is what we will do, first we will include the `PyFileNotFoundError` provided by PyO3:

```rust
use pyo3::exceptions::PyFileNotFoundError;
```

Then for the function:

```rust
/// Give a registration list and check if name is in it
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

## Exercise 3 - Python and Rust type conversion

In the last part of exercise 1, you may notice in the macro for the function signature:

```rust
#[pyo3(signature = (name, conf="the conference".to_string()))]
```

There is a `.to_string()` after the string. It is because there are different string types in Rust. It can be confusing at first as there may not be a one to one conversion between the types for Python and Rust. Other than that, PyO3 also define some native Python types in Rust for users to make interface passing objects between the two. I suggest you keep the table of mapping handy, you can find it as [here in the user guide](https://pyo3.rs/v0.21.2/conversions/tables).

So far we have been only using the string type. Let's try using other types in this exercise.

Let say we want to take a list of attendee as Python list and count how many of them and return as an integer:

```rust
/// Give a list of attendee and count
#[pyfunction]
fn count_att(att_list: Vec<String>) -> PyResult<usize> {
    Ok(att_list.len())
}
```

And don't forget to add it to the module:

```rust
/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_101(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(say_hello, m)?)?;
    m.add_function(wrap_pyfunction!(check_reg, m)?)?;
    m.add_function(wrap_pyfunction!(count_att, m)?)?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
```

As we see, in Rust we have to define the type of the elements in `Vec`, unlike Python which a list can hold various type of objects.

Next we will take a Python dictionary, storing the travel budget of all the attendees, and we will calculate the average of travel spending, return as float.

As dictionary in Python is map to HashMap in Rust, we will need to include it:

```rust
use std::collections::HashMap;
```

Then for the function:
```rust
/// Give a dictionary of travel budgets and calculate average
#[pyfunction]
fn travel_avg(budget_dict: HashMap<String, f32>) -> PyResult<f32> {
    let mut sum: f32 = 0.0;
    let mut count: f32 = 0.0;
    for (_, budget) in budget_dict {
        sum = sum + budget;
        count = count + 1.0;
    }
    Ok(sum/count)
}
```

Note that since we want the result to be in float, both sum and count need to be a float type. We are using `f32` here.

As always we need to add it to the module:

```rust
/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_101(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(say_hello, m)?)?;
    m.add_function(wrap_pyfunction!(check_reg, m)?)?;
    m.add_function(wrap_pyfunction!(count_att, m)?)?;
    m.add_function(wrap_pyfunction!(travel_avg, m)?)?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
```

Next we `marutin develop` and try it out:

```python
# test count_att
budget_dict = {
"John": 850,
"Susan": 790,
"Peter":1030,
"Judy": 540,
}
print(p1.travel_avg(budget_dict))
```

You can compare the result with a function written in Python if you like. I will leave it for you to try it out yourself.

If there are extra time, you can challenge yourself to create more functions using various types of Python objects as arguments and return objects.

---

## Exercise 4 - Custom Python classes

Now, we know how to create functions and add them in our Python module, how about if I want to create class and methods in our module? Can we do that with PyO3?

Of cause we can, PyO3 provides macros like `#[pyclass]` and `#[pymethods]` for us. In this exercise, we will create an Attendee class.

Before we do that, let me explain a bit about using the `#[pyclass]` macro. It can be used with a Rust [enum](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html) or [struct](https://doc.rust-lang.org/book/ch05-01-defining-structs.html). In this case, we will use `struct` as we would like the Attendee class acting a bit like a dataclass is Python. So here we have:

```rust
/// Class for all attendees.
#[pyclass]
struct Attendee {
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    speaker: bool,
}
```

We have two attributes for the Attendee class for now. One is the name which will be a string, the other is to indicate if the Attendee is a speaker.

You see we also use the `#[pyo3(get)]` macro. This will create a simple getter for `name` and `speaker`. If both getter and setter is needed, i.e. if the attributes are mutable, then we will use `#[pyo3(get, set)]`. You can also set custom getters and setters which we will do a bit later in this exercise.

Next, we need to provide a `__new__` method so the instance of that class can be created with Python code:

```rust
#[pymethods]
impl Attendee {
    #[new]
    fn new(name: String, speaker: bool) -> Self {
        Attendee{
            name: name,
            speaker: speaker,
        }
    }
}
```

See here, we use `#[pymethods]` since `__new__`  would be a method for our Attendee class. We then create the method in Rust, so it follow the Rust [method syntax](https://doc.rust-lang.org/book/ch05-03-method-syntax.html) of using `impl` to create and return a new struct. Note that we use the `#[new]` macro so it will be become `__new__` menthod in Python.

Last, don't forget to add the Attendee class in our module:

```rust
/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_101(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(say_hello, m)?)?;
    m.add_function(wrap_pyfunction!(check_reg, m)?)?;
    m.add_function(wrap_pyfunction!(count_att, m)?)?;
    m.add_function(wrap_pyfunction!(travel_avg, m)?)?;
    m.add_class::<Attendee>()?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
```

Now we can `maturin develop` and try it out with Python script:

```python
# test Attendee
me = p1.Attendee('Cheuk', True)
print(me.name, me.speaker)
```

But this is not good, if we try this:

```python
# test Attendee
me = p1.Attendee('', True)
print(me.name, me.speaker)
```

Even the name is an empty string, it will still work. We should be able to do some check and handle error better. Let's change up the `new` method:

```rust
#[pymethods]
impl Attendee {
    #[new]
    fn new(name: String, speaker: bool) -> PyResult<Self> {
        if name.len() == 0 {
            Err(PyValueError::new_err("Please enter a name"))
        } else {
            Ok(
                Attendee{
                    name: name,
                    speaker: speaker,
                }
            )
        }
    }
}
```

We also need to add this:

```rust
use pyo3::exceptions::PyValueError;
```

As we would like to use the `PyValueError`.

This time we will return a `PyResult` and be able to do error handling like we did in exercise 2. Try again with an empty string as name and you will get an error this time instead.

It's time for some custom getter and setter. First, let's write a getter for `name` so it will return the name in all caps for easy reading. Within the `impl Attendee` and after `fn new`, add:

```rust
#[getter]
fn get_name(&self) -> PyResult<String> {
    Ok(self.name.to_uppercase())
}
```

Next, we want to create a custom setter for name which will do the same check as `new`. Within the `impl Attendee` and after `fn get_name`, add:

```rust
#[setter]
fn set_name(&mut self, name:String) -> PyResult<()> {
    if name.len() == 0 {
        Err(PyValueError::new_err("Please enter a name"))
    } else {
        self.name = name;
        Ok(())
    }
}
```

There are 2 things to note here, first for `self` that we pass in as the 1st argument for name, it need to be mutable since it is a setter, so we will need `&mut` instead of `&`, which we used in getter. Both `&mut` and `&` will pass `self` as a reference.

Second, for the setter, we do not return anything if there's no error, so the return type is `PyResult<()>`, while the getter will return a string.

You can also now remove the line `#[pyo3(get)]` above name when we define the struct Attendee. But it will still work if you keep it there. It is more tidy to remove it though.

Now `maturin develop` and play with it with some Python code to try it out. I will leave this to you.

---

## Exercise 5 - Creating iterators in Rust

*(this is a bonus exercise since someone has asked how to create iterators using PyO3)*

Now, after learning how to create custom Python classes, we can try to create an iterator for Python in Rust. Let's look at how we can create an iterator in Rust, for example in the Rust book, there is [an example of create an iterator that generate Fibonacci numbers](https://doc.rust-lang.org/rust-by-example/trait/iter.html).

We will try to create the same but use PyO3 to make it into something that can be used in Python. If you are confident, try doing it yourself before following the steps below.

First, let's create the class:

```rust
/// Iterator class for Fibonacci numbers.
#[pyclass]
struct Fibonacci {
    curr: u32,
    next: u32,
}
```

Then, we will need to define `__new__`:

```rust
#[pymethods]
impl Fibonacci {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Fibonacci { curr: 0, next: 1 })
    }
}
```

Since for a Python iterator, we need `__iter__` and `__next__` to be implemented. Let's go ahead and do it. Within `impl Fibonacci`, after `new` add:

```rust
fn __iter__(& self) -> PyResult<Self> {
    Ok(Fibonacci { curr: self.curr, next: self.next })
}
fn __next__(&mut self) -> PyResult<u32> {
    let current = self.curr;

    self.curr = self.next;
    self.next = current + self.next;

    Ok(current)
}
```

Note that for `__iter__` we will make a copy of `self`.

Don't forget to add it to our module:

```rust
/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_101(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(say_hello, m)?)?;
    m.add_function(wrap_pyfunction!(check_reg, m)?)?;
    m.add_function(wrap_pyfunction!(count_att, m)?)?;
    m.add_function(wrap_pyfunction!(travel_avg, m)?)?;
    m.add_class::<Attendee>()?;
    m.add_class::<Fibonacci>()?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
```

Now let's `maturin develop` and try it out in Python. I will leave this to you.

Right now this iterator does not have an end so we cannot use it in a `for` loop. If you try it in a `for` loop ~~you will get an endless loop (in theory)~~ it will stop when the integer overflow. This is bad, let's add an option to cap out the maximum number that it can generate.

First, we need an extra attribute:

```rust
#[pyclass]
struct Fibonacci {
    curr: u32,
    next: u32,
    max: u32,
}
```

Then we need to update `new` and `__iter__`:

```rust
#[new]
fn new(max: u32) -> PyResult<Self> {
    Ok(Fibonacci { curr: 0, next: 1, max: max})
}
fn __iter__(& self) -> PyResult<Self> {
    Ok(Fibonacci { curr: self.curr, next: self.next, max: self.max })
}
```
Next, we need to incorporate a check to stop the iteration in `__next__` using `StopIteration` in Python. To do that:

```rust
fn __next__(&mut self) -> PyResult<u32> {
    if self.next > self.max {
        Err(PyStopIteration::new_err("Reaching the end."))
    } else {
        let current = self.curr;

        self.curr = self.next;
        self.next = current + self.next;

        Ok(current)
    }
}
```

Since we use `PyStopIteration`, don't forget to add:

```rust
use pyo3::exceptions::PyStopIteration;
```

Now we can add a cap to the numbers by doing something like:

```python
for num in p1.Fibonacci(9999):
    print(num)
```

in Python. However, we can make it better by adding a default value so:

```python
for num in p1.Fibonacci():
    print(num)
```

will stop when it is going to overflow. To do that, we can use the `#[pyo3(signature = (...))]` macro like we do in exercise 1:

```rust
#[new]
#[pyo3(signature = (max=u32::MAX/2))]
fn new(max: u32) -> PyResult<Self> {
    Ok(Fibonacci { curr: 0, next: 1, max: max})
}
```

Note that `u32::MAX` is the maximum number that a `u32` can store. We need to divide it by two as we do not want to attempt adding two huge numbers in `self.next = current + self.next;` that will cause an overflow.

Now it's time to test it out and have a play with it. Technically, you can now also create Python generators. Feel free to create something else yourself before moving on to the next exercise.

---

## Exercise 6 - Class methods and attributes

Let's dive deeper into creating our own class. In the previous exercise we have created a Python class that allow us to create instances with attributes. However, there are cases that we would like to have Class methods and attributes. Here we will look at how to do that.

Now we would like to automatically generate a registration number when an Attendee is created. We would change the Attendee struct to this:

```rust
#[pyclass]
struct Attendee {
    #[pyo3(get)]
    reg_num: u32,
    name: String,
    #[pyo3(get)]
    speaker: bool,
}
```

Then we will store the counter of the number of registration as a class attribute. To declare a class attribute using PyO3, we can do this. Within `impl Attendee`, add:

```rust
#[classattr]
fn cur_reg_num() -> u32 {
    0
}
```

As you can see it is an unsigned integer with the initial value as 0. Next we want the `__new__` method to use this attribute when creating a new Attendee. To do this, we can make the `new` function a class method as well using `#[classmethod]`. Let's update the `new` function as:

```rust
#[new]
#[classmethod]
fn new(cls: &Bound<'_, PyType>, name: String, speaker: bool) -> PyResult<Self> {
    if name.len() == 0 {
        Err(PyValueError::new_err("Please enter a name"))
    } else {
        let cur_reg_num: u32 = cls.getattr("cur_reg_num")?.extract()?;
        cls.setattr("cur_reg_num", cur_reg_num + 1)?;
        Ok(
            Attendee{
                reg_num: cur_reg_num,
                name: name,
                speaker: speaker,
            }
        )
    }
}
```

As a class method, the first argument is a smart pointer to the Python class `Attendee` itself ([check here](https://pyo3.rs/v0.21.2/types#pyo3s-smart-pointers) for explanation regarding `Bound` smart pointers). We can then use `getattr` to get back the value of the attribute `cur_reg_num` and then convert it to a Rust integer using `extract`. Both methods will return a `Result` therefore we add the `?` to get back the value if `Ok`. We then use `setattr` to increate the class attribute by 1 and then add the number to our new attendee created.

Also, don't forget to add this:
```rust
use pyo3::types::PyType;
```

Since we use `PyType` in the first argument.

Now, lets test it out to see if it works as expected:

```python
# test Attendee
print(f"Number of attendees are {p1.Attendee.cur_reg_num}")

me = p1.Attendee('Cheuk', True)
print(me.name, me.speaker, me.reg_num)
print(f"Number of attendees are {p1.Attendee.cur_reg_num}")

keynote = p1.Attendee('John', True)
print(keynote.name, keynote.speaker, keynote.reg_num)
keynote.name = 'Jon'
print(keynote.name, keynote.speaker, keynote.reg_num)

print(f"Number of attendees are {p1.Attendee.cur_reg_num}")
```

Feel free to create another class attribute and class methods before moving to the next exercise.

---

## Exercise 7 - Creating Class Decorators

*(caution: in this exercise we will be using [RefCell](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html), which involve some unique concept in Rust regarding [smart pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html). Feel free to study about it at your own time. For now, you can just follow along to finish the exercise)*

Next we will take one step further and create decorators in Rust. Let's try to create one that will **add a log that store all the return output as strings to any Python functions**.

Before we write such decorator, let's look at a [simpler example in the PyO3 documentation](https://pyo3.rs/v0.21.2/class/call). Here a decorator class is created to add a call counter to any function written in Python. It is very similar to what we could have done using pure Python. Create a `__call__` method and return a wrapped function.

One tricky issue being mentioned is that, we cannot do a mutable borrow to self using `&mut self`, as it will create issues during runtime. (See [the documentation page](https://pyo3.rs/v0.21.2/class/call#what-is-the-cell-for) for details). So in the example, the `count` attribute is wrapped in `Cell` instead of just an integer. That way, the reference to the `Cell` object does not change, only the value inside changes.

You may think that we can do the same with our design here. However, since we will use `String` to store the log and `Cell` require the value stored can be copied ([this Stackoverflow answer](https://stackoverflow.com/a/72379465) provides a good explanation), but `String` cannot (more precisely, `String` does not have the `Copy` trait, only `Clone` trait). So we have to use a smart pointer `RefCell` to make our own borrow and clone the `String` inside ourselves.

If you are confident in writing Rust code and using `RefCell` please try to create your own version of our decorator before looking at the example below. If you are new to Rust, feel free to study the code below to learn how it works.

This is one of the ways to create such decorator:

```rust
/// Decorator class for creating logs.
#[pyclass]
struct SaveLog {
    log: RefCell<String>,
    wraps: Py<PyAny>,
}

#[pymethods]
impl SaveLog {
    #[new]
    fn __new__(wraps: Py<PyAny>) -> Self {
        SaveLog {
            log: RefCell::new(String::new()),
            wraps: wraps,
        }
    }
    #[getter]
    fn log(&self) -> String {
        self.log.borrow().clone()
    }
    #[pyo3(signature = (*args, **kwargs))]
    fn __call__(
        &self,
        py: Python<'_>,
        args: &Bound<'_, PyTuple>,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Py<PyAny>> {
        let old_log = self.log.borrow().clone();
        let ret = self.wraps.call_bound(py, args, kwargs)?;
        let new_log;
        if old_log.len() > 0 {
            new_log = format!("{}\n{}",old_log,ret);
        } else {
            new_log = format!("{}",ret);
        }
        self.log.replace(new_log);
        Ok(ret)
    }
}
```

As you can see, it is very similar to the example in the documentation. Instead of printing the count, we will store the result of the call before returning it.

Don't forget to add:
```rust
use pyo3::types::{PyDict, PyTuple};
```
and
```rust
use std::cell::RefCell;
```

and also remember to put this new decorator class in our module:

```rust
/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_101(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(say_hello, m)?)?;
    m.add_function(wrap_pyfunction!(check_reg, m)?)?;
    m.add_function(wrap_pyfunction!(count_att, m)?)?;
    m.add_function(wrap_pyfunction!(travel_avg, m)?)?;
    m.add_class::<Attendee>()?;
    m.add_class::<Fibonacci>()?;
    m.add_class::<SaveLog>()?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
```

After writing this decorator, try to use it in some Python code. If you want, you can also create some other decorators to do other things. I will leave the exploration to you. Have fun!

---

## Reference

This is the end of the workshop, there are much more in the usage of PyO3, however, we only have enough time to scratch the surface. Also, to make a usable Python package with PyO3, knowledge in Rust is needed. Here are links to resources that you can keep learning Rust and PyO3:

- [The Rust Book](https://doc.rust-lang.org/book/title-page.html)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Teach-rs (GitHub repo)](https://github.com/tweedegolf/teach-rs)
- [The PyO3 user guide](https://pyo3.rs/)

---

## Support this workshop

This workshop is created by Cheuk and is open source for everyone to use (under MIT license). Please consider sponsoring Cheuk's work via [GitHub Sponsor](https://github.com/sponsors/Cheukting).
