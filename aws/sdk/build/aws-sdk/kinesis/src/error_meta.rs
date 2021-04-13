// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    ExpiredIteratorError(crate::error::ExpiredIteratorError),
    ExpiredNextTokenError(crate::error::ExpiredNextTokenError),
    InvalidArgumentError(crate::error::InvalidArgumentError),
    KMSAccessDeniedError(crate::error::KMSAccessDeniedError),
    KMSDisabledError(crate::error::KMSDisabledError),
    KMSInvalidStateError(crate::error::KMSInvalidStateError),
    KMSNotFoundError(crate::error::KMSNotFoundError),
    KMSOptInRequired(crate::error::KMSOptInRequired),
    KMSThrottlingError(crate::error::KMSThrottlingError),
    LimitExceededError(crate::error::LimitExceededError),
    ProvisionedThroughputExceededError(crate::error::ProvisionedThroughputExceededError),
    ResourceInUseError(crate::error::ResourceInUseError),
    ResourceNotFoundError(crate::error::ResourceNotFoundError),
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ExpiredIteratorError(inner) => inner.fmt(f),
            Error::ExpiredNextTokenError(inner) => inner.fmt(f),
            Error::InvalidArgumentError(inner) => inner.fmt(f),
            Error::KMSAccessDeniedError(inner) => inner.fmt(f),
            Error::KMSDisabledError(inner) => inner.fmt(f),
            Error::KMSInvalidStateError(inner) => inner.fmt(f),
            Error::KMSNotFoundError(inner) => inner.fmt(f),
            Error::KMSOptInRequired(inner) => inner.fmt(f),
            Error::KMSThrottlingError(inner) => inner.fmt(f),
            Error::LimitExceededError(inner) => inner.fmt(f),
            Error::ProvisionedThroughputExceededError(inner) => inner.fmt(f),
            Error::ResourceInUseError(inner) => inner.fmt(f),
            Error::ResourceNotFoundError(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::AddTagsToStreamError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::AddTagsToStreamError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::AddTagsToStreamErrorKind::InvalidArgumentError(inner) => {
                    Error::InvalidArgumentError(inner)
                }
                crate::error::AddTagsToStreamErrorKind::LimitExceededError(inner) => {
                    Error::LimitExceededError(inner)
                }
                crate::error::AddTagsToStreamErrorKind::ResourceInUseError(inner) => {
                    Error::ResourceInUseError(inner)
                }
                crate::error::AddTagsToStreamErrorKind::ResourceNotFoundError(inner) => {
                    Error::ResourceNotFoundError(inner)
                }
                crate::error::AddTagsToStreamErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::CreateStreamError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::CreateStreamError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateStreamErrorKind::InvalidArgumentError(inner) => {
                    Error::InvalidArgumentError(inner)
                }
                crate::error::CreateStreamErrorKind::LimitExceededError(inner) => {
                    Error::LimitExceededError(inner)
                }
                crate::error::CreateStreamErrorKind::ResourceInUseError(inner) => {
                    Error::ResourceInUseError(inner)
                }
                crate::error::CreateStreamErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DecreaseStreamRetentionPeriodError>>
    for Error
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::DecreaseStreamRetentionPeriodError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DecreaseStreamRetentionPeriodErrorKind::InvalidArgumentError(
                    inner,
                ) => Error::InvalidArgumentError(inner),
                crate::error::DecreaseStreamRetentionPeriodErrorKind::LimitExceededError(inner) => {
                    Error::LimitExceededError(inner)
                }
                crate::error::DecreaseStreamRetentionPeriodErrorKind::ResourceInUseError(inner) => {
                    Error::ResourceInUseError(inner)
                }
                crate::error::DecreaseStreamRetentionPeriodErrorKind::ResourceNotFoundError(
                    inner,
                ) => Error::ResourceNotFoundError(inner),
                crate::error::DecreaseStreamRetentionPeriodErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DeleteStreamError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteStreamError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteStreamErrorKind::LimitExceededError(inner) => {
                    Error::LimitExceededError(inner)
                }
                crate::error::DeleteStreamErrorKind::ResourceInUseError(inner) => {
                    Error::ResourceInUseError(inner)
                }
                crate::error::DeleteStreamErrorKind::ResourceNotFoundError(inner) => {
                    Error::ResourceNotFoundError(inner)
                }
                crate::error::DeleteStreamErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DeregisterStreamConsumerError>> for Error {
    fn from(
        err: smithy_http::result::SdkError<crate::error::DeregisterStreamConsumerError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeregisterStreamConsumerErrorKind::InvalidArgumentError(inner) => {
                    Error::InvalidArgumentError(inner)
                }
                crate::error::DeregisterStreamConsumerErrorKind::LimitExceededError(inner) => {
                    Error::LimitExceededError(inner)
                }
                crate::error::DeregisterStreamConsumerErrorKind::ResourceNotFoundError(inner) => {
                    Error::ResourceNotFoundError(inner)
                }
                crate::error::DeregisterStreamConsumerErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeLimitsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DescribeLimitsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeLimitsErrorKind::LimitExceededError(inner) => {
                    Error::LimitExceededError(inner)
                }
                crate::error::DescribeLimitsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeStreamError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DescribeStreamError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeStreamErrorKind::LimitExceededError(inner) => {
                    Error::LimitExceededError(inner)
                }
                crate::error::DescribeStreamErrorKind::ResourceNotFoundError(inner) => {
                    Error::ResourceNotFoundError(inner)
                }
                crate::error::DescribeStreamErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeStreamConsumerError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DescribeStreamConsumerError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeStreamConsumerErrorKind::InvalidArgumentError(inner) => {
                    Error::InvalidArgumentError(inner)
                }
                crate::error::DescribeStreamConsumerErrorKind::LimitExceededError(inner) => {
                    Error::LimitExceededError(inner)
                }
                crate::error::DescribeStreamConsumerErrorKind::ResourceNotFoundError(inner) => {
                    Error::ResourceNotFoundError(inner)
                }
                crate::error::DescribeStreamConsumerErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DescribeStreamSummaryError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DescribeStreamSummaryError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeStreamSummaryErrorKind::LimitExceededError(inner) => {
                    Error::LimitExceededError(inner)
                }
                crate::error::DescribeStreamSummaryErrorKind::ResourceNotFoundError(inner) => {
                    Error::ResourceNotFoundError(inner)
                }
                crate::error::DescribeStreamSummaryErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DisableEnhancedMonitoringError>> for Error {
    fn from(
        err: smithy_http::result::SdkError<crate::error::DisableEnhancedMonitoringError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DisableEnhancedMonitoringErrorKind::InvalidArgumentError(inner) => {
                    Error::InvalidArgumentError(inner)
                }
                crate::error::DisableEnhancedMonitoringErrorKind::LimitExceededError(inner) => {
                    Error::LimitExceededError(inner)
                }
                crate::error::DisableEnhancedMonitoringErrorKind::ResourceInUseError(inner) => {
                    Error::ResourceInUseError(inner)
                }
                crate::error::DisableEnhancedMonitoringErrorKind::ResourceNotFoundError(inner) => {
                    Error::ResourceNotFoundError(inner)
                }
                crate::error::DisableEnhancedMonitoringErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::EnableEnhancedMonitoringError>> for Error {
    fn from(
        err: smithy_http::result::SdkError<crate::error::EnableEnhancedMonitoringError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::EnableEnhancedMonitoringErrorKind::InvalidArgumentError(inner) => {
                    Error::InvalidArgumentError(inner)
                }
                crate::error::EnableEnhancedMonitoringErrorKind::LimitExceededError(inner) => {
                    Error::LimitExceededError(inner)
                }
                crate::error::EnableEnhancedMonitoringErrorKind::ResourceInUseError(inner) => {
                    Error::ResourceInUseError(inner)
                }
                crate::error::EnableEnhancedMonitoringErrorKind::ResourceNotFoundError(inner) => {
                    Error::ResourceNotFoundError(inner)
                }
                crate::error::EnableEnhancedMonitoringErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::GetRecordsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::GetRecordsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetRecordsErrorKind::ExpiredIteratorError(inner) => {
                    Error::ExpiredIteratorError(inner)
                }
                crate::error::GetRecordsErrorKind::InvalidArgumentError(inner) => {
                    Error::InvalidArgumentError(inner)
                }
                crate::error::GetRecordsErrorKind::KMSAccessDeniedError(inner) => {
                    Error::KMSAccessDeniedError(inner)
                }
                crate::error::GetRecordsErrorKind::KMSDisabledError(inner) => {
                    Error::KMSDisabledError(inner)
                }
                crate::error::GetRecordsErrorKind::KMSInvalidStateError(inner) => {
                    Error::KMSInvalidStateError(inner)
                }
                crate::error::GetRecordsErrorKind::KMSNotFoundError(inner) => {
                    Error::KMSNotFoundError(inner)
                }
                crate::error::GetRecordsErrorKind::KMSOptInRequired(inner) => {
                    Error::KMSOptInRequired(inner)
                }
                crate::error::GetRecordsErrorKind::KMSThrottlingError(inner) => {
                    Error::KMSThrottlingError(inner)
                }
                crate::error::GetRecordsErrorKind::ProvisionedThroughputExceededError(inner) => {
                    Error::ProvisionedThroughputExceededError(inner)
                }
                crate::error::GetRecordsErrorKind::ResourceNotFoundError(inner) => {
                    Error::ResourceNotFoundError(inner)
                }
                crate::error::GetRecordsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::GetShardIteratorError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::GetShardIteratorError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetShardIteratorErrorKind::InvalidArgumentError(inner) => {
                    Error::InvalidArgumentError(inner)
                }
                crate::error::GetShardIteratorErrorKind::ProvisionedThroughputExceededError(
                    inner,
                ) => Error::ProvisionedThroughputExceededError(inner),
                crate::error::GetShardIteratorErrorKind::ResourceNotFoundError(inner) => {
                    Error::ResourceNotFoundError(inner)
                }
                crate::error::GetShardIteratorErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::IncreaseStreamRetentionPeriodError>>
    for Error
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::IncreaseStreamRetentionPeriodError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::IncreaseStreamRetentionPeriodErrorKind::InvalidArgumentError(
                    inner,
                ) => Error::InvalidArgumentError(inner),
                crate::error::IncreaseStreamRetentionPeriodErrorKind::LimitExceededError(inner) => {
                    Error::LimitExceededError(inner)
                }
                crate::error::IncreaseStreamRetentionPeriodErrorKind::ResourceInUseError(inner) => {
                    Error::ResourceInUseError(inner)
                }
                crate::error::IncreaseStreamRetentionPeriodErrorKind::ResourceNotFoundError(
                    inner,
                ) => Error::ResourceNotFoundError(inner),
                crate::error::IncreaseStreamRetentionPeriodErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::ListShardsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::ListShardsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListShardsErrorKind::ExpiredNextTokenError(inner) => {
                    Error::ExpiredNextTokenError(inner)
                }
                crate::error::ListShardsErrorKind::InvalidArgumentError(inner) => {
                    Error::InvalidArgumentError(inner)
                }
                crate::error::ListShardsErrorKind::LimitExceededError(inner) => {
                    Error::LimitExceededError(inner)
                }
                crate::error::ListShardsErrorKind::ResourceInUseError(inner) => {
                    Error::ResourceInUseError(inner)
                }
                crate::error::ListShardsErrorKind::ResourceNotFoundError(inner) => {
                    Error::ResourceNotFoundError(inner)
                }
                crate::error::ListShardsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::ListStreamConsumersError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::ListStreamConsumersError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListStreamConsumersErrorKind::ExpiredNextTokenError(inner) => {
                    Error::ExpiredNextTokenError(inner)
                }
                crate::error::ListStreamConsumersErrorKind::InvalidArgumentError(inner) => {
                    Error::InvalidArgumentError(inner)
                }
                crate::error::ListStreamConsumersErrorKind::LimitExceededError(inner) => {
                    Error::LimitExceededError(inner)
                }
                crate::error::ListStreamConsumersErrorKind::ResourceInUseError(inner) => {
                    Error::ResourceInUseError(inner)
                }
                crate::error::ListStreamConsumersErrorKind::ResourceNotFoundError(inner) => {
                    Error::ResourceNotFoundError(inner)
                }
                crate::error::ListStreamConsumersErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::ListStreamsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::ListStreamsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListStreamsErrorKind::LimitExceededError(inner) => {
                    Error::LimitExceededError(inner)
                }
                crate::error::ListStreamsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::ListTagsForStreamError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::ListTagsForStreamError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListTagsForStreamErrorKind::InvalidArgumentError(inner) => {
                    Error::InvalidArgumentError(inner)
                }
                crate::error::ListTagsForStreamErrorKind::LimitExceededError(inner) => {
                    Error::LimitExceededError(inner)
                }
                crate::error::ListTagsForStreamErrorKind::ResourceNotFoundError(inner) => {
                    Error::ResourceNotFoundError(inner)
                }
                crate::error::ListTagsForStreamErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::MergeShardsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::MergeShardsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::MergeShardsErrorKind::InvalidArgumentError(inner) => {
                    Error::InvalidArgumentError(inner)
                }
                crate::error::MergeShardsErrorKind::LimitExceededError(inner) => {
                    Error::LimitExceededError(inner)
                }
                crate::error::MergeShardsErrorKind::ResourceInUseError(inner) => {
                    Error::ResourceInUseError(inner)
                }
                crate::error::MergeShardsErrorKind::ResourceNotFoundError(inner) => {
                    Error::ResourceNotFoundError(inner)
                }
                crate::error::MergeShardsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::PutRecordError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::PutRecordError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::PutRecordErrorKind::InvalidArgumentError(inner) => {
                    Error::InvalidArgumentError(inner)
                }
                crate::error::PutRecordErrorKind::KMSAccessDeniedError(inner) => {
                    Error::KMSAccessDeniedError(inner)
                }
                crate::error::PutRecordErrorKind::KMSDisabledError(inner) => {
                    Error::KMSDisabledError(inner)
                }
                crate::error::PutRecordErrorKind::KMSInvalidStateError(inner) => {
                    Error::KMSInvalidStateError(inner)
                }
                crate::error::PutRecordErrorKind::KMSNotFoundError(inner) => {
                    Error::KMSNotFoundError(inner)
                }
                crate::error::PutRecordErrorKind::KMSOptInRequired(inner) => {
                    Error::KMSOptInRequired(inner)
                }
                crate::error::PutRecordErrorKind::KMSThrottlingError(inner) => {
                    Error::KMSThrottlingError(inner)
                }
                crate::error::PutRecordErrorKind::ProvisionedThroughputExceededError(inner) => {
                    Error::ProvisionedThroughputExceededError(inner)
                }
                crate::error::PutRecordErrorKind::ResourceNotFoundError(inner) => {
                    Error::ResourceNotFoundError(inner)
                }
                crate::error::PutRecordErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::PutRecordsError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::PutRecordsError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::PutRecordsErrorKind::InvalidArgumentError(inner) => {
                    Error::InvalidArgumentError(inner)
                }
                crate::error::PutRecordsErrorKind::KMSAccessDeniedError(inner) => {
                    Error::KMSAccessDeniedError(inner)
                }
                crate::error::PutRecordsErrorKind::KMSDisabledError(inner) => {
                    Error::KMSDisabledError(inner)
                }
                crate::error::PutRecordsErrorKind::KMSInvalidStateError(inner) => {
                    Error::KMSInvalidStateError(inner)
                }
                crate::error::PutRecordsErrorKind::KMSNotFoundError(inner) => {
                    Error::KMSNotFoundError(inner)
                }
                crate::error::PutRecordsErrorKind::KMSOptInRequired(inner) => {
                    Error::KMSOptInRequired(inner)
                }
                crate::error::PutRecordsErrorKind::KMSThrottlingError(inner) => {
                    Error::KMSThrottlingError(inner)
                }
                crate::error::PutRecordsErrorKind::ProvisionedThroughputExceededError(inner) => {
                    Error::ProvisionedThroughputExceededError(inner)
                }
                crate::error::PutRecordsErrorKind::ResourceNotFoundError(inner) => {
                    Error::ResourceNotFoundError(inner)
                }
                crate::error::PutRecordsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::RegisterStreamConsumerError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::RegisterStreamConsumerError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::RegisterStreamConsumerErrorKind::InvalidArgumentError(inner) => {
                    Error::InvalidArgumentError(inner)
                }
                crate::error::RegisterStreamConsumerErrorKind::LimitExceededError(inner) => {
                    Error::LimitExceededError(inner)
                }
                crate::error::RegisterStreamConsumerErrorKind::ResourceInUseError(inner) => {
                    Error::ResourceInUseError(inner)
                }
                crate::error::RegisterStreamConsumerErrorKind::ResourceNotFoundError(inner) => {
                    Error::ResourceNotFoundError(inner)
                }
                crate::error::RegisterStreamConsumerErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::RemoveTagsFromStreamError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::RemoveTagsFromStreamError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::RemoveTagsFromStreamErrorKind::InvalidArgumentError(inner) => {
                    Error::InvalidArgumentError(inner)
                }
                crate::error::RemoveTagsFromStreamErrorKind::LimitExceededError(inner) => {
                    Error::LimitExceededError(inner)
                }
                crate::error::RemoveTagsFromStreamErrorKind::ResourceInUseError(inner) => {
                    Error::ResourceInUseError(inner)
                }
                crate::error::RemoveTagsFromStreamErrorKind::ResourceNotFoundError(inner) => {
                    Error::ResourceNotFoundError(inner)
                }
                crate::error::RemoveTagsFromStreamErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::SplitShardError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::SplitShardError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::SplitShardErrorKind::InvalidArgumentError(inner) => {
                    Error::InvalidArgumentError(inner)
                }
                crate::error::SplitShardErrorKind::LimitExceededError(inner) => {
                    Error::LimitExceededError(inner)
                }
                crate::error::SplitShardErrorKind::ResourceInUseError(inner) => {
                    Error::ResourceInUseError(inner)
                }
                crate::error::SplitShardErrorKind::ResourceNotFoundError(inner) => {
                    Error::ResourceNotFoundError(inner)
                }
                crate::error::SplitShardErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::StartStreamEncryptionError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::StartStreamEncryptionError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::StartStreamEncryptionErrorKind::InvalidArgumentError(inner) => {
                    Error::InvalidArgumentError(inner)
                }
                crate::error::StartStreamEncryptionErrorKind::KMSAccessDeniedError(inner) => {
                    Error::KMSAccessDeniedError(inner)
                }
                crate::error::StartStreamEncryptionErrorKind::KMSDisabledError(inner) => {
                    Error::KMSDisabledError(inner)
                }
                crate::error::StartStreamEncryptionErrorKind::KMSInvalidStateError(inner) => {
                    Error::KMSInvalidStateError(inner)
                }
                crate::error::StartStreamEncryptionErrorKind::KMSNotFoundError(inner) => {
                    Error::KMSNotFoundError(inner)
                }
                crate::error::StartStreamEncryptionErrorKind::KMSOptInRequired(inner) => {
                    Error::KMSOptInRequired(inner)
                }
                crate::error::StartStreamEncryptionErrorKind::KMSThrottlingError(inner) => {
                    Error::KMSThrottlingError(inner)
                }
                crate::error::StartStreamEncryptionErrorKind::LimitExceededError(inner) => {
                    Error::LimitExceededError(inner)
                }
                crate::error::StartStreamEncryptionErrorKind::ResourceInUseError(inner) => {
                    Error::ResourceInUseError(inner)
                }
                crate::error::StartStreamEncryptionErrorKind::ResourceNotFoundError(inner) => {
                    Error::ResourceNotFoundError(inner)
                }
                crate::error::StartStreamEncryptionErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::StopStreamEncryptionError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::StopStreamEncryptionError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::StopStreamEncryptionErrorKind::InvalidArgumentError(inner) => {
                    Error::InvalidArgumentError(inner)
                }
                crate::error::StopStreamEncryptionErrorKind::LimitExceededError(inner) => {
                    Error::LimitExceededError(inner)
                }
                crate::error::StopStreamEncryptionErrorKind::ResourceInUseError(inner) => {
                    Error::ResourceInUseError(inner)
                }
                crate::error::StopStreamEncryptionErrorKind::ResourceNotFoundError(inner) => {
                    Error::ResourceNotFoundError(inner)
                }
                crate::error::StopStreamEncryptionErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::SubscribeToShardError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::SubscribeToShardError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::SubscribeToShardErrorKind::InvalidArgumentError(inner) => {
                    Error::InvalidArgumentError(inner)
                }
                crate::error::SubscribeToShardErrorKind::LimitExceededError(inner) => {
                    Error::LimitExceededError(inner)
                }
                crate::error::SubscribeToShardErrorKind::ResourceInUseError(inner) => {
                    Error::ResourceInUseError(inner)
                }
                crate::error::SubscribeToShardErrorKind::ResourceNotFoundError(inner) => {
                    Error::ResourceNotFoundError(inner)
                }
                crate::error::SubscribeToShardErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::UpdateShardCountError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::UpdateShardCountError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateShardCountErrorKind::InvalidArgumentError(inner) => {
                    Error::InvalidArgumentError(inner)
                }
                crate::error::UpdateShardCountErrorKind::LimitExceededError(inner) => {
                    Error::LimitExceededError(inner)
                }
                crate::error::UpdateShardCountErrorKind::ResourceInUseError(inner) => {
                    Error::ResourceInUseError(inner)
                }
                crate::error::UpdateShardCountErrorKind::ResourceNotFoundError(inner) => {
                    Error::ResourceNotFoundError(inner)
                }
                crate::error::UpdateShardCountErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}
