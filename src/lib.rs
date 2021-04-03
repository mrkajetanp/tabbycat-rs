use hyper::Client;
use hyper_tls::HttpsConnector;

pub struct Tabbycat {
    endpoint: String,
    private_urls: String,
    api_key: &'static str,
    client: Client<HttpsConnector<hyper::client::HttpConnector>>,
}

impl Tabbycat {
    pub fn new(url: &'static str, slug: &'static str, api_key: &'static str) -> Tabbycat {
        let https = HttpsConnector::new();

        Tabbycat {
            endpoint: format!("{}/api/v1/tournaments/{}", url, slug),
            private_urls: format!("{}/{}/privateurls/", url, slug),
            api_key,
            client: Client::builder().build::<_, hyper::Body>(https),
        }
    }

    pub async fn make_request(&self) -> Result<String, &'static str> {
        let uri = self.endpoint.parse().unwrap();

        println!("Endpoint: {:?}", self.endpoint);
        let response = self.client.get(uri).await;

        match response {
            Ok(res) => {
                let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
                let result = String::from_utf8(bytes.into_iter().collect()).expect("");
                Ok(result)
            }
            Err(err) => {
                println!("Error: {:?}", err);
                Err("Error occurred")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn http_connects() {
        let tabbycat = Tabbycat::new(
            "https://workinprogressopen2021.herokuapp.com",
            "necropen",
            "xxx",
        );

        println!("{:?}", tabbycat.make_request().await);
    }
}
