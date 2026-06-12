#[allow(dead_code)]
trait FileSystemProvider {
    async fn initialize(&self);
    async fn watch_changes(&self);
    async fn metadata(&self);
}