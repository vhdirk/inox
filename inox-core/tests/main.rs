use notmuch;
use dirs;

fn main() {

    let mut mail_path = dirs::home_dir().unwrap();
    mail_path.push(".mail");

    let _db = notmuch::Database::open(&mail_path.to_str().unwrap().to_string(), notmuch::DatabaseMode::ReadOnly).unwrap();



        // Ok(db) => {
            
        //     #[cfg(feature = "v0_21")]
        //     {
        //         let rev = db.revision();
        //         println!("db revision: {:?}", rev);
        //     }
            
        //     let query = db.create_query(&"".to_string()).unwrap();
        //     let mut threads = query.search_threads().unwrap();

        //     loop {
        //         match threads.next() {
        //             Some(thread) => {
        //                 println!("thread {:?} {:?}", thread.subject(), thread.authors());
        //             },
        //             None => { break }
        //         }
        //     }

        // },
        // Err(err) =>{
        //     println!("Got error while trying to open db: {:?}", err);
        // }
    
}