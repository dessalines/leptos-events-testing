use leptos::{
    attr::Value,
    ev::{self, SubmitEvent},
    html::*,
    prelude::*,
};

fn main() {
    leptos::mount::mount_to_body(|| app())
}

fn app() -> impl IntoView {
    let comment_form = RwSignal::new(None);

    div().child((form_1(move |e| comment_form.set(Some(e))), move || {
        show_submitted_comment_form(comment_form.get())
    }))
}

fn show_submitted_comment_form(comment_form: Option<CommentForm>) -> impl IntoView {
    comment_form.map(|cf| div().child(format!("Comment: {}, Likes: {}", cf.comment, cf.likes)))
}

#[derive(Clone, Debug)]
struct CommentForm {
    comment: String,
    likes: i32,
}

fn form_1(on_save: impl Fn(CommentForm) + 'static) -> impl IntoView {
    let comment = RwSignal::new("".to_string());
    let likes = RwSignal::new("".to_string());

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        on_save(CommentForm {
            comment: comment.get(),
            likes: likes.get().parse().expect("is number"),
        })
    };

    div().child(form().on(ev::submit, on_submit).child((
        input().bind(Value, comment),
        input().attr("type", "number").bind(Value, likes),
        button().attr("type", "submit").child("Save"),
    )))
}
