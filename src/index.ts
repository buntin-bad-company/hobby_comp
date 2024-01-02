const estimatePi = (iterations: number): number => {
  let insideCircle = 0;
  for (let i = 0; i < iterations; i++) {
    const x = Math.random();
    const y = Math.random();
    if (x * x + y * y <= 1) {
      insideCircle++;
    }
  }
  return 4 * (insideCircle / iterations);
};

const piEstimates: number[] = [];
const executionTimes: number[] = [];
const totalIterations = 10000;
const repeat = 10;

for (let i = 0; i < repeat; i++) {
  const start = performance.now();
  const piEstimate = estimatePi(totalIterations);
  const end = performance.now();

  piEstimates.push(piEstimate);
  executionTimes.push(end - start);

  console.log(`実行 ${i + 1}: πの推定値 = ${piEstimate}, 実行時間 = ${(end - start).toFixed(2)} ミリ秒`);
}

const averagePi = piEstimates.reduce((a, b) => a + b, 0) / piEstimates.length;
const averageTime = executionTimes.reduce((a, b) => a + b, 0) / executionTimes.length;

console.log(`平均 πの推定値 = ${averagePi}`);
console.log(`平均実行時間 = ${averageTime} ミリ秒`);
