import * as wasm from "cao-math";

wasm.init_error_handling();

const points = [];
for (let i = 0; i < 4; ++i) {
  for (let j = 0; j < 4; ++j) {
    points.push(new wasm.Vec2Int(i, j));
  }
}

const mat = new wasm.Matrix2Int();
mat.set(0, 0, 1);
mat.set(1, 1, 2);

console.log(points, mat);
const results = mat.rightProd(points);

document.getElementById("test-results").innerText = `
All done
${JSON.stringify(results, null, 2)}
}`;
