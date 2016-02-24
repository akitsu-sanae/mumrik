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

FizzBuzz

```
func main args : std.vec<string>  -> int =
    {1 ... 20}.each |i| ->
        std.io.print if x % 15 == 0 then "FizzBuzz"
        else if i % 3 == 0 then "Fizz"
        else if i % 5 == 0 then "Buzz"
        else i.to_string()
    0
```

