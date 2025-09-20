# Leptos-events-testing

Shows an example of how to use leptos forms.

## Rules

- Use the [leptos builder syntax](https://book.leptos.dev/view/builder.html). We can then benefit from cargo format, better error messages, no macros, and not have to write pseudo-html.
- Only pass down static data (not signals), and only push up events. This makes components easy to test.
- The events you push up should be well-typed objects, not html events.
