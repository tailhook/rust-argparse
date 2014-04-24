========
Argparse
========

The ``rust-argparse`` is command-line parsing module for rust. It's inspired
by python's ``argparse`` module.

Features:

* Supports standard (GNU) option conventions
* Properly typed values
* Automatically generated help and usage messages


Example
=======

The following code is a simple Rust program with command-line arguments:

.. code-block:: rust

    extern crate argparse;

    use std::os;

    use argparse::{ArgumentParser, cell, StoreTrue, SetStr};


    fn main() {
        let mut verbose = false;
        let mut name = ~"World";

        let mut ap = ArgumentParser::new();
        ap.set_description("Greet somebody.");
        ap.add_option(~["-v", "--verbose"],
            "Be verbose",
            StoreTrue(cell(&mut verbose)));
        ap.add_option(~["--name"], "",
            "Name for the greeting",
            SetStr(cell(&mut name)));
        ap.parse_args();

        if(verbose) {
            println!("name is {}", name);
        }
        println!("Hello {}!", name);
    }

Assuming the Rust code above is saved into a file ``greeting.rs``, let's see
what we have now::

    $ rustc greeting.rs
    $ ./greeting -h
    usage: ./greeting [-h] [-v|--verbose] [--name NAME]

    Greet somebody.

    optional arguments:
     -h, --help  show this help message and exit
     -v, --verbose
                 Be verbose
     --name NAME Name for the greeting
    $ ./greeting
    Hello World!
    $ ./greeting --name Bob
    Hello Bob!
    $ ./greeting -v --name Alice
    name is Alice
    Hello Alice!


Basic Workflow
==============


Create ArgumentParser
---------------------

The argument parser is created empty and is built incrementally. So we create
and immutable variable::

    extern crate argparse;
    use argparse::ArgumentParser;

    let mut parser = ArgumentParser::new();


Customize
---------

There are optional customization methods. The most important one is::

    parser.set_description("My command-line utility")

The descripion is rewrapped to fit 80 column string nicely. Just like option
descriptions.

Add Options
-----------

The ``add_option`` method adds an optional argument that starts with
dash, or a double dash. Multiple aliases may be specified for an option.
For example::

    parser.add_option(~["-v", "--verbose"],
        "Be verbose",
        StoreTrue(cell(&mut verbose)));

In case you need several options that refer to the same variable, just create
a cell variable::

    let verbose_option = cell(&mut verbose);
    parser.add_option(~["-v", "--verbose"],
        "Be verbose",
        StoreTrue(verbose_option));
    parser.add_option(~["-q", "--quiet"],
        "Be verbose",
        StoreFalse(verbose_option));

Note that in both cases the lifetime of the borrow equals to the lifetime of
the argument parser itself.

Organizing Options
------------------

It's often useful to organize options into some kind of structure. You can
easily borrow variables from the structure into option parser. For example::

    struct Options {
        verbose: bool,
    }
    ...
    let mut options = Options { verbose: false }
    parser.add_option(~["-v"], "Be verbose",
        StoreTrue(cell(&mut options.verbose)))

Parsing Arguments
-----------------

Just call::

    parser.parse_args()

And all the references are filled with values. Note that references used in
argument parser are borrowed for the lifetime of the parser. It usually means
that you may borrow variables again just after ``parser.parse_args()`` but in
case you use ``parser.error()`` or some other methods later in the code, it
may not be the case.


ArgumentParser Methods
======================

``parser.add_option(names:~[&str], helpstring: &str, action: Action)``
    Add an option. Help string may be rewrapped. Returns
    ``argparse::CliOption`` instance, which may be used to alter option's
    properties.

``parser.add_argument(name: &str, helpstring: &str, action: Action)``
    Add positional argument. Works similarly to ``add_option``.

``parser.set_description(descr: &str)``
    Set description that is at the top of help message.

``parser.disable_fromfile()``
    Disables special handling of ``@``-prefixed arguments. By default ``@path``
    argument on the command-line reads options from the file ``path``. Note:
    ``@something`` at the place of the argument to the option passes this
    literal value to the option.

``parser.print_usage(writer: Writer)``
    Prints usage string to stderr.

``parser.print_help(writer: Writer)``
    Writes help to ``writer``, used by ``--help`` option internally.

``parser.error(message: ~str)``
    Print usage, adding the message and terminates program with exit status 2.
    This method is useful if you have own validation on the command-line
    arguments.

``parser.parse_args()``
    Method that does all the dirty work.


CliOption Methods
=================

The ``argparse::CliOption`` object is returned from ``parser.add_option()`` or
``parser.add_argument()`` method call.  The following methods are used to
further customize arguments:

``option.metavar(var: &str)``
    A name of the argument in usage messages (for options having argument).
    It's error to call it on options having no argument.

``option.envvar(var: &str)``
    A name of the environment variable to get option value from

``option.prompt(message: &str)``
    Ask for the option value explicitly if it's not specified on the
    command-line.

``option.password_prompt(message: &str, confirm: bool)``
    Similar to ``prompt`` but hides input, and optionally prompts for
    confirmation.


Actions
=======

In description of actions we refer to ``Cell<T>`` as pointer to a value
typed ``T``. The actual type used is implementation detail. But you create
a cell object using a function call ``argparse::cell(&mut var)`` where ``var``
is a mutable variable (or field in a structure, whatever).

The following actions are available out of the box


``StoreTrue(Cell<bool>)``
    Stores boolean ``true`` value in a variable

``StoreFalse(Cell<bool>)``
    Stores boolean ``false`` value in a variable

``SetInt(Cell<int>)``
    Stores integer value from command-line in a variable

``IncrInt(Cell<int>)``
    Increments the value stored in a variable by one

``DecrInt(Cell<int>)``
    Decrements the value stored in a variable by one
