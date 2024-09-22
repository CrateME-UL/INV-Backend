use domain::Item;
//WOW: we are on the good path!
pub trait FetchItems {
    fn fetch_items(
    ) -> impl std::future::Future<Output = Result<Vec<Item>, Box<dyn std::error::Error>>> + Send;
}
