[4000000].forEach (num) ->
    console.time "while #{num}"

    [a, b, sum] = [1, 2, 0]
    while b < num
        sum += b if b % 2 == 0
        [a, b] = [b, a + b]

    console.timeEnd "while #{num}"
    console.log "a = #{a}, b = #{b}, sum = #{sum}"

    console.time "recursive #{num}"

    f = (a, b, sum) ->
        if b < num
            f b, a + b, if b % 2 == 0 then sum + b else sum
        else
            [a, b, sum]
    [a, b, sum] = f 1, 2, 0

    console.timeEnd "recursive #{num}"
    console.log "a = #{a}, b = #{b}, sum = #{sum}"
    console.log ""