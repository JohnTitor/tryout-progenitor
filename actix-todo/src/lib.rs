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

    impl Todo {
        pub fn builder() -> builder::Todo {
            builder::Todo::default()
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

    impl TodoUpdateRequest {
        pub fn builder() -> builder::TodoUpdateRequest {
            builder::TodoUpdateRequest::default()
        }
    }

    pub mod builder {
        #[derive(Clone, Debug)]
        pub struct Todo {
            checked: Result<bool, String>,
            id: Result<i32, String>,
            value: Result<String, String>,
        }

        impl Default for Todo {
            fn default() -> Self {
                Self {
                    checked: Err("no value supplied for checked".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    value: Err("no value supplied for value".to_string()),
                }
            }
        }

        impl Todo {
            pub fn checked<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<bool>,
                T::Error: std::fmt::Display,
            {
                self.checked = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for checked: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<i32>,
                T::Error: std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn value<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.value = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for value: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<Todo> for super::Todo {
            type Error = String;
            fn try_from(value: Todo) -> Result<Self, String> {
                Ok(Self {
                    checked: value.checked?,
                    id: value.id?,
                    value: value.value?,
                })
            }
        }

        impl From<super::Todo> for Todo {
            fn from(value: super::Todo) -> Self {
                Self {
                    checked: Ok(value.checked),
                    id: Ok(value.id),
                    value: Ok(value.value),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct TodoUpdateRequest {
            checked: Result<Option<bool>, String>,
            value: Result<Option<String>, String>,
        }

        impl Default for TodoUpdateRequest {
            fn default() -> Self {
                Self {
                    checked: Ok(Default::default()),
                    value: Ok(Default::default()),
                }
            }
        }

        impl TodoUpdateRequest {
            pub fn checked<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.checked = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for checked: {}", e));
                self
            }
            pub fn value<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.value = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for value: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<TodoUpdateRequest> for super::TodoUpdateRequest {
            type Error = String;
            fn try_from(value: TodoUpdateRequest) -> Result<Self, String> {
                Ok(Self {
                    checked: value.checked?,
                    value: value.value?,
                })
            }
        }

        impl From<super::TodoUpdateRequest> for TodoUpdateRequest {
            fn from(value: super::TodoUpdateRequest) -> Self {
                Self {
                    checked: Ok(value.checked),
                    value: Ok(value.value),
                }
            }
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

pub trait ClientTodoExt {
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
    ///
    ///```ignore
    /// let response = client.get_todos()
    ///    .send()
    ///    .await;
    /// ```
    fn get_todos(&self) -> builder::GetTodos;
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
    ///
    ///```ignore
    /// let response = client.create_todo()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn create_todo(&self) -> builder::CreateTodo;
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
    ///```ignore
    /// let response = client.search_todos()
    ///    .value(value)
    ///    .send()
    ///    .await;
    /// ```
    fn search_todos(&self) -> builder::SearchTodos;
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
    ///```ignore
    /// let response = client.get_todo_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    fn get_todo_by_id(&self) -> builder::GetTodoById;
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
    ///```ignore
    /// let response = client.update_todo()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn update_todo(&self) -> builder::UpdateTodo;
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
    ///```ignore
    /// let response = client.delete_todo()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    fn delete_todo(&self) -> builder::DeleteTodo;
}

impl ClientTodoExt for Client {
    fn get_todos(&self) -> builder::GetTodos {
        builder::GetTodos::new(self)
    }

    fn create_todo(&self) -> builder::CreateTodo {
        builder::CreateTodo::new(self)
    }

    fn search_todos(&self) -> builder::SearchTodos {
        builder::SearchTodos::new(self)
    }

    fn get_todo_by_id(&self) -> builder::GetTodoById {
        builder::GetTodoById::new(self)
    }

    fn update_todo(&self) -> builder::UpdateTodo {
        builder::UpdateTodo::new(self)
    }

    fn delete_todo(&self) -> builder::DeleteTodo {
        builder::DeleteTodo::new(self)
    }
}

pub mod builder {
    use super::types;
    #[allow(unused_imports)]
    use super::{
        encode_path, ByteStream, Error, HeaderMap, HeaderValue, RequestBuilderExt, ResponseValue,
    };
    ///Builder for [`ClientTodoExt::get_todos`]
    ///
    ///[`ClientTodoExt::get_todos`]: super::ClientTodoExt::get_todos
    #[derive(Debug, Clone)]
    pub struct GetTodos<'a> {
        client: &'a super::Client,
    }

    impl<'a> GetTodos<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client }
        }

        ///Sends a `GET` request to `/todo`
        pub async fn send(self) -> Result<ResponseValue<Vec<types::Todo>>, Error<()>> {
            let Self { client } = self;
            let url = format!("{}/todo", client.baseurl,);
            let request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`ClientTodoExt::create_todo`]
    ///
    ///[`ClientTodoExt::create_todo`]: super::ClientTodoExt::create_todo
    #[derive(Debug, Clone)]
    pub struct CreateTodo<'a> {
        client: &'a super::Client,
        body: Result<types::builder::Todo, String>,
    }

    impl<'a> CreateTodo<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client,
                body: Ok(types::builder::Todo::default()),
            }
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::Todo>,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|_| "conversion to `Todo` for body failed".to_string());
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::Todo) -> types::builder::Todo,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/todo`
        pub async fn send(self) -> Result<ResponseValue<types::Todo>, Error<types::ErrorResponse>> {
            let Self { client, body } = self;
            let body = body
                .and_then(std::convert::TryInto::<types::Todo>::try_into)
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/todo", client.baseurl,);
            let request = client
                .client
                .post(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                201u16 => ResponseValue::from_response(response).await,
                409u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`ClientTodoExt::search_todos`]
    ///
    ///[`ClientTodoExt::search_todos`]: super::ClientTodoExt::search_todos
    #[derive(Debug, Clone)]
    pub struct SearchTodos<'a> {
        client: &'a super::Client,
        value: Result<Option<String>, String>,
    }

    impl<'a> SearchTodos<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client,
                value: Ok(None),
            }
        }

        pub fn value<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.value = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for value failed".to_string());
            self
        }

        ///Sends a `GET` request to `/todo/search`
        pub async fn send(self) -> Result<ResponseValue<Vec<types::Todo>>, Error<()>> {
            let Self { client, value } = self;
            let value = value.map_err(Error::InvalidRequest)?;
            let url = format!("{}/todo/search", client.baseurl,);
            let mut query = Vec::with_capacity(1usize);
            if let Some(v) = &value {
                query.push(("value", v.to_string()));
            }
            let request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&query)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`ClientTodoExt::get_todo_by_id`]
    ///
    ///[`ClientTodoExt::get_todo_by_id`]: super::ClientTodoExt::get_todo_by_id
    #[derive(Debug, Clone)]
    pub struct GetTodoById<'a> {
        client: &'a super::Client,
        id: Result<i32, String>,
    }

    impl<'a> GetTodoById<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client,
                id: Err("id was not initialized".to_string()),
            }
        }

        pub fn id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i32>,
        {
            self.id = value
                .try_into()
                .map_err(|_| "conversion to `i32` for id failed".to_string());
            self
        }

        ///Sends a `GET` request to `/todo/{id}`
        pub async fn send(self) -> Result<ResponseValue<types::Todo>, Error<types::ErrorResponse>> {
            let Self { client, id } = self;
            let id = id.map_err(Error::InvalidRequest)?;
            let url = format!("{}/todo/{}", client.baseurl, encode_path(&id.to_string()),);
            let request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`ClientTodoExt::update_todo`]
    ///
    ///[`ClientTodoExt::update_todo`]: super::ClientTodoExt::update_todo
    #[derive(Debug, Clone)]
    pub struct UpdateTodo<'a> {
        client: &'a super::Client,
        id: Result<i32, String>,
        body: Result<types::builder::TodoUpdateRequest, String>,
    }

    impl<'a> UpdateTodo<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client,
                id: Err("id was not initialized".to_string()),
                body: Ok(types::builder::TodoUpdateRequest::default()),
            }
        }

        pub fn id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i32>,
        {
            self.id = value
                .try_into()
                .map_err(|_| "conversion to `i32` for id failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TodoUpdateRequest>,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|_| "conversion to `TodoUpdateRequest` for body failed".to_string());
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::TodoUpdateRequest,
            ) -> types::builder::TodoUpdateRequest,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `PUT` request to `/todo/{id}`
        pub async fn send(self) -> Result<ResponseValue<types::Todo>, Error<types::ErrorResponse>> {
            let Self { client, id, body } = self;
            let id = id.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(std::convert::TryInto::<types::TodoUpdateRequest>::try_into)
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/todo/{}", client.baseurl, encode_path(&id.to_string()),);
            let request = client
                .client
                .put(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`ClientTodoExt::delete_todo`]
    ///
    ///[`ClientTodoExt::delete_todo`]: super::ClientTodoExt::delete_todo
    #[derive(Debug, Clone)]
    pub struct DeleteTodo<'a> {
        client: &'a super::Client,
        id: Result<i32, String>,
    }

    impl<'a> DeleteTodo<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client,
                id: Err("id was not initialized".to_string()),
            }
        }

        pub fn id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i32>,
        {
            self.id = value
                .try_into()
                .map_err(|_| "conversion to `i32` for id failed".to_string());
            self
        }

        ///Sends a `DELETE` request to `/todo/{id}`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<types::ErrorResponse>> {
            let Self { client, id } = self;
            let id = id.map_err(Error::InvalidRequest)?;
            let url = format!("{}/todo/{}", client.baseurl, encode_path(&id.to_string()),);
            let request = client
                .client
                .delete(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
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
}

pub mod prelude {
    pub use super::Client;
    pub use super::ClientTodoExt;
}
