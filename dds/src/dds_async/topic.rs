use super::{
    condition::StatusConditionAsync, domain_participant::DomainParticipantAsync,
    topic_listener::TopicListener,
};
use crate::{
    dcps::{
        actor::ActorAddress,
        channels::oneshot::oneshot,
        domain_participant_mail::{DcpsDomainParticipantMail, TopicServiceMail},
        status_condition::DcpsStatusCondition,
    },
    infrastructure::{
        error::DdsResult,
        instance::InstanceHandle,
        qos::{QosKind, TopicQos},
        status::{InconsistentTopicStatus, StatusKind},
    },
    xtypes::dynamic_type::DynamicType,
};
use alloc::{string::String, vec::Vec};
use crate::sync::Arc;

/// Async version of [`Topic`](crate::topic_definition::topic::Topic).
pub struct TopicAsync {
    handle: InstanceHandle,
    status_condition_address: ActorAddress<DcpsStatusCondition>,
    type_name: String,
    topic_name: String,
    participant: DomainParticipantAsync,
}

impl Clone for TopicAsync {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle,
            status_condition_address: self.status_condition_address.clone(),
            type_name: self.type_name.clone(),
            topic_name: self.topic_name.clone(),
            participant: self.participant.clone(),
        }
    }
}

impl TopicAsync {
    pub(crate) fn new(
        handle: InstanceHandle,
        status_condition_address: ActorAddress<DcpsStatusCondition>,
        type_name: String,
        topic_name: String,
        participant: DomainParticipantAsync,
    ) -> Self {
        Self {
            handle,
            status_condition_address,
            type_name,
            topic_name,
            participant,
        }
    }
}

impl TopicAsync {
    /// Async version of [`get_inconsistent_topic_status`](crate::topic_definition::topic::Topic::get_inconsistent_topic_status).
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn get_inconsistent_topic_status(&self) -> DdsResult<InconsistentTopicStatus> {
        let (reply_sender, reply_receiver) = oneshot();
        self.participant
            .participant_address()
            .send(DcpsDomainParticipantMail::Topic(
                TopicServiceMail::GetInconsistentTopicStatus {
                    topic_name: self.topic_name.clone(),
                    reply_sender,
                },
            ))
            .await?;
        reply_receiver.await?
    }
}

impl TopicAsync {
    /// Async version of [`get_participant`](crate::topic_definition::topic::Topic::get_participant).
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub fn get_participant(&self) -> DomainParticipantAsync {
        self.participant.clone()
    }

    /// Async version of [`get_type_name`](crate::topic_definition::topic::Topic::get_type_name).
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub fn get_type_name(&self) -> String {
        self.type_name.clone()
    }

    /// Async version of [`get_name`](crate::topic_definition::topic::Topic::get_name).
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub fn get_name(&self) -> String {
        self.topic_name.clone()
    }
}

impl TopicAsync {
    /// Async version of [`set_qos`](crate::topic_definition::topic::Topic::set_qos).
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn set_qos(&self, qos: QosKind<TopicQos>) -> DdsResult<()> {
        let (reply_sender, reply_receiver) = oneshot();
        self.participant
            .participant_address()
            .send(DcpsDomainParticipantMail::Topic(TopicServiceMail::SetQos {
                topic_name: self.topic_name.clone(),
                topic_qos: qos,
                reply_sender,
            }))
            .await?;

        reply_receiver.await?
    }

    /// Async version of [`get_qos`](crate::topic_definition::topic::Topic::get_qos).
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn get_qos(&self) -> DdsResult<TopicQos> {
        let (reply_sender, reply_receiver) = oneshot();
        self.participant
            .participant_address()
            .send(DcpsDomainParticipantMail::Topic(TopicServiceMail::GetQos {
                topic_name: self.topic_name.clone(),
                reply_sender,
            }))
            .await?;

        reply_receiver.await?
    }

    /// Async version of [`get_statuscondition`](crate::topic_definition::topic::Topic::get_statuscondition).
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub fn get_statuscondition(&self) -> StatusConditionAsync {
        StatusConditionAsync::new(self.status_condition_address.clone())
    }

    /// Async version of [`get_status_changes`](crate::topic_definition::topic::Topic::get_status_changes).
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn get_status_changes(&self) -> DdsResult<Vec<StatusKind>> {
        todo!()
    }

    /// Async version of [`enable`](crate::topic_definition::topic::Topic::enable).
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn enable(&self) -> DdsResult<()> {
        let (reply_sender, reply_receiver) = oneshot();
        self.participant
            .participant_address()
            .send(DcpsDomainParticipantMail::Topic(TopicServiceMail::Enable {
                topic_name: self.topic_name.clone(),
                participant_address: self.participant.participant_address().clone(),
                reply_sender,
            }))
            .await?;
        reply_receiver.await?
    }

    /// Async version of [`get_instance_handle`](crate::topic_definition::topic::Topic::get_instance_handle).
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn get_instance_handle(&self) -> InstanceHandle {
        self.handle
    }

    /// Async version of [`set_listener`](crate::topic_definition::topic::Topic::set_listener).
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self, _a_listener)))]
    pub async fn set_listener(
        &self,
        _a_listener: Option<impl TopicListener + Send + 'static>,
        _mask: &[StatusKind],
    ) -> DdsResult<()> {
        todo!()
    }
}

impl TopicAsync {
    #[doc(hidden)]
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn get_type_support(&self) -> DdsResult<Arc<DynamicType>> {
        let (reply_sender, reply_receiver) = oneshot();
        self.participant
            .participant_address()
            .send(DcpsDomainParticipantMail::Topic(
                TopicServiceMail::GetTypeSupport {
                    topic_name: self.topic_name.clone(),
                    reply_sender,
                },
            ))
            .await?;

        reply_receiver.await?
    }
}
