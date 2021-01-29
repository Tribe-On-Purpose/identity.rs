// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use futures::stream::FuturesUnordered;
use futures::stream::StreamExt;
use identity_core::{common::Url, convert::ToJson};
use iota::{Message, MessageId};

use crate::{
    chain::{AuthChain, DiffChain, DocumentChain},
    client::{ClientBuilder, Network},
    did::{DocumentDiff, IotaDID, IotaDocument},
    error::{Error, Result},
};

#[derive(Clone, Debug)]
pub struct Messages {
    message_ids: Box<[MessageId]>,
    messages: Vec<Message>,
}

#[derive(Debug)]
pub struct Client {
    pub(crate) client: iota::Client,
    pub(crate) network: Network,
}

impl Client {
    /// Creates a new `Client`  with default settings.
    pub fn new() -> Result<Self> {
        Self::from_builder(Self::builder())
    }

    /// Creates a `ClientBuilder` to configure a new `Client`.
    ///
    /// This is the same as `ClientBuilder::new()`.
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    /// Creates a new `Client` with default settings for the given `Network`.
    pub fn from_network(network: Network) -> Result<Self> {
        Self::builder()
            .node(network.node_url().as_str())
            .network(network)
            .build()
    }

    /// Creates a new `Client` based on the `ClientBuilder` configuration.
    pub fn from_builder(builder: ClientBuilder) -> Result<Self> {
        let mut client: iota::ClientBuilder = iota::ClientBuilder::new();

        if builder.nodes.is_empty() {
            client = client.with_node(builder.network.node_url().as_str())?;
        } else {
            for node in builder.nodes {
                client = client.with_node(&node)?;
            }
        }

        client = client.with_network(builder.network.as_str());

        Ok(Self {
            client: client.finish()?,
            network: builder.network,
        })
    }

    /// Returns the `Client` Tangle network.
    pub fn network(&self) -> Network {
        self.network
    }

    /// Returns the default node URL of the `Client` network.
    pub fn default_node_url(&self) -> &'static Url {
        self.network.node_url()
    }

    /// Returns the web explorer URL of the `Client` network.
    pub fn explorer_url(&self) -> &'static Url {
        self.network.explorer_url()
    }

    /// Returns the web explorer URL of the given `transaction`.
    pub fn transaction_url(&self, message_id: &str) -> Url {
        format!("{}/message/{}", self.network.explorer_url(), message_id)
            .parse()
            .unwrap()
    }

    /// Publishes an `IotaDocument` to the Tangle.
    ///
    /// Note: The only validation performed is to ensure the correct Tangle
    /// network is selected.
    pub async fn publish_document(&self, document: &IotaDocument) -> Result<MessageId> {
        trace!("Publish Document: {}", document.id());

        self.check_network(document.id())?;

        let message: Message = self
            .client
            .send()
            .with_index(document.id().tag())
            .with_data(document.to_json()?.into_bytes())
            .finish()
            .await?;

        Ok(message.id().0)
    }

    /// Publishes a `DocumentDiff` to the Tangle.
    ///
    /// Note: The only validation performed is to ensure the correct Tangle
    /// network is selected.
    pub async fn publish_diff(&self, message_id: &MessageId, diff: &DocumentDiff) -> Result<MessageId> {
        trace!("Publish Diff: {}", diff.id());

        self.check_network(diff.id())?;

        let message: Message = self
            .client
            .send()
            .with_index(&IotaDocument::diff_address(message_id)?)
            .with_data(diff.to_json()?.into_bytes())
            .finish()
            .await?;

        Ok(message.id().0)
    }

    pub async fn read_document(&self, did: &IotaDID) -> Result<IotaDocument> {
        self.read_document_chain(did).await.and_then(DocumentChain::fold)
    }

    pub async fn read_document_chain(&self, did: &IotaDID) -> Result<DocumentChain> {
        trace!("Read Document Chain: {}", did);
        trace!("Auth Chain Address: {}", did.tag());

        // Fetch all messages for the auth chain.
        let messages: Messages = self.read_messages(did.tag()).await?;
        let auth: AuthChain = AuthChain::try_from_messages(did, &messages.messages)?;

        let diff: DiffChain = if auth.current().immutable() {
            DiffChain::new()
        } else {
            // Fetch all messages for the diff chain.
            let address: String = IotaDocument::diff_address(auth.current_message_id())?;
            let messages: Messages = self.read_messages(&address).await?;

            trace!("Tangle Messages: {:?}", messages);

            DiffChain::try_from_messages(&auth, &messages.messages)?
        };

        DocumentChain::with_diff_chain(auth, diff)
    }

    pub async fn read_messages(&self, address: &str) -> Result<Messages> {
        let message_ids: Box<[MessageId]> = self.client.get_message().index(address).await?;

        let messages: Vec<Message> = message_ids
            .iter()
            .map(|message| self.client.get_message().data(message))
            .collect::<FuturesUnordered<_>>()
            .filter_map(|message| async move { message.ok() })
            .collect()
            .await;

        Ok(Messages { message_ids, messages })
    }

    pub fn check_network(&self, did: &IotaDID) -> Result<()> {
        if !self.network.matches_did(did) {
            return Err(Error::InvalidDIDNetwork);
        }

        Ok(())
    }
}