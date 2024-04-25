## Hi there

```rust
impl App {
    pub async fn run(&self) -> Result<()> {
        let router = router();
        let listener = tokio::net::TcpListener::bind((self.addr, self.port)).await?;

        axum::serve(listener, router).await?;

        Ok(())
    }
}
```
