use tokio::time::{sleep, Duration};

async fn request_model(name: &str) -> String {
    println!("{name}：开始发送请求");

    sleep(Duration::from_secs(2)).await;

    println!("{name}：收到响应");
    format!("{name} 的回答")
}

#[tokio::main]
async fn main() {
    let task_a = tokio::spawn(async {
        request_model("请求 A").await
    });

    let task_b = tokio::spawn(async {
        request_model("请求 B").await
    });

    let answer_a = task_a.await.unwrap();
    let answer_b = task_b.await.unwrap();

    println!("{answer_a}");
    println!("{answer_b}");
}