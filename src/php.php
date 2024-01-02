<?php
function estimatePi($iterations)
{
  $insideCircle = 0;
  for ($i = 0; $i < $iterations; $i++) {
    $x = mt_rand() / mt_getrandmax();
    $y = mt_rand() / mt_getrandmax();
    if (sqrt($x * $x + $y * $y) <= 1.0) {
      $insideCircle++;
    }
  }
  return 4 * ($insideCircle / $iterations);
}

function calculateAverage($array)
{
  return array_sum($array) / count($array);
}

$piEstimates = [];
$executionTimes = [];
$iterations = 10000;
$maxExecutions = 10;

for ($i = 0; $i < $maxExecutions; $i++) {
  $startTime = microtime(true) * 1000; // ミリ秒に変換
  $piEstimate = estimatePi($iterations);
  $endTime = microtime(true) * 1000; // ミリ秒に変換

  $executionTime = $endTime - $startTime;
  $piEstimates[] = $piEstimate;
  $executionTimes[] = $executionTime;

  echo "推定値: $piEstimate, 時間: {$executionTime}ms\n";
}

$averagePi = calculateAverage($piEstimates);
$averageTime = calculateAverage($executionTimes);
echo "\n平均円周率の推定値: $averagePi\n";
echo "平均実行時間: {$averageTime}ms\n";
?>