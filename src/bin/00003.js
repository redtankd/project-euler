console.time("Largest prime factor")

function f(n, x) {
    if (n == 1 || x == 1)
        return 1
    else if (n % x == 0)
        return f(n / x, x)
    else
        return n
}

let n = 600851475143
let x

for (x = 2; x <= n; x++) {
    n = f(n, x)
    if (n == 1) break;
}

console.timeEnd("Largest prime factor")

console.log("Largest prime factor = %d", x)

console.assert(6857 == x, "Wrong!")