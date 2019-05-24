## TODO

* Shift and arrow keys to select text
    * Allow highlighting text/changing the background
      * Currently we send down an `Option<StatusLineInfo<V>>`. I think instead
      we should send down an `Option<HighlightInfo>` where a `HighlightInfo`
      contains whether each glyph should be highlighted or not. Then if a glyph
      is highlighted we queue a "█" glyph at its position.
      * Will drawing up to 2x glyphs be a perf issue? Do we care about merging
      adjacent glyphs? Would it be better to talk about highlighted ranges?

    * Highlight only between a fixed region
    * allow multiple, dynamic, regions
    * replace text inside regions when typing
