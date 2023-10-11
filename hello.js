import init, { Trie } from "./pkg/trie_wasm.js";

init().then(() => {
  const t = new Trie();
  t.insert("hello");
  t.insert("hello1");
  t.insert("hello2");
  t.insert("hello3");
  t.insert("hello4");
  t.insert("hello5");
  t.insert("hello6");
  t.insert("hello7");
  console.log(t.prefix("hello"));
});
