main = print . factors $ 600851475143 

-- 因数分解
factors :: Integer -> [Integer]
factors d = fact [] 2 d

-- 参数
--  已使用的质因数
--  当前使用的除数，因式分解的起始数
--  待因数分解的数
fact :: [Integer] -> Integer -> Integer -> [Integer]
fact _ _ 1 = []
fact _ _ 2 = [2]
fact fs f d
  -- 如果除数已经大于待分解数的一半，待分解的数应该是质因数
  | f > d `div` 2 + 1 = d:fs
  -- 当前的除数不能分解，使用下一除数
  | d `mod` f /= 0 = fact fs (nextFactor f) d
  -- 当前的除数能整除，是质因数，继续尝试用当前除数分解。
  | d `mod` f == 0 = fact (f:fs) f (d `div` f)
  where 
    -- 如果下一除数能被已使用质因数分解，则跳过
    nextFactor f
      | ( filter (\x -> f' `mod` x == 0) ) fs == [] = f'
      | otherwise = nextFactor f'
      where f'= f+1