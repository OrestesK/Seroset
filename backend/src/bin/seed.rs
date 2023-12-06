use backend::prisma;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = prisma::new_client().await.expect("Failed to create client");

    let user = client
        .user()
        .create(String::from("username"), String::from("Webrc"), vec![])
        .exec()
        .await?;

    println!("Created user: {:?}", user);

    Ok(())
}
