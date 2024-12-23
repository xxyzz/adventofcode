import System.Environment
import System.IO

isArithmeticSequence :: [Int] -> Bool
isArithmeticSequence [] = True
isArithmeticSequence xs = isSafe (differences xs) || isSafe (differences $ reverse xs)
  where
    differences l = zipWith (-) (tail l) l
    isSafe = all (\x -> x >= 1 && x <= 3)

removeOne :: [Int] -> [[Int]]
removeOne [] = []
removeOne (x : xs) = xs : map (x :) (removeOne xs)

main = do
  fileName : _ <- getArgs
  contents <- readFile fileName
  let numbers = map (map read . words) $ lines contents
      q1result = foldl (\acc line -> if isArithmeticSequence line then acc + 1 else acc) 0 numbers
      q2result = foldl (\acc line -> if isArithmeticSequence line || any isArithmeticSequence (removeOne line) then acc + 1 else acc) 0 numbers
  print q1result
  print q2result
