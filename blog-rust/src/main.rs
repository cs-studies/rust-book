use blog_rust::Post;

fn main() {
    let mut post = Post::new();
    let text = "My fancy text.";

    post.add_text(text);

    let post = post.request_review();
    let post = post.approve();

    assert_eq!(text, post.content());

    println!("The content is '{}'", post.content());
}
