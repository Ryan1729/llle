## TODO

* Make/find general purpose "print this if it changed" function/macro.
    * This enables easy logging of things that rarely change, yet are accessed/recalculated as often as once per frame.
    * the first question is "how do we determine equality?". I think the answer, although not performant, is to generate the string and see if it is any different.
    * Make a crate containing a static `HashMap<&'static str, String>` and provide macros that resemble `println!` and `print!` which call a function with keys generated using `file!` and `line!` and values based on the specified string. in the function, if the previous value for the keys do not match the current value, then print the new string and store it in the old ones place.

* Put the status line in the right spot with the help of the above function
