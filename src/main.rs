

use tokio;
use tokio::io::{stdout, AsyncWriteExt as _};
use tokio::fs::File;
use hyper::Client;
use hyper::body::HttpBody as _;
use hyper_tls::HttpsConnector;
use hyper::client::connect::HttpConnector;

use hyper::body::Buf;

use std::any::type_name;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let worker = Worker::new();

    let urls = vec!("https://pbs.twimg.com/media/E34QpTpUYAECYzi?format=jpg&name=small", 
    );
    for (_idx, url) in urls.iter().enumerate() {
        // yet without MIME-type checking
        worker.fetch(url, &format!("test.jpg")).await?;
    }

    
    // By running all async expressions on the current task, the expressions are able to run concurrently but not in parallel.
    // This means all expressions are run on the same thread and if one branch blocks the thread, all other expressions will be unable to continue. 
    // // If parallelism is required, spawn each async expression using tokio::spawn and pass the join handle to join!.

    Ok(())
}

struct Worker {
    client: Client<HttpsConnector<HttpConnector>, hyper::Body>
}

impl Worker {
    pub fn new() -> Worker {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        Worker {
            client
        }
    }

    pub async fn fetch(&self, url: &str, filename: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let uri: hyper::Uri = url.parse()?;

        let mut response = self.client.get(uri).await?;
    
        println!("Response: {}", response.status());
    
        let header_map = response.headers();
        for (key, value) in header_map.iter() {
            println!("{:?}, {:?}", key, value);
        }
    
        let mut file = File::create(filename).await?;
    
        print_type(&response.body_mut().data());
    
        while let Some(chunk) = response.body_mut().data().await {
            let buf = chunk?;
            // print_type(&buf);
    
            // stdout().write_all(&buf).await?;
    
            let arr = buf.chunk();
            // print_type(&arr);
    
            // println!("{}, {:?}", arr.len(), &arr);
            println!("chunk size: {}", arr.len());
            file.write_all(arr).await?;
        }

        Ok(())
    }
}



pub fn print_type<T>(_: &T) {
    println!("{}", type_name::<T>());
}