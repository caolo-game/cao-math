import * as wasm from "cao-math";
import { bench } from "./bench";

wasm.init_error_handling();

const points = [];
const RADIUS = 100;
for (let i = 0; i <= RADIUS; ++i) {
  for (let j = 0; j <= RADIUS; ++j) {
    points.push(new wasm.Vec2Float(i, j));
  }
}

const OFFSET = 50;
const renderList = (ctx, points) => {
  ctx.fillStyle = "#000000";
  for (let p of points) {
    const circle = new Path2D();
    circle.arc(OFFSET + p.x, OFFSET + p.y, 3, 0, 2 * Math.PI);
    ctx.fill(circle);
  }
  ctx.fillStyle = "#ff0000";
  const circle = new Path2D();
  circle.arc(OFFSET, OFFSET, 3, 0, 2 * Math.PI);
  ctx.fill(circle);
};

const scale = wasm.Matrix2Float.scaleMatrix(10);

let canvas = document.getElementById("origin");
let ctx = canvas.getContext("2d");
renderList(
  ctx,
  points.map(p => scale.rightProd(p))
);

const a2p = wasm.axialToPixelMatrixPointy();

bench(
  "axial to pixel",
  () => {
    points.forEach(p => {
      p = a2p.rightProd(p);
      p = scale.rightProd(p);
    });
  },
  {
    sampleSize: 10,
    iterations: 10
  }
).then(res => {
  console.log(res);
  const summary = document.createElement("pre");
  summary.innerText = JSON.stringify(res, null, 4);
  document.getElementById("bench-results").appendChild(summary);
});

setTimeout(() => {
  const results = points.map(p => {
    p = a2p.rightProd(p);
    return scale.rightProd(p);
  });
  canvas = document.getElementById("transformed");
  ctx = canvas.getContext("2d");
  renderList(ctx, results);

  const p2a = wasm.pixelToAxialMatrixPointy();
  const scaleInv = wasm.Matrix2Float.scaleMatrix(0.1);
  canvas.addEventListener("mousemove", event => {
    const x = event.offsetX;
    const y = event.offsetY;
    let p = new wasm.Vec2Float(x-OFFSET, y-OFFSET);
    let h = scaleInv.rightProd(p);
    h = p2a.rightProd(h);
    h = wasm.roundToNearestHex(h);
    document.getElementById("transformed-hex-pos").innerText = JSON.stringify(h, null, 2);
  });
}, 0);
