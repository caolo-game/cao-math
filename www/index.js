import * as wasm from "cao-math";

wasm.init();

const points = [];
for (let i = 0; i < 4; ++i) {
  for (let j = 0; j < 4; ++j) {
    points.push(new wasm.Point2(i, j));
  }
}

const mat = new wasm.Matrix2Int();
mat.set(0, 0, 1);
mat.set(1, 1, 2);

console.log(points, mat);
const results = mat.leftProd(points);

document.getElementById("test-results").innerText = `
All done
${JSON.stringify(results, null, 2)}
}`;
