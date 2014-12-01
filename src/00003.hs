main = print . factors $ 600851475143 

-- �����ֽ�
factors :: Integer -> [Integer]
factors d = fact [] 2 d

-- ����
--  ��ʹ�õ�������
--  ��ǰʹ�õĳ�������ʽ�ֽ����ʼ��
--  �������ֽ����
fact :: [Integer] -> Integer -> Integer -> [Integer]
fact _ _ 1 = []
fact _ _ 2 = [2]
fact fs f d
  -- ��������Ѿ����ڴ��ֽ�����һ�룬���ֽ����Ӧ����������
  | f > d `div` 2 + 1 = d:fs
  -- ��ǰ�ĳ������ֽܷ⣬ʹ����һ����
  | d `mod` f /= 0 = fact fs (nextFactor f) d
  -- ��ǰ�ĳ������������������������������õ�ǰ�����ֽ⡣
  | d `mod` f == 0 = fact (f:fs) f (d `div` f)
  where 
    -- �����һ�����ܱ���ʹ���������ֽ⣬������
    nextFactor f
      | ( filter (\x -> f' `mod` x == 0) ) fs == [] = f'
      | otherwise = nextFactor f'
      where f'= f+1