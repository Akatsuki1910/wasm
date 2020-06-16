const js = import("../pkg/index.js").catch(console.error);
js.then(js => {
  js.greet("WebAssembly");
});
console.log(123);