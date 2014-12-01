console.time("Largest prime factor")

f = (n, x) ->
    if n == 1
        1
    else if n % x == 0
        f n / x, x
    else
        n

n = 600851475143
for x in [2...n]
    n = f n, x
    if n == 1 then break

console.timeEnd("Largest prime factor")

console.log "Largest prime factor = #{x}"