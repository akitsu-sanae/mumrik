# Mumrik Programming Language

Mumrikは秋津早苗が趣味で作っているプログラミング言語です。

# 特徴
Mumrikは以下の特徴を持つ言語を目指しています。

* C++の型操作
* 簡潔なコンパイル時処理のサポート
* 関数型プログラミング
* オブジェクト指向プログラミング
* 強い静的型付け

# 例

8th Fibonacci number
```
rec func fib x: int :int =
    if x < 2 { 1 }
    else { fib@(x-1) + fib@(x-2) }

println fib@8
```

# Copyright
Copyright (C) 2016 akitsu sanae.  
Distributed under the Boost Software License, Version 1.0. 
(See accompanying file LICENSE or copy at http://www.boost/org/LICENSE_1_0.txt)  



