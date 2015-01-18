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

    use argparse::{ArgumentParser, StoreTrue, Store};

    fn main() {
        let mut verbose = false;
        let mut name = "World".to_string();
        {  // this block limits scope of borrows by ap.refer() method
            let mut ap = ArgumentParser::new();
            ap.set_description("Greet somebody.");
            ap.refer(&mut verbose)
                .add_option(["-v", "--verbose"], box StoreTrue,
                "Be verbose");
            ap.refer(&mut name)
                .add_option(["--name"], box Store::<String>,
                "Name for the greeting");
            match ap.parse_args() {
                Ok(()) => {}
                Err(x) => {
                    os::set_exit_status(x);
                    return;
                }
            }
        }

        if verbose {
            println!("name is {}", name);
        }
        println!("Hello {}!", name);
    }

Assuming the Rust code above is saved into a file ``greeting.rs``, let's see
what we have now::

    $ rustc greeting.rs
    $ ./greeting -h
    Usage: ./greeting [OPTIONS]

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
a mutable variable::

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

The ``refer`` method creates a cell variable, which the result will be written
to:

    let mut verbose = false;
    parser.refer(&mut verbose);

Next we add an options which control the variable:
For example::

    parser.refer(&mut verbose)
        .add_option(["-v", "--verbose"], box StoreTrue,
                    "Be verbose");

You made add multiple options for the same variable::

    parser.refer(&mut verbose)
        .add_option(["-v", "--verbose"], box StoreTrue,
                    "Be verbose")
        .add_option(["-q", "--quiet"], box StoreFalse,
                    "Be verbose");

Similarly positional arguments are added::

    let mut command = String;
    parser.refer(&mut command)
        .add_argument("command", box Store::<String>,
                      "Command to run");



Organizing Options
------------------

It's often useful to organize options into some kind of structure. You can
easily borrow variables from the structure into option parser. For example::

    struct Options {
        verbose: bool,
    }
    ...
    let mut options = Options { verbose: false }
    parser.refer(&mut options.verbose)
        .add_option(["-v"], box StoreTrue,
                    "Be verbose");


Parsing Arguments
-----------------

All the complex work is done in ``parser.parser_args()``, however, because
no exit function exists in rust, some more lines of code needed to check
the result::

    match parser.parse_args() {
        Ok(()) =>  {}
        Err(x) => {
            os::set_exit_status(x);
            return;
        }
    }


ArgumentParser Methods
======================

``parser.refer<T>(&mut self, var: &mut T) -> Ref``
    Attach the variable to argument parser. The options are added to the
    returned ``Ref`` object and modify a variable passed to the method.

``parser.set_description(descr: &str)``
    Set description that is at the top of help message.

``parser.stop_on_first_argument(val: bool)``
    If called with ``true``. Parser will stop searching for options when first
    non-option (the one doesn't start with ``-``) argument is encountered. This
    is useful if you want to parse following options with another argparser or
    external program.

``parser.print_usage(writer: Writer)``
    Prints usage string to stderr.

``parser.print_help(writer: Writer)``
    Writes help to ``writer``, used by ``--help`` option internally.

``parser.parse_args()``
    Method that does all the dirty work.


Variable Reference Methods
==========================

The ``argparse::Ref`` object is returned from ``parser.refer()``.
The following methods are used to add and customize arguments:

``option.add_option(names: &[&str], action: box TypedAction, help: &str)``
    Add an option. All items in names should be either in format ``-X`` or
    ``--long-option`` (i.e. one dash and one char or two dashes and long name).
    How this option will be interpreted and whether it will have an argument
    dependes on the action. See below list of actions.

``option.add_argument(name: &str, action: box TypedAction, help: &str)``
    Add a positional argument

``option.metavar(var: &str)``
    A name of the argument in usage messages (for options having argument).

``option.envvar(var: &str)``
    A name of the environment variable to get option value from. The value
    would be parsed with ``FromStr::from_str``, just like an option having
    ``Store`` action.


Actions
=======

The following actions are available out of the box. They may be used in either
``add_option`` or ``add_argument``:

``Store``
    An option has single argument. Stores a value from command-line in a
    variable. Any type that has ``FromStr`` trait implemented may be used. This
    action must be specified with ``box Store::<TYPE>`` syntax, because of
    limitation of rust type deriving algorithm. (Known types to work are all
    integer and floating types, str and path).

``StoreConst(value)``
    An option has no arguments. Store a hard-coded ``value`` into variable,
    when specified. Any type may be used.

``StoreTrue``
    Stores boolean ``true`` value in a variable.
    (shortcut for ``StoreConst(true)``)

``StoreFalse``
    Stores boolean ``false`` value in a variable.
    (shortcut for ``StoreConst(false)``)


``IncrBy(num)``
    An option has no arguments. Increments the value stored in a variable by a
    value ``num``. Any type which has ``Add`` trait may be used.

``DecrBy(nym)``
    Decrements the value stored in a variable by a value ``num``. Any type
    which has ``Add`` trait may be used.

``Collect``
    When used for an ``--option``, requires single argument. When used for a
    positional argument consumes all remaining arguments. Parsed options are
    added to the list. I.e. a ``box Collect::<int>`` action requires a
    ``Vec<int>`` variable. Parses arguments using ``FromStr`` trait.

``List``
    When used for positional argument, works the same as ``List``. When used
    as an option, consumes all remaining arguments.

    Note the usage of ``List`` is strongly discouraged, because of complex
    rules below. Use ``Collect`` and positional options if possible. But usage
    of ``List`` action may be useful if you need shell expansion of anything
    other than last positional argument.

    Let's learn rules by example. For the next options::

        ap.refer(&mut lst1).add_option(["-X", "--xx"], box List::<int>, "List1");
        ap.refer(&mut lst2).add_argument("yy", box List::<int>, "List2");

    The following command line::

        ./run 1 2 3 -X 4 5 6

    Will return ``[1, 2, 3]`` in the ``lst1`` and the ``[4,5,6]`` in the
    ``lst2``.

    Note that using when using ``=`` or equivalent short option mode, the
    'consume all' mode is not enabled. I.e. in the following command-line::

        ./run 1 2 -X3 4 --xx=5 6

    The ``lst1`` has ``[3, 5]`` and ``lst2`` has ``[1, 2, 4, 6]``.
    The argument consuming also stops on ``--`` or the next option::

        ./run: -X 1 2 3 -- 4 5 6
        ./run: -X 1 2 --xx=3 4 5 6

    Both of the above parse ``[4, 5, 6]`` as ``lst1`` and
    the ``[1, 2, 3]`` as the ``lst2``.



