[1000, 100000].forEach (num) ->
    console.time("loop - #{num}")

    sum = 0
    sum += i for i in [1...num] when i % 3 == 0 or i % 5 == 0
    console.log sum

    console.timeEnd("loop - #{num}")


    console.time("filter - #{num}")

    console.log (
        [1...num]
        .filter (i) ->
            i % 3 == 0 or i % 5 == 0
        .reduce (a, b) ->
            a += b
    )

    console.timeEnd("filter - #{num}")
    console.log ""
