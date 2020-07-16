func hoge {x: Int, y: Int}: Int {
    (func x_: Int => x_ + y)(x)
}

hoge {x = 1, y = 11}


