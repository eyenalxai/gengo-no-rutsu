use crate::health_check;
use axum::routing::get;
use std::convert::Infallible;
use teloxide::dispatching::update_listeners::webhooks::{axum_to_router, Options};
use teloxide::dispatching::update_listeners::UpdateListener;
use teloxide::requests::Requester;

pub async fn build_listener<R>(
    bot: R,
    options: Options,
) -> Result<impl UpdateListener<Err = Infallible>, R::Err>
where
    R: Requester + Send + 'static,
    <R as Requester>::DeleteWebhook: Send,
{
    let Options { address, .. } = options;

    let (mut update_listener, stop_flag, app) = axum_to_router(bot, options).await?;
    let stop_token = update_listener.stop_token();

    let app_health_check = app.route("/heath", get(health_check));

    tokio::spawn(async move {
        axum::Server::bind(&address)
            .serve(app_health_check.into_make_service())
            .with_graceful_shutdown(stop_flag)
            .await
            .map_err(|err| {
                stop_token.stop();
                err
            })
            .expect("Axum server error");
    });

    Ok(update_listener)
}
