use blog::Post;

fn main() {
    let mut draft_post = Post::new();
    let init_content = "I had green tea this morning.";

    draft_post.add_text(init_content);

    let pending_review_post = draft_post.request_review();

    assert_eq!(init_content, pending_review_post.approve().content());
}
