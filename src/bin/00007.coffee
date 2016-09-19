_ = require 'lodash'

isPrime = (number, list) ->
    _.isUndefined _(list).find (p) -> number % p == 0

console.time 'loop'

wanted = 10001
number = 2
list = []
while list.length < wanted
    list.push number if isPrime(number, list)
    number++

console.timeEnd 'loop'
console.log "loop result: #{_.last(list)}"
console.log ""

# Recursive function
# When 10001, maximum call stack size exceeded
wanted = 100
func = (number, list) ->
    list.push(number) if isPrime(number, list)
    if _.size(list) >= wanted
        list
    else    
        func(number + 1, list)
list = func(2, [])

console.log "recursive functionloop result: #{_.last(list)}"