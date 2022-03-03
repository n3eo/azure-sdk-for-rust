use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::resources::Attachment;
use crate::ResourceQuota;

use azure_core::headers::{etag_from_headers, session_token_from_headers};
use azure_core::prelude::*;
use azure_core::SessionToken;
use azure_core::{collect_pinned_stream, Response as HttpResponse};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct CreateReferenceAttachmentBuilder {
    client: AttachmentClient,
    media: String,
    content_type: String,
    consistency_level: Option<ConsistencyLevel>,
    context: Context,
}

impl CreateReferenceAttachmentBuilder {
    pub(crate) fn new(client: AttachmentClient, media: String, content_type: String) -> Self {
        Self {
            client,
            media,
            content_type,
            consistency_level: None,
            context: Context::new(),
        }
    }
    setters! {
        consistency_level: ConsistencyLevel => Some(consistency_level),
        context: Context => context,
    }

    pub fn into_future(self) -> CreateReferenceAttachment {
        Box::pin(async move {
            let mut req = self.client.prepare_pipeline(http::Method::POST);

            azure_core::headers::add_optional_header2(&self.consistency_level, &mut req)?;
            crate::cosmos_entity::add_as_partition_key_header_serialized2(
                self.client.document_client().partition_key_serialized(),
                &mut req,
            );

            #[derive(Debug, Serialize)]
            struct Request<'r> {
                pub id: &'r str,
                #[serde(rename = "contentType")]
                pub content_type: &'r str,
                pub media: &'r str,
            }

            let body = azure_core::to_json(&Request {
                id: self.client.attachment_name(),
                content_type: &self.content_type,
                media: &self.media,
            })?;

            req.set_body(body.into());

            let response = self
                .client
                .pipeline()
                .send(
                    self.context.clone().insert(ResourceType::Attachments),
                    &mut req,
                )
                .await?;
            CreateReferenceAttachmentResponse::try_from(response).await
        })
    }
}

/// The future returned by calling `into_future` on the builder.
pub type CreateReferenceAttachment =
    futures::future::BoxFuture<'static, crate::Result<CreateReferenceAttachmentResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for CreateReferenceAttachmentBuilder {
    type Future = CreateReferenceAttachment;
    type Output = <CreateReferenceAttachment as std::future::Future>::Output;
    fn into_future(self) -> Self::Future {
        Self::into_future(self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateReferenceAttachmentResponse {
    pub attachment: Attachment,
    pub max_media_storage_usage_mb: u64,
    pub media_storage_usage_mb: u64,
    pub last_change: DateTime<Utc>,
    pub etag: String,
    pub resource_quota: Vec<ResourceQuota>,
    pub resource_usage: Vec<ResourceQuota>,
    pub lsn: u64,
    pub alt_content_path: String,
    pub content_path: String,
    pub quorum_acked_lsn: u64,
    pub current_write_quorum: u64,
    pub current_replica_set_size: u64,
    pub global_committed_lsn: u64,
    pub number_of_read_regions: u32,
    pub transport_request_id: u64,
    pub cosmos_llsn: u64,
    pub cosmos_quorum_acked_llsn: u64,
    pub session_token: SessionToken,
    pub request_charge: f64,
    pub service_version: String,
    pub activity_id: uuid::Uuid,
    pub gateway_version: String,
    pub date: DateTime<Utc>,
}

impl CreateReferenceAttachmentResponse {
    pub async fn try_from(response: HttpResponse) -> crate::Result<Self> {
        let (_status_code, headers, pinned_stream) = response.deconstruct();
        let body = collect_pinned_stream(pinned_stream).await?;

        let attachment: Attachment = serde_json::from_slice(&body)?;

        Ok(Self {
            attachment,
            max_media_storage_usage_mb: max_media_storage_usage_mb_from_headers(&headers)?,
            media_storage_usage_mb: media_storage_usage_mb_from_headers(&headers)?,
            last_change: last_state_change_from_headers(&headers)?,
            etag: etag_from_headers(&headers)?,
            resource_quota: resource_quota_from_headers(&headers)?,
            resource_usage: resource_usage_from_headers(&headers)?,
            lsn: lsn_from_headers(&headers)?,
            alt_content_path: alt_content_path_from_headers(&headers)?.to_owned(),
            content_path: content_path_from_headers(&headers)?.to_owned(),
            quorum_acked_lsn: quorum_acked_lsn_from_headers(&headers)?,
            current_write_quorum: current_write_quorum_from_headers(&headers)?,
            current_replica_set_size: current_replica_set_size_from_headers(&headers)?,
            global_committed_lsn: global_committed_lsn_from_headers(&headers)?,
            number_of_read_regions: number_of_read_regions_from_headers(&headers)?,
            transport_request_id: transport_request_id_from_headers(&headers)?,
            cosmos_llsn: cosmos_llsn_from_headers(&headers)?,
            cosmos_quorum_acked_llsn: cosmos_quorum_acked_llsn_from_headers(&headers)?,
            session_token: session_token_from_headers(&headers)?,
            request_charge: request_charge_from_headers(&headers)?,
            service_version: service_version_from_headers(&headers)?.to_owned(),
            activity_id: activity_id_from_headers(&headers)?,
            gateway_version: gateway_version_from_headers(&headers)?.to_owned(),
            date: date_from_headers(&headers)?,
        })
    }
}