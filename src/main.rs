use leptos::{
    attr::Value,
    ev::{self, SubmitEvent},
    html::*,
    prelude::*,
};

#[derive(Clone, Debug)]
struct CommentForm {
    comment: String,
    likes: i32,
}

fn main() {
    leptos::mount::mount_to_body(|| app())
}

fn app() -> impl IntoView {
    let comment_form_data = RwSignal::new(None);

    div().child((
        h1().child("A sample form"),
        // The event pushed up is being used to set a signal directly,
        // although instead it would more likely do an API call, the results setting the signal
        comment_form(move |e| comment_form_data.set(Some(e))),
        move || submitted_comment_form(comment_form_data.get()),
    ))
}

/// An example of a show if component. Don't pass signals for static components, use move in the level above.
/// This makes testing and debugging much easier.
fn submitted_comment_form(comment_form: Option<CommentForm>) -> impl IntoView {
    comment_form.map(|cf| {
        div().child(format!(
            "Submitted Comment: {}, Likes: {}",
            cf.comment, cf.likes
        ))
    })
}

/// A sample comment form, which pushes up an on_submit event with the form data.
fn comment_form(on_submit: impl Fn(CommentForm) + 'static) -> impl IntoView {
    let comment = RwSignal::new("".to_string());

    // The browser seems to demand strings, no numbers.
    let likes = RwSignal::new("".to_string());

    let form_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        on_submit(CommentForm {
            comment: comment.get(),
            likes: likes.get().parse().expect("is number"),
        })
    };

    div().child(
        form().on(ev::submit, form_submit).child((
            input().bind(Value, comment).placeholder("a comment"),
            input()
                .attr("type", "number")
                .bind(Value, likes)
                .placeholder("number of likes"),
            button().attr("type", "submit").child("Submit"),
        )),
    )
}
