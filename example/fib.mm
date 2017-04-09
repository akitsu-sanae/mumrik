rec func fib x: Int :Int {
    if x < 2 { 1 }
    else { fib (x-1) + fib (x-2) }
}

println fib 8

