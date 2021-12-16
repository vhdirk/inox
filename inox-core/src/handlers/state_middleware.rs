
#[derive(Clone, Debug)]
struct Meta(usize);
impl Metadata for Meta {}

#[derive(Default)]
struct MyMiddleware(AtomicUsize);
impl Middleware<Meta> for MyMiddleware {
	type Future = FutureResponse;
	type CallFuture = middleware::NoopCallFuture;

	fn on_request<F, X>(&self, request: Request, meta: Meta, next: F) -> Either<Self::Future, X>
	where
		F: FnOnce(Request, Meta) -> X + Send,
		X: Future<Output = Option<Response>> + Send + 'static,
	{
		let start = Instant::now();
		let request_number = self.0.fetch_add(1, atomic::Ordering::SeqCst);
		println!("Processing request {}: {:?}, {:?}", request_number, request, meta);

		Either::Left(Box::pin(next(request, meta).map(move |res| {
			println!("Processing took: {:?}", start.elapsed());
			res
		})))
	}
}