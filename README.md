# GPUi Examples
This is in no way intended to be used as gospel, it's merely me exploring with design patterns within GPUi as I work towards making something more cohesive
I do try to be as "correct" as possible, but there are no guarantees (please tell me if anything is particularly egregious or if there is anything I'm missing)
I may revisit concepts upon learning there are more intentional / cleaner / more consise design patterns

Comes with gpui-component by default, mostly for ease with icons and theming.

It is structured in such a way that if you want to use this as well, it's easy to clone and add your own experiments by copying `test/template`
You can turn on / off the titlebar by changing the result of `has_custom_titlebar()` to `false` and `true` respectively.

The entry point for main is determined by what `MainWindow` is being referenced in the `use testing::...::main::MainWindow` call in `src/main.rs`
Changing this import will change which window is being used
