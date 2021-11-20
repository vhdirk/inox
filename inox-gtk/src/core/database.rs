use std::path::Path;

pub trait DatabaseExt {
    fn find_thread_by_id(&self, id: &str) -> Result<Option<notmuch::Thread>, notmuch::Error>;
}

impl DatabaseExt for notmuch::Database {
    fn find_thread_by_id(&self, id: &str) -> Result<Option<notmuch::Thread>, notmuch::Error> {
        let query_str = format!("thread:{}", id);
        let query = self.create_query(&query_str)?;
        let mut threads = query.search_threads()?;

        Ok(threads.next())
    }
}
