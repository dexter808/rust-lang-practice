use blog::Post;

fn main() {
    let mut post = Post::new();
    let init_content = "I had green tea this morning.";

    post.add_text(init_content);
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!(init_content, post.content());
}
