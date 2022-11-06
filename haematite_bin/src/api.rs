use std::convert::Infallible;
use std::sync::{Arc, Mutex, RwLock};

use closure::closure;
use futures::{SinkExt as _, StreamExt as _};
use sqlx::Database as SqlxDatabase;
use tokio::sync::broadcast::Receiver;
use warp::ws::{Message, Ws};
use warp::{Filter, Reply as _};

use haematite_api::{Api, Error as ApiError, Format};
use haematite_dal::Database;
use haematite_models::config::Config;
use haematite_models::irc::network::Network;
use haematite_models::meta::permissions::Path;
use haematite_models::meta::user::User;
use haematite_ser::WrapType;

#[derive(Debug)]
enum Error {
    Unauthorized,
    Api(ApiError),
}

impl warp::reject::Reject for Error {}

impl From<ApiError> for Error {
    fn from(e: ApiError) -> Self {
        Self::Api(e)
    }
}

fn authorization<D: SqlxDatabase>(
    database: Arc<Database<D>>,
) -> impl Filter<Extract = (User,), Error = warp::Rejection> + Clone {
    warp::header::<String>("Authorization").and_then(move |token: String| {
        let database = Arc::clone(&database);
        async move {
            match database.user_store.access_token(&token).await {
                Some(user) => Ok(user),
                None => Err(warp::reject::custom(Error::Unauthorized)),
            }
        }
    })
}

#[allow(clippy::unused_async)]
async fn recover(
    err: warp::Rejection,
) -> Result<(Result<warp::reply::Response, Error>,), warp::Rejection> {
    if let Some(Error::Unauthorized) = err.find() {
        Ok((Ok(warp::reply::with_status(
            "bad access token".to_string(),
            warp::http::StatusCode::UNAUTHORIZED,
        )
        .into_response()),))
    } else {
        Err(err)
    }
}

fn display(res: Result<warp::reply::Response, Error>) -> warp::reply::Response {
    match res {
        Ok(response) => response,
        Err(e) => warp::reply::with_status(
            format!("unknown error: {:?}", e),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response(),
    }
}

pub async fn run<D: SqlxDatabase>(
    config: &Config,
    network: Arc<RwLock<Network>>,
    stream: Receiver<(Path, WrapType)>,
    database: Database<D>,
) -> Result<(), Infallible> {
    let stream = Arc::new(Mutex::new(stream));
    let database = Arc::new(database);
    let api = Arc::new(Api::new(network, Format::Pretty));

    let path_network = warp::path!("rest" / "network")
        .and(authorization(Arc::clone(&database)))
        .map(closure!(clone api, |user| {
            Ok(api.get_network(&user)?.into_response())
        }));

    let path_user = warp::path!("rest" / "user")
        .and(authorization(Arc::clone(&database)))
        .and(warp::path::param())
        .map(closure!(clone api, |user, uid: String| {
            Ok(api.get_user(&user, &uid)?.into_response())
        }));

    let path_stream = warp::path("stream")
        .and(authorization(Arc::clone(&database)))
        .and(warp::ws())
        .map(move |user: User, ws1: Ws| {
            let mut stream = stream.lock().unwrap().resubscribe();
            Ok(ws1
                .on_upgrade(|ws2| async move {
                    let (mut tx, _rx) = ws2.split();

                    while let Ok((path, mut value)) = stream.recv().await {
                        if let Some(tree) = user.permissions.walk(&path) {
                            value.update_with(tree);

                            let serialized = serde_json::to_string(&value).unwrap();
                            if tx
                                .send(Message::text(format!(
                                    "{} {}",
                                    path.to_string(),
                                    serialized
                                )))
                                .await
                                .is_err()
                            {
                                break;
                            }
                        }
                    }
                })
                .into_response())
        });

    warp::serve(
        path_network
            .or(path_user)
            .unify()
            .or(path_stream)
            .unify()
            .or_else(recover)
            .map(display),
    )
    .run(config.bind)
    .await;

    Ok(())
}
