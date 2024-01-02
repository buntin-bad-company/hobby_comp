import Foundation

func estimatePi(iterations: Int) -> Double {
    var insideCircle = 0
    for _ in 0..<iterations {
        let x = Double.random(in: 0...1)
        let y = Double.random(in: 0...1)
        if x*x + y*y <= 1 {
            insideCircle += 1
        }
    }
    return 4.0 * Double(insideCircle) / Double(iterations)
}

var piEstimates = [Double]()
var executionTimes = [Double]()
let iterations = 10000
let maxExecutions = 10

for _ in 1...maxExecutions {
    let startTime = CFAbsoluteTimeGetCurrent()
    let piEstimate = estimatePi(iterations: iterations)
    let endTime = CFAbsoluteTimeGetCurrent()

    let executionTime = (endTime - startTime) * 1000 // milliseconds
    piEstimates.append(piEstimate)
    executionTimes.append(executionTime)

    print("推定値: \(piEstimate), 時間: \(executionTime)ms")
}

let averagePi = piEstimates.reduce(0, +) / Double(piEstimates.count)
let averageTime = executionTimes.reduce(0, +) / Double(executionTimes.count)
print("\n平均円周率の推定値: \(averagePi)")
print("平均実行時間: \(averageTime)ms")
