import System.Environment
import System.IO
import Data.List
import qualified Data.Map as Map


parseInput :: String -> IO ([Int], [Int])
parseInput fileName = do
  fileContents <- readFile fileName
  let numbers = map (map read . words) $ lines fileContents
      firstList = map head numbers
      secondList = map (head . tail) $ numbers
  return (firstList, secondList)


countNumbers :: (Ord a) => [a] -> Map.Map a Int
countNumbers = foldr insertCount Map.empty
  where
    insertCount x = Map.insertWith (+) x 1


main = do
  fileName:_ <- getArgs
  (firstList, secondList) <- parseInput fileName
  let sortedFirst = sort firstList
      sortedSecond = sort secondList
      q1Result = sum $ zipWith (\x y -> abs (x - y)) sortedFirst sortedSecond
  putStrLn $ show q1Result

  let counts = countNumbers secondList
      q2Result = sum [x * Map.findWithDefault 0 x counts | x <- firstList]
  putStrLn $ show q2Result
