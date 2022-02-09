mod error;
pub mod model;

use error::Error;

use async_trait::async_trait;

#[async_trait]
pub trait Datasource {
    async fn list_crts(&self, domain: &str) -> Result<model::Crts, Error>;
}

pub struct CrtShDatasource {
    base_url: String,
    http_client: reqwest::Client,
}

impl CrtShDatasource {
    pub fn new(http_client: reqwest::Client) -> Self {
        Self {
            base_url: String::from("https://crt.sh"),
            http_client,
        }
    }
}

#[async_trait]
impl Datasource for CrtShDatasource {
    async fn list_crts(&self, domain: &str) -> Result<model::Crts, Error> {
        let url = &format!("{}/?q=%.{}&output=json", self.base_url, domain);
        let res = self.http_client.get(url).send().await?;

        match res.status() {
            reqwest::StatusCode::OK => {
                let crts = res.json().await?;

                Ok(crts)
            }
            _ => Err(Error::RequestFailed(String::from("Something went wrong!"))),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
