## TODO


* figure out where the latency when pressing shift is coming from.
  * print out time it takes on editor thread and on render thread to figure out which needs optimizing.
  * write perf test that ensures that no `editor` action takes less than x ms?

* fix cursor being visible through status line (!?)

* find or write `less` like program that allows toggling between full view of stdout, only those lines that match a multiline regex or whatever and only those that don't.
  * display input logs`[libs\editor\./src/editor.rs:148] input` etc
