# Mumrik Programming Language

Mumrik is a toy language.

# Feature

Mumrik will have the following features (not implemented yet, WIP)
* simple compile time processing
* functional programming
* OOP
* strong static typing

# Example

8th Fibonacci number
```
$ cat fib.mm
rec func fib x: Int :Int {
    if x < 2 { 1 }
    else { fib (x-1) + fib (x-2) }
}

fib 8
$ cargo run -- build fib.mm
$ ./a.out
$ echo $?
34
```

# Copyright
Copyright (C) 2016-2019 akitsu sanae.  
Distributed under the Boost Software License, Version 1.0. 
(See accompanying file LICENSE or copy at http://www.boost/org/LICENSE_1_0.txt)  



