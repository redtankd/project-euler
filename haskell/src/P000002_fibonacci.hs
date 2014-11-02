{-   
  fibonacci
  see http://www.haskell.org/haskellwiki/The_Fibonacci_sequence
  see http://www.haskell.org/haskellwiki/Memoization
-}

main = print . sum . filter isEven . takeWhile (<4000000) $ memoized_fibonacci
  where isEven a = a `mod` 2 == 0

-- slow native fibonacci

fibonacci :: [Int]
fibonacci = map fibonacci' [1..]

fibonacci' :: Int -> Int
fibonacci' 1 = 1
fibonacci' 2 = 2
fibonacci' n = (fibonacci' (n-2)) + (fibonacci' (n-1))


-- Memoization
-- ע������ĵ�һ��Ԫ�ص��±�Ϊ0

memoized_fibonacci :: [Int]
memoized_fibonacci = map memoized_fib [1..]

memoized_fib :: Int -> Int
memoized_fib = (map fib [0..] !!)
  where fib 0 = 0
        fib 1 = 1
        fib 2 = 2
        fib n = memoized_fib (n-2) + memoized_fib (n-1)


-- easy infinite list

fibs1 = 1 : 2 : zipWith (+) fibs1 (tail fibs1)

fibs2 = scanl (+) 0 (1:fibs2)
{-
ԭ�����£�

fibonacci���У�0��1��1��2��3��5��8...

Fn+2 = Fn+1 + Fn
Fn+1 = Fn   + Fn-1
Fn   = Fn-1 + Fn-2
Fn-1 = Fn-2 + Fn-3
...
F3   = F2   + F1
F2   = F2
F1   = F1

��ͺ�õ�
Fn+2 = F2 + F1 + F2 + ... + Fn-1 + Fn
Fn+2 = F1 + F2 + F1 + F2 + ... + Fn-1 + Fn ����ΪF1=0��

���Ƴ�
F3 = F1 + F2 + F1 
F2 = F1 + F2
F1 = F1   
-}