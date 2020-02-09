import * as wasm from "cao-math";

wasm.init_error_handling();

const points = [];
const RADIUS = 32;
for (let i = -RADIUS; i <= RADIUS; ++i) {
  for (let j = -RADIUS; j <= RADIUS; ++j) {
    points.push(new wasm.Vec2Float(i, j));
  }
}

const renderList = (ctx, points) => {
  ctx.fillStyle = "#000000";
  for (let p of points) {
    const circle = new Path2D();
    circle.arc(145 + p.x, 145 + p.y, 5, 0, 2 * Math.PI);
    ctx.fill(circle);
  }
  ctx.fillStyle = "#ff0000";
  const circle = new Path2D();
  circle.arc(145, 145, 5, 0, 2 * Math.PI);
  ctx.fill(circle);
};

const scale = wasm.Matrix2Float.scaleMatrix(10);

let canvas = document.getElementById("origin");
let ctx = canvas.getContext("2d");
renderList(
  ctx,
  points.map(p => scale.rightProd(p))
);

const a2p = wasm.axialToPixelMatrix();
console.time("calculate projections");
let results = points.map(p => {
  p = a2p.rightProd(p);
  return scale.rightProd(p);
});
console.timeEnd("calculate projections");
console.log(results);
console.time("calculate projections2");
results = [
  ...results,
  ...points.map(p => {
    p = a2p.rightProd(p);
    return scale.rightProd(p);
  })
];
console.timeEnd("calculate projections2");
console.log(results);

canvas = document.getElementById("transformed");
ctx = canvas.getContext("2d");

renderList(ctx, results);
