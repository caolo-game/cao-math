# Getting Started

Add _CaoMath_ to your _package.json_

```bash
npm i @caolo-game/cao-math
```

---

Initialize it in your code.

```javascript
import * as caoMath from "@caolo-game/cao-math";
// register error handling hooks once
caoMath.init_error_handling();

function foo() {
  const a = new caoMath.Vec2f(42, 13);
  const b = new caoMath.Vec2f(-13, 42);

  return a.dot(b);
}
```

---

## Integrating with React

You can create a file that provides CaoMath

```javascript
export var caoMath = null;
const caoMathImport = import("@caolo-game/cao-math")
  .then((cao) => {
    cao.init_error_handling();
    return (caoMath = cao);
  })
  .catch(console.error);

export const useCaoMath = () => {
  const [cao, setCao] = useState(caoMath);
  const [err, setErr] = useState(null);
  caoMathImport
    .then((c) => setCao(c))
    .catch((e) => {
      console.error("Failed to load cao math", e);
      setErr(e);
    });
  return [cao, err];
};
```

```javascript
import { caoMath, useCaoMath } from "./CaoProvider.jsx";

function Foo() {
  const [loadedCaoMath, caoMathError] = useCaoMath();
  if (caoMathError) return "Error loading cao-math";
  if (!loadedCaoMath) return "loading cao-math...";
  return <Bar />;
}

function Bar() {
  // Bar assumes that caoMath has been loaded successfully at this point

  const a = new caoMath.Vec2f(42, 13);
  const b = new caoMath.Vec2f(-13, 42);

  return <pre>JSON.stringify(a.dot(b), null, 4)</pre>;
}
```
