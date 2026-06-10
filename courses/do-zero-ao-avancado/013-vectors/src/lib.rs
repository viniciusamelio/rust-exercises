use std::sync::Mutex;

pub mod post;

static _POSTS: Mutex<Vec<post::Post>> = Mutex::new(Vec::new());

pub fn get_posts() -> Vec<post::Post> {
    _POSTS.lock().unwrap().clone()
}

pub fn get_post_at(index: usize) -> Option<post::Post> {
    _POSTS.lock().unwrap().get(index).cloned()
}

pub fn create_post(id: String, title: String, content: String) -> post::Post {
    let post = post::Post::new(id, title, content);
    _POSTS.lock().unwrap().push(post.clone());
    post
}
