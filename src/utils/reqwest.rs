use serde::{
    de::DeserializeOwned, 
    //ser::Error
};
use serde::{
    //Deserialize, 
    Serialize
};
use std::fmt::Debug;
use std::result;
use std::sync::Arc;


struct ReqResult<T> {
    code: Arc<u16>,
    body: Arc<T>,
}


async fn request<U, T> (
    url: String, 
    method: reqwest::Method, 
    body: &U,
    is_auth: bool
) -> Result<T, u16>
where
    T: DeserializeOwned + Debug + Send,
    U: Serialize + Debug,
    //T: DeserializeOwned + Send,
    //U: Serialize,
{ 
    let allow_body = method == reqwest::Method::POST || method == reqwest::Method::PUT;
    let mut req = reqwest::Client::new()
        .request(method, url)
        .header("Content-Type", "application/json");

    if is_auth {
        if let Some(token) = get_token(){
            req = req.bearer_auth(token);
        }
    }

    if allow_body { 
        req = req.json(body);
    }
    println!("=============");
    println!("Req: {:?}", req);
    let res_resp = req.send().await;
    println!("Resp: {:?}", res_resp);
    println!("=============");
    match res_resp {
        Ok(resp) => {

        match resp.status().is_success(){
            true => {
                match resp.json::<T>().await{
                    Ok(data) => Ok(data),
                    Err(_) => {
                        println!("Failed parse body");
                        Err(0)
                    },
                }
            },
            false => {
                Err(resp.status().as_u16())
            }
        }
    },
        Err(err) => {
            println!("err: {:?}", err);
            Err(0)
        }
    }
}

pub async fn request_delete<T>(url: String, is_auth: bool) -> Result<T, u16>
where
    T: DeserializeOwned + 'static + std::fmt::Debug + Send,
    //T: DeserializeOwned + 'static + Send,
{
    request(url, reqwest::Method::DELETE, &(), is_auth).await
}

/// Get request
pub async fn request_get<T>(url: String, is_auth: bool) -> Result<T, u16>
where
    T: DeserializeOwned + 'static + std::fmt::Debug + Send,
    //T: DeserializeOwned + 'static + Send,
{
    request(url, reqwest::Method::GET, &(), is_auth).await
}

/// Post request with a body
pub async fn request_post<U, T>(url: String, body: &U, is_auth: bool) -> Result<T, u16>
where
    T: DeserializeOwned + 'static + std::fmt::Debug + Send,
    U: Serialize + std::fmt::Debug,
    //T: DeserializeOwned + 'static + Send,
    //U: Serialize,
{
    request(url, reqwest::Method::POST, body, is_auth).await
}

/// Put request with a body
pub async fn request_put<U, T>(url: String, body: &U, is_auth: bool) -> Result<T, u16>
where
    T: DeserializeOwned + 'static + std::fmt::Debug + Send,
    U: Serialize + std::fmt::Debug,
    //T: DeserializeOwned + 'static + Send,
    //U: Serialize,
{
    request(url, reqwest::Method::PUT, body, is_auth).await
}