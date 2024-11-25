#[allow(warnings, unused)]
mod prisma;
use prisma::PrismaClient;
use prisma_client_rust::NewClientError;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建客户端
    let client = PrismaClient::_builder().build().await?;

    // 创建一个新的帖子
    let post = client
        .post()
        .create(
            "My first post".to_string(),
            "Hello, Prisma!".to_string(),
            vec![prisma::post::published::set(true)],
        )
        .exec()
        .await?;

    println!("Created post: {:?}", post);

    // 为帖子添加评论
    let comment = client
        .comment()
        .create(
            "Great post!".to_string(),
            prisma::post::id::equals(post.id.clone()),
            vec![],
        )
        .exec()
        .await?;

    println!("Created comment: {:?}", comment);

    // 查询带有评论的帖子
    let posts = client
        .post()
        .find_many(vec![])
        .with(prisma::post::comments::fetch(vec![]))
        .exec()
        .await?;

    println!("All posts with comments: {:?}", posts);

    Ok(())
} 