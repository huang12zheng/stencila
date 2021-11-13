use eyre::{bail, Result};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::{Mutex, MutexGuard};
use tokio::sync::mpsc;

/// An event updating progress of some task
#[derive(Default, Debug, Deserialize, Serialize)]
pub struct ProgressEvent {
    /// The id of the task that this progress event relates to
    pub id: Option<String>,

    /// The id of the parent task (if any)
    pub parent: Option<String>,

    /// The event message
    pub message: Option<String>,

    /// The current value
    pub current: Option<i64>,

    /// The expected value when complete
    pub expected: Option<i64>,

    // Whether or not the task is complete
    pub complete: bool,
}

pub type Message = (String, serde_json::Value);

pub enum Subscriber {
    Function(fn(topic: String, event: serde_json::Value) -> ()),
    Sender(mpsc::UnboundedSender<Message>),
}

struct Subscription {
    topic: String,
    subscriber: Subscriber,
}

static SUBSCRIPTIONS: Lazy<Mutex<Vec<Subscription>>> = Lazy::new(|| Mutex::new(Vec::new()));

/// Obtain the subscriptions store
fn obtain() -> Result<MutexGuard<'static, Vec<Subscription>>> {
    // Use `lock`, not `try_lock`, which means this thread may block waiting
    // a little for the subscriptions to become available.
    match SUBSCRIPTIONS.lock() {
        Ok(guard) => Ok(guard),
        Err(error) => bail!("While attempting to obtain subscriptions: {}", error),
    }
}

/// Subscribe to a topic
pub fn subscribe(topic: &str, subscriber: Subscriber) -> Result<()> {
    tracing::debug!("Subscribing to topic: {}", topic);

    match obtain() {
        Ok(mut subscriptions) => {
            subscriptions.push(Subscription {
                topic: topic.to_string(),
                subscriber,
            });
            Ok(())
        }
        Err(error) => {
            bail!("Unable to subscribe: {}", error.to_string())
        }
    }
}

/// Publish an event for a topic
///
/// Publishing an event should be treated as 'fire-and-forget'.
/// This function does not return an `Err` if it fails but will
/// log an error (if not already attempting to publish to logging channel).
pub fn publish<Event>(topic: &str, event: Event)
where
    Event: Serialize,
{
    match obtain() {
        Ok(subscriptions) => {
            for subscription in &*subscriptions {
                if subscription.topic == "*"
                    || subscription.topic == topic
                    || topic.starts_with(&subscription.topic)
                {
                    let value = serde_json::to_value(&event).unwrap_or(serde_json::Value::Null);
                    match &subscription.subscriber {
                        Subscriber::Function(function) => {
                            function(topic.into(), value);
                        }
                        Subscriber::Sender(sender) => {
                            if let Err(error) = sender.send((topic.into(), value)) {
                                tracing::error!("Error sending event: {}", error);
                            }
                        }
                    }
                }
            }
        }
        Err(error) => {
            // Do not log error if the topic is logging since that could lead to recursion
            if topic != "logging" {
                tracing::error!("Unable to publish event: {}", error)
            }
        }
    }
}