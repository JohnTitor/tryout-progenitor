#[allow(unused_imports)]
use progenitor_client::{encode_path, RequestBuilderExt};
pub use progenitor_client::{ByteStream, Error, ResponseValue};
#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};
pub mod types {
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use std::convert::TryFrom;
    ///Todo endpoint error responses
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum ErrorResponse {
        NotFound(String),
        Conflict(String),
        Unauthorized(String),
    }

    impl From<&ErrorResponse> for ErrorResponse {
        fn from(value: &ErrorResponse) -> Self {
            value.clone()
        }
    }

    ///Task to do.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Todo {
        ///Mark is the task done or not
        pub checked: bool,
        ///Unique id for the todo item.
        pub id: i32,
        ///Description of the tasks to do.
        pub value: String,
    }

    impl From<&Todo> for Todo {
        fn from(value: &Todo) -> Self {
            value.clone()
        }
    }

    ///Request to update existing `Todo` item.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TodoUpdateRequest {
        ///Optional check status to mark is the task done or not.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub checked: Option<bool>,
        ///Optional new value for the `Todo` task.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    impl From<&TodoUpdateRequest> for TodoUpdateRequest {
        fn from(value: &TodoUpdateRequest) -> Self {
            value.clone()
        }
    }
}

#[derive(Clone, Debug)]
///Client for todo-actix
///
///Simple actix-web todo example api with utoipa and Swagger UI and Redoc
///
///Version: 0.1.0
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}

impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = std::time::Duration::from_secs(15);
            reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }

    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }

    /// Get the base URL to which requests are made.
    pub fn baseurl(&self) -> &String {
        &self.baseurl
    }

    /// Get the internal `reqwest::Client` used to make requests.
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    /// Get the version of this API.
    ///
    /// This string is pulled directly from the source OpenAPI
    /// document and may be in any format the API selects.
    pub fn api_version(&self) -> &'static str {
        "0.1.0"
    }
}

impl Client {
    ///Get list of todos
    ///
    ///Get list of todos.
    ///
    ///List todos from in-memory todo store.
    ///
    ///One could call the api endpoint with following curl.
    ///```text
    /// curl localhost:8080/todo
    /// ```
    ///
    ///Sends a `GET` request to `/todo`
    pub async fn get_todos<'a>(&'a self) -> Result<ResponseValue<Vec<types::Todo>>, Error<()>> {
        let url = format!("{}/todo", self.baseurl,);
        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create new Todo to shared in-memory storage
    ///
    ///Create new Todo to shared in-memory storage.
    ///
    ///Post a new `Todo` in request body as json to store it. Api will return
    ///created `Todo` on success or `ErrorResponse::Conflict` if todo with same
    /// id already exists.
    ///
    ///One could call the api with.
    ///```text
    /// curl localhost:8080/todo -d '{"id": 1, "value": "Buy movie ticket", "checked": false}'
    /// ```
    ///
    ///Sends a `POST` request to `/todo`
    pub async fn create_todo<'a>(
        &'a self,
        body: &'a types::Todo,
    ) -> Result<ResponseValue<types::Todo>, Error<types::ErrorResponse>> {
        let url = format!("{}/todo", self.baseurl,);
        let request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            409u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Search Todos with by value
    ///
    ///Search Todos with by value
    ///
    ///Perform search from `Todo`s present in in-memory storage by matching
    /// Todo's value to value provided as query parameter. Returns 200 and
    /// matching `Todo` items.
    ///
    ///Sends a `GET` request to `/todo/search`
    ///
    ///Arguments:
    /// - `value`: Content that should be found from Todo's value field
    pub async fn search_todos<'a>(
        &'a self,
        value: &'a str,
    ) -> Result<ResponseValue<Vec<types::Todo>>, Error<()>> {
        let url = format!("{}/todo/search", self.baseurl,);
        let mut query = Vec::with_capacity(1usize);
        query.push(("value", value.to_string()));
        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get Todo by given todo id
    ///
    ///Get Todo by given todo id.
    ///
    ///Return found `Todo` with status 200 or 404 not found if `Todo` is not
    /// found from shared in-memory storage.
    ///
    ///Sends a `GET` request to `/todo/{id}`
    ///
    ///Arguments:
    /// - `id`: Unique storage id of Todo
    pub async fn get_todo_by_id<'a>(
        &'a self,
        id: i32,
    ) -> Result<ResponseValue<types::Todo>, Error<types::ErrorResponse>> {
        let url = format!("{}/todo/{}", self.baseurl, encode_path(&id.to_string()),);
        let request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Update Todo with given id
    ///
    ///Update Todo with given id.
    ///
    ///This endpoint supports optional authentication.
    ///
    ///Tries to update `Todo` by given id as path variable. If todo is found by
    /// id values are updated according `TodoUpdateRequest` and updated
    /// `Todo` is returned with status 200. If todo is not found then 404
    /// not found is returned.
    ///
    ///Sends a `PUT` request to `/todo/{id}`
    ///
    ///Arguments:
    /// - `id`: Unique storage id of Todo
    /// - `body`
    pub async fn update_todo<'a>(
        &'a self,
        id: i32,
        body: &'a types::TodoUpdateRequest,
    ) -> Result<ResponseValue<types::Todo>, Error<types::ErrorResponse>> {
        let url = format!("{}/todo/{}", self.baseurl, encode_path(&id.to_string()),);
        let request = self
            .client
            .put(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete Todo by given path variable id
    ///
    ///Delete Todo by given path variable id.
    ///
    ///This endpoint needs `api_key` authentication in order to call. Api key
    /// can be found from README.md.
    ///
    ///Api will delete todo from shared in-memory storage by the provided id
    /// and return success 200. If storage does not contain `Todo` with
    /// given id 404 not found will be returned.
    ///
    ///Sends a `DELETE` request to `/todo/{id}`
    ///
    ///Arguments:
    /// - `id`: Unique storage id of Todo
    pub async fn delete_todo<'a>(
        &'a self,
        id: i32,
    ) -> Result<ResponseValue<()>, Error<types::ErrorResponse>> {
        let url = format!("{}/todo/{}", self.baseurl, encode_path(&id.to_string()),);
        let request = self
            .client
            .delete(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

pub mod prelude {
    pub use super::Client;
}
