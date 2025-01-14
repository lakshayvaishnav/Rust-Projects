use error_chain::error_chain;

// to handle errors
error_chain! {
    foreign_links{
        Io(std::io::Error);
        Httprequest(reqwest::Error);
    }
}
#[tokio::main]
async fn main() -> Result<()> {
    let res = reqwest::get("http://httpbin.org/get").await?;
    println!("Status : {}", res.status());
    println!("headers : \n{:?}", res.headers());
    let body = res.text().await?;

    println!("Body : \n{}", body);
    
    
    Ok(())
}
