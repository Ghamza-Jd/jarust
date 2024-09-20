use crate::error::JaTransportError;
use crate::handle_msg::HandleMessage;
use crate::handle_msg::HandleMessageWithEstablishment;
use crate::handle_msg::HandleMessageWithEstablishmentAndTimeout;
use crate::handle_msg::HandleMessageWithTimeout;
use crate::janus_interface::ConnectionParams;
use crate::janus_interface::JanusInterface;
use crate::japrotocol::EstablishmentProtocol;
use crate::japrotocol::JaResponse;
use crate::japrotocol::JaSuccessProtocol;
use crate::japrotocol::ResponseType;
use crate::prelude::JaTransportResult;
use crate::respones::ServerInfoRsp;
use crate::tgenerator::GenerateTransaction;
use crate::tgenerator::TransactionGenerator;
use jarust_rt::JaTask;
use serde_json::json;
use serde_json::Value;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::Mutex;

#[derive(Debug)]
struct Shared {
    apisecret: Option<String>,
    transaction_generator: TransactionGenerator,
    client: reqwest::Client,
    url: String,
}

#[derive(Debug)]
struct Exclusive {
    tasks: Vec<JaTask>,
}

#[derive(Debug)]
struct InnerResultfulInterface {
    shared: Shared,
    exclusive: Mutex<Exclusive>,
}

#[derive(Debug, Clone)]
pub struct RestfulInterface {
    inner: Arc<InnerResultfulInterface>,
}

impl RestfulInterface {
    fn decorate_request(&self, mut request: Value) -> (Value, String) {
        let transaction = self
            .inner
            .shared
            .transaction_generator
            .generate_transaction();
        if let Some(apisecret) = self.inner.shared.apisecret.clone() {
            request["apisecret"] = apisecret.into();
        };
        request["transaction"] = transaction.clone().into();
        (request, transaction)
    }
}

#[async_trait::async_trait]
impl JanusInterface for RestfulInterface {
    async fn make_interface(
        conn_params: ConnectionParams,
        transaction_generator: impl GenerateTransaction,
    ) -> JaTransportResult<Self> {
        let client = reqwest::Client::new();
        let transaction_generator = TransactionGenerator::new(transaction_generator);
        let shared = Shared {
            apisecret: conn_params.apisecret,
            transaction_generator,
            client,
            url: format!("{}/{}", conn_params.url, conn_params.namespace),
        };
        let exclusive = Exclusive { tasks: Vec::new() };
        let inner = InnerResultfulInterface {
            shared,
            exclusive: Mutex::new(exclusive),
        };
        Ok(Self {
            inner: Arc::new(inner),
        })
    }

    async fn create(&self, timeout: Duration) -> JaTransportResult<u64> {
        let url = &self.inner.shared.url;
        let request = json!({"janus": "create"});
        let (request, _) = self.decorate_request(request);

        let response = self
            .inner
            .shared
            .client
            .post(url.to_string())
            .json(&request)
            .timeout(timeout)
            .send()
            .await?
            .json::<JaResponse>()
            .await?;

        let session_id = match response.janus {
            ResponseType::Success(JaSuccessProtocol::Data { data }) => data.id,
            ResponseType::Error { error } => {
                let what = JaTransportError::JanusError {
                    code: error.code,
                    reason: error.reason,
                };
                tracing::error!("{what}");
                return Err(what);
            }
            _ => {
                tracing::error!("Unexpected response");
                return Err(JaTransportError::UnexpectedResponse);
            }
        };
        Ok(session_id)
    }

    async fn server_info(&self, timeout: Duration) -> JaTransportResult<ServerInfoRsp> {
        let url = &self.inner.shared.url;
        let response = self
            .inner
            .shared
            .client
            .get(format!("{url}/info"))
            .timeout(timeout)
            .send()
            .await?
            .json::<JaResponse>()
            .await?;
        match response.janus {
            ResponseType::ServerInfo(info) => Ok(*info),
            ResponseType::Error { error } => Err(JaTransportError::JanusError {
                code: error.code,
                reason: error.reason,
            }),
            _ => Err(JaTransportError::IncompletePacket),
        }
    }

    async fn attach(
        &self,
        session_id: u64,
        plugin_id: String,
        timeout: Duration,
    ) -> JaTransportResult<(u64, mpsc::UnboundedReceiver<JaResponse>)> {
        let url = &self.inner.shared.url;
        let request = json!({
            "janus": "attach",
            "plugin": plugin_id
        });
        let (request, _) = self.decorate_request(request);

        let response = self
            .inner
            .shared
            .client
            .post(format!("{url}/{session_id}"))
            .json(&request)
            .timeout(timeout)
            .send()
            .await?
            .json::<JaResponse>()
            .await?;
        let handle_id = match response.janus {
            ResponseType::Success(JaSuccessProtocol::Data { data }) => data.id,
            ResponseType::Error { error } => {
                let what = JaTransportError::JanusError {
                    code: error.code,
                    reason: error.reason,
                };
                tracing::error!("{what}");
                return Err(what);
            }
            _ => {
                tracing::error!("Unexpected response");
                return Err(JaTransportError::UnexpectedResponse);
            }
        };
        let (tx, rx) = mpsc::unbounded_channel();

        let handle = jarust_rt::spawn({
            let client = self.inner.shared.client.clone();
            let url = url.clone();

            async move {
                loop {
                    if let Ok(response) = client
                        .get(format!("{url}/{session_id}?maxev=5"))
                        .send()
                        .await
                    {
                        if let Ok(res) = response.json::<Vec<JaResponse>>().await {
                            for r in res {
                                let _ = tx.send(r);
                            }
                        }
                    };
                }
            }
        });

        self.inner.exclusive.lock().await.tasks.push(handle);

        Ok((handle_id, rx))
    }

    async fn keep_alive(&self, _: u64, _: Duration) -> JaTransportResult<()> {
        Ok(())
    }

    async fn destory(&self, session_id: u64, timeout: Duration) -> JaTransportResult<()> {
        let url = &self.inner.shared.url;
        let request = json!({
            "janus": "destroy"
        });
        let (request, _) = self.decorate_request(request);

        self.inner
            .shared
            .client
            .post(format!("{url}/{session_id}"))
            .json(&request)
            .timeout(timeout)
            .send()
            .await?;
        Ok(())
    }

    async fn fire_and_forget_msg(&self, message: HandleMessage) -> JaTransportResult<()> {
        let url = &self.inner.shared.url;
        let session_id = message.session_id;
        let handle_id = message.handle_id;

        let request = json!({
            "janus": "message",
            "body": message.body
        });
        let (request, _) = self.decorate_request(request);
        self.inner
            .shared
            .client
            .post(format!("{url}/{session_id}/{handle_id}"))
            .json(&request)
            .send()
            .await?;
        Ok(())
    }

    async fn send_msg_waiton_ack(
        &self,
        message: HandleMessageWithTimeout,
    ) -> JaTransportResult<JaResponse> {
        let url = &self.inner.shared.url;
        let session_id = message.session_id;
        let handle_id = message.handle_id;

        let request = json!({
            "janus": "message",
            "body": message.body
        });
        let (request, _) = self.decorate_request(request);
        let response = self
            .inner
            .shared
            .client
            .post(format!("{url}/{session_id}/{handle_id}"))
            .json(&request)
            .timeout(message.timeout)
            .send()
            .await?
            .json::<JaResponse>()
            .await?;
        Ok(response)
    }

    async fn internal_send_msg_waiton_rsp(
        &self,
        message: HandleMessageWithTimeout,
    ) -> JaTransportResult<JaResponse> {
        let url = &self.inner.shared.url;
        let session_id = message.session_id;
        let handle_id = message.handle_id;

        let request = json!({
            "janus": "message",
            "body": message.body
        });
        let (request, _) = self.decorate_request(request);
        let response = self
            .inner
            .shared
            .client
            .post(format!("{url}/{session_id}/{handle_id}"))
            .json(&request)
            .timeout(message.timeout)
            .send()
            .await?
            .json::<JaResponse>()
            .await?;
        Ok(response)
    }

    async fn fire_and_forget_msg_with_est(
        &self,
        message: HandleMessageWithEstablishment,
    ) -> JaTransportResult<()> {
        let url = &self.inner.shared.url;
        let session_id = message.session_id;
        let handle_id = message.handle_id;

        let mut request = json!({
            "janus": "message",
            "body": message.body,
        });
        match message.protocol {
            EstablishmentProtocol::JSEP(jsep) => {
                request["jsep"] = serde_json::to_value(jsep)?;
            }
            EstablishmentProtocol::RTP(rtp) => {
                request["rtp"] = serde_json::to_value(rtp)?;
            }
        };
        let (request, _) = self.decorate_request(request);
        self.inner
            .shared
            .client
            .post(format!("{url}/{session_id}/{handle_id}"))
            .json(&request)
            .send()
            .await?;
        Ok(())
    }

    async fn send_msg_waiton_ack_with_est(
        &self,
        message: HandleMessageWithEstablishmentAndTimeout,
    ) -> JaTransportResult<JaResponse> {
        let url = &self.inner.shared.url;
        let session_id = message.session_id;
        let handle_id = message.handle_id;

        let mut request = json!({
            "janus": "message",
            "body": message.body,
        });
        match message.protocol {
            EstablishmentProtocol::JSEP(jsep) => {
                request["jsep"] = serde_json::to_value(jsep)?;
            }
            EstablishmentProtocol::RTP(rtp) => {
                request["rtp"] = serde_json::to_value(rtp)?;
            }
        };
        let (request, _) = self.decorate_request(request);
        let response = self
            .inner
            .shared
            .client
            .post(format!("{url}/{session_id}/{handle_id}"))
            .json(&request)
            .send()
            .await?
            .json::<JaResponse>()
            .await?;
        Ok(response)
    }

    fn name(&self) -> Box<str> {
        "Restful Interface".to_string().into_boxed_str()
    }
}

impl Drop for Exclusive {
    fn drop(&mut self) {
        for task in self.tasks.drain(..) {
            task.cancel();
        }
    }
}