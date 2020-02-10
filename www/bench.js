import * as _ from "lodash";

export const bench = (name, fn, { sampleSize, iterations } = {}) => {
  let tasks = [];
  iterations = iterations || 100;
  sampleSize = sampleSize || 100;
  for (let index = 0; index < sampleSize; index++) {
    tasks.push(
      new Promise((resolve, reject) => {
        const start = new Date();
        for (let i = 0; i < iterations; ++i) {
          fn();
        }
        const end = new Date();
        const meanDeltaMs = (end - start) / iterations;

        resolve({
          meanDeltaMs,
          end
        });
      })
    );
  }
  return Promise.all(tasks).then(results => {
    results.sort((a, b) => a.end - b.end);
    const deltas = results.map(r => r.meanDeltaMs);
    console.log(JSON.stringify(deltas, null, 4));
    const min = _.min(deltas);
    const max = _.max(deltas);
    const mean = _.mean(deltas);
    deltas.sort((a, b) => a - b);
    const median = results[Math.floor(results.length / 2)].meanDeltaMs;

    return {
      name,
      min,
      max,
      mean,
      median
    };
  });
};
