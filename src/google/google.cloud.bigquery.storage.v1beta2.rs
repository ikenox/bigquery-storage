/// Arrow schema as specified in
/// https://arrow.apache.org/docs/python/api/datatypes.html
/// and serialized to bytes using IPC:
/// https://arrow.apache.org/docs/format/Columnar.html#serialization-and-interprocess-communication-ipc
///
/// See code samples on how this message can be deserialized.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArrowSchema {
    /// IPC serialized Arrow schema.
    #[prost(bytes = "vec", tag = "1")]
    pub serialized_schema: ::prost::alloc::vec::Vec<u8>,
}
/// Arrow RecordBatch.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArrowRecordBatch {
    /// IPC-serialized Arrow RecordBatch.
    #[prost(bytes = "vec", tag = "1")]
    pub serialized_record_batch: ::prost::alloc::vec::Vec<u8>,
}
/// Contains options specific to Arrow Serialization.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArrowSerializationOptions {
    /// The Arrow IPC format to use.
    #[prost(enumeration = "arrow_serialization_options::Format", tag = "1")]
    pub format: i32,
}
/// Nested message and enum types in `ArrowSerializationOptions`.
pub mod arrow_serialization_options {
    /// The IPC format to use when serializing Arrow streams.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Format {
        /// If unspecied the IPC format as of 0.15 release will be used.
        Unspecified = 0,
        /// Use the legacy IPC message format as of Apache Arrow Release 0.14.
        Arrow014 = 1,
        /// Use the message format as of Apache Arrow Release 0.15.
        Arrow015 = 2,
    }
}
/// Avro schema.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AvroSchema {
    /// Json serialized schema, as described at
    /// https://avro.apache.org/docs/1.8.1/spec.html.
    #[prost(string, tag = "1")]
    pub schema: ::prost::alloc::string::String,
}
/// Avro rows.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AvroRows {
    /// Binary serialized rows in a block.
    #[prost(bytes = "vec", tag = "1")]
    pub serialized_binary_rows: ::prost::alloc::vec::Vec<u8>,
}
/// Protobuf schema is an API presentation the proto buffer schema.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoSchema {
    /// Descriptor for input message. The descriptor has to be self contained,
    /// including all the nested types, excepted for proto buffer well known types
    /// (https://developers.google.com/protocol-buffers/docs/reference/google.protobuf).
    #[prost(message, optional, tag = "1")]
    pub proto_descriptor: ::core::option::Option<::prost_types::DescriptorProto>,
}
/// Protobuf rows.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoRows {
    /// A sequence of rows serialized as a Protocol Buffer.
    ///
    /// See https://developers.google.com/protocol-buffers/docs/overview for more
    /// information on deserializing this field.
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub serialized_rows: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// Schema of a table
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableSchema {
    /// Describes the fields in a table.
    #[prost(message, repeated, tag = "1")]
    pub fields: ::prost::alloc::vec::Vec<TableFieldSchema>,
}
/// A field in TableSchema
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableFieldSchema {
    /// Required. The field name. The name must contain only letters (a-z, A-Z),
    /// numbers (0-9), or underscores (_), and must start with a letter or
    /// underscore. The maximum length is 128 characters.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The field data type.
    #[prost(enumeration = "table_field_schema::Type", tag = "2")]
    pub r#type: i32,
    /// Optional. The field mode. The default value is NULLABLE.
    #[prost(enumeration = "table_field_schema::Mode", tag = "3")]
    pub mode: i32,
    /// Optional. Describes the nested schema fields if the type property is set to STRUCT.
    #[prost(message, repeated, tag = "4")]
    pub fields: ::prost::alloc::vec::Vec<TableFieldSchema>,
    /// Optional. The field description. The maximum length is 1,024 characters.
    #[prost(string, tag = "6")]
    pub description: ::prost::alloc::string::String,
}
/// Nested message and enum types in `TableFieldSchema`.
pub mod table_field_schema {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Illegal value
        Unspecified = 0,
        /// 64K, UTF8
        String = 1,
        /// 64-bit signed
        Int64 = 2,
        /// 64-bit IEEE floating point
        Double = 3,
        /// Aggregate type
        Struct = 4,
        /// 64K, Binary
        Bytes = 5,
        /// 2-valued
        Bool = 6,
        /// 64-bit signed usec since UTC epoch
        Timestamp = 7,
        /// Civil date - Year, Month, Day
        Date = 8,
        /// Civil time - Hour, Minute, Second, Microseconds
        Time = 9,
        /// Combination of civil date and civil time
        Datetime = 10,
        /// Geography object
        Geography = 11,
        /// Numeric value
        Numeric = 12,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Mode {
        /// Illegal value
        Unspecified = 0,
        Nullable = 1,
        Required = 2,
        Repeated = 3,
    }
}
/// Information about the ReadSession.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadSession {
    /// Output only. Unique identifier for the session, in the form
    /// `projects/{project_id}/locations/{location}/sessions/{session_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Time at which the session becomes invalid. After this time, subsequent
    /// requests to read this Session will return errors. The expire_time is
    /// automatically assigned and currently cannot be specified or updated.
    #[prost(message, optional, tag = "2")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Immutable. Data format of the output data.
    #[prost(enumeration = "DataFormat", tag = "3")]
    pub data_format: i32,
    /// Immutable. Table that this ReadSession is reading from, in the form
    /// `projects/{project_id}/datasets/{dataset_id}/tables/{table_id}
    #[prost(string, tag = "6")]
    pub table: ::prost::alloc::string::String,
    /// Optional. Any modifiers which are applied when reading from the specified table.
    #[prost(message, optional, tag = "7")]
    pub table_modifiers: ::core::option::Option<read_session::TableModifiers>,
    /// Optional. Read options for this session (e.g. column selection, filters).
    #[prost(message, optional, tag = "8")]
    pub read_options: ::core::option::Option<read_session::TableReadOptions>,
    /// Output only. A list of streams created with the session.
    ///
    /// At least one stream is created with the session. In the future, larger
    /// request_stream_count values *may* result in this list being unpopulated,
    /// in that case, the user will need to use a List method to get the streams
    /// instead, which is not yet available.
    #[prost(message, repeated, tag = "10")]
    pub streams: ::prost::alloc::vec::Vec<ReadStream>,
    /// The schema for the read. If read_options.selected_fields is set, the
    /// schema may be different from the table schema as it will only contain
    /// the selected fields.
    #[prost(oneof = "read_session::Schema", tags = "4, 5")]
    pub schema: ::core::option::Option<read_session::Schema>,
}
/// Nested message and enum types in `ReadSession`.
pub mod read_session {
    /// Additional attributes when reading a table.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TableModifiers {
        /// The snapshot time of the table. If not set, interpreted as now.
        #[prost(message, optional, tag = "1")]
        pub snapshot_time: ::core::option::Option<::prost_types::Timestamp>,
    }
    /// Options dictating how we read a table.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TableReadOptions {
        /// Names of the fields in the table that should be read. If empty, all
        /// fields will be read. If the specified field is a nested field, all
        /// the sub-fields in the field will be selected. The output field order is
        /// unrelated to the order of fields in selected_fields.
        #[prost(string, repeated, tag = "1")]
        pub selected_fields: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// SQL text filtering statement, similar to a WHERE clause in a query.
        /// Aggregates are not supported.
        ///
        /// Examples: "int_field > 5"
        ///           "date_field = CAST('2014-9-27' as DATE)"
        ///           "nullable_field is not NULL"
        ///           "st_equals(geo_field, st_geofromtext("POINT(2, 2)"))"
        ///           "numeric_field BETWEEN 1.0 AND 5.0"
        #[prost(string, tag = "2")]
        pub row_restriction: ::prost::alloc::string::String,
        /// Optional. Options specific to the Apache Arrow output format.
        #[prost(message, optional, tag = "3")]
        pub arrow_serialization_options: ::core::option::Option<super::ArrowSerializationOptions>,
    }
    /// The schema for the read. If read_options.selected_fields is set, the
    /// schema may be different from the table schema as it will only contain
    /// the selected fields.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Schema {
        /// Output only. Avro schema.
        #[prost(message, tag = "4")]
        AvroSchema(super::AvroSchema),
        /// Output only. Arrow schema.
        #[prost(message, tag = "5")]
        ArrowSchema(super::ArrowSchema),
    }
}
/// Information about a single stream that gets data out of the storage system.
/// Most of the information about `ReadStream` instances is aggregated, making
/// `ReadStream` lightweight.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadStream {
    /// Output only. Name of the stream, in the form
    /// `projects/{project_id}/locations/{location}/sessions/{session_id}/streams/{stream_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Information about a single stream that gets data inside the storage system.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteStream {
    /// Output only. Name of the stream, in the form
    /// `projects/{project}/datasets/{dataset}/tables/{table}/streams/{stream}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. Type of the stream.
    #[prost(enumeration = "write_stream::Type", tag = "2")]
    pub r#type: i32,
    /// Output only. Create time of the stream. For the _default stream, this is the
    /// creation_time of the table.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Commit time of the stream.
    /// If a stream is of `COMMITTED` type, then it will have a commit_time same as
    /// `create_time`. If the stream is of `PENDING` type, commit_time being empty
    /// means it is not committed.
    #[prost(message, optional, tag = "4")]
    pub commit_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The schema of the destination table. It is only returned in
    /// `CreateWriteStream` response. Caller should generate data that's
    /// compatible with this schema to send in initial `AppendRowsRequest`.
    /// The table schema could go out of date during the life time of the stream.
    #[prost(message, optional, tag = "5")]
    pub table_schema: ::core::option::Option<TableSchema>,
}
/// Nested message and enum types in `WriteStream`.
pub mod write_stream {
    /// Type enum of the stream.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Unknown type.
        Unspecified = 0,
        /// Data will commit automatically and appear as soon as the write is
        /// acknowledged.
        Committed = 1,
        /// Data is invisible until the stream is committed.
        Pending = 2,
        /// Data is only visible up to the offset to which it was flushed.
        Buffered = 3,
    }
}
/// Data format for input or output data.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DataFormat {
    Unspecified = 0,
    /// Avro is a standard open source row based file format.
    /// See https://avro.apache.org/ for more details.
    Avro = 1,
    /// Arrow is a standard open source column-based message format.
    /// See https://arrow.apache.org/ for more details.
    Arrow = 2,
}
/// Request message for `CreateReadSession`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateReadSessionRequest {
    /// Required. The request project that owns the session, in the form of
    /// `projects/{project_id}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Session to be created.
    #[prost(message, optional, tag = "2")]
    pub read_session: ::core::option::Option<ReadSession>,
    /// Max initial number of streams. If unset or zero, the server will
    /// provide a value of streams so as to produce reasonable throughput. Must be
    /// non-negative. The number of streams may be lower than the requested number,
    /// depending on the amount parallelism that is reasonable for the table. Error
    /// will be returned if the max count is greater than the current system
    /// max limit of 1,000.
    ///
    /// Streams must be read starting from offset 0.
    #[prost(int32, tag = "3")]
    pub max_stream_count: i32,
}
/// Request message for `ReadRows`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadRowsRequest {
    /// Required. Stream to read rows from.
    #[prost(string, tag = "1")]
    pub read_stream: ::prost::alloc::string::String,
    /// The offset requested must be less than the last row read from Read.
    /// Requesting a larger offset is undefined. If not specified, start reading
    /// from offset zero.
    #[prost(int64, tag = "2")]
    pub offset: i64,
}
/// Information on if the current connection is being throttled.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThrottleState {
    /// How much this connection is being throttled. Zero means no throttling,
    /// 100 means fully throttled.
    #[prost(int32, tag = "1")]
    pub throttle_percent: i32,
}
/// Estimated stream statistics for a given Stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamStats {
    /// Represents the progress of the current stream.
    #[prost(message, optional, tag = "2")]
    pub progress: ::core::option::Option<stream_stats::Progress>,
}
/// Nested message and enum types in `StreamStats`.
pub mod stream_stats {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Progress {
        /// The fraction of rows assigned to the stream that have been processed by
        /// the server so far, not including the rows in the current response
        /// message.
        ///
        /// This value, along with `at_response_end`, can be used to interpolate
        /// the progress made as the rows in the message are being processed using
        /// the following formula: `at_response_start + (at_response_end -
        /// at_response_start) * rows_processed_from_response / rows_in_response`.
        ///
        /// Note that if a filter is provided, the `at_response_end` value of the
        /// previous response may not necessarily be equal to the
        /// `at_response_start` value of the current response.
        #[prost(double, tag = "1")]
        pub at_response_start: f64,
        /// Similar to `at_response_start`, except that this value includes the
        /// rows in the current response.
        #[prost(double, tag = "2")]
        pub at_response_end: f64,
    }
}
/// Response from calling `ReadRows` may include row data, progress and
/// throttling information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadRowsResponse {
    /// Number of serialized rows in the rows block.
    #[prost(int64, tag = "6")]
    pub row_count: i64,
    /// Statistics for the stream.
    #[prost(message, optional, tag = "2")]
    pub stats: ::core::option::Option<StreamStats>,
    /// Throttling state. If unset, the latest response still describes
    /// the current throttling status.
    #[prost(message, optional, tag = "5")]
    pub throttle_state: ::core::option::Option<ThrottleState>,
    /// Row data is returned in format specified during session creation.
    #[prost(oneof = "read_rows_response::Rows", tags = "3, 4")]
    pub rows: ::core::option::Option<read_rows_response::Rows>,
}
/// Nested message and enum types in `ReadRowsResponse`.
pub mod read_rows_response {
    /// Row data is returned in format specified during session creation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Rows {
        /// Serialized row data in AVRO format.
        #[prost(message, tag = "3")]
        AvroRows(super::AvroRows),
        /// Serialized row data in Arrow RecordBatch format.
        #[prost(message, tag = "4")]
        ArrowRecordBatch(super::ArrowRecordBatch),
    }
}
/// Request message for `SplitReadStream`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SplitReadStreamRequest {
    /// Required. Name of the stream to split.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A value in the range (0.0, 1.0) that specifies the fractional point at
    /// which the original stream should be split. The actual split point is
    /// evaluated on pre-filtered rows, so if a filter is provided, then there is
    /// no guarantee that the division of the rows between the new child streams
    /// will be proportional to this fractional value. Additionally, because the
    /// server-side unit for assigning data is collections of rows, this fraction
    /// will always map to a data storage boundary on the server side.
    #[prost(double, tag = "2")]
    pub fraction: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SplitReadStreamResponse {
    /// Primary stream, which contains the beginning portion of
    /// |original_stream|. An empty value indicates that the original stream can no
    /// longer be split.
    #[prost(message, optional, tag = "1")]
    pub primary_stream: ::core::option::Option<ReadStream>,
    /// Remainder stream, which contains the tail of |original_stream|. An empty
    /// value indicates that the original stream can no longer be split.
    #[prost(message, optional, tag = "2")]
    pub remainder_stream: ::core::option::Option<ReadStream>,
}
/// Request message for `CreateWriteStream`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWriteStreamRequest {
    /// Required. Reference to the table to which the stream belongs, in the format
    /// of `projects/{project}/datasets/{dataset}/tables/{table}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Stream to be created.
    #[prost(message, optional, tag = "2")]
    pub write_stream: ::core::option::Option<WriteStream>,
}
/// Request message for `AppendRows`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppendRowsRequest {
    /// Required. The stream that is the target of the append operation. This value
    /// must be specified for the initial request. If subsequent requests specify
    /// the stream name, it must equal to the value provided in the first request.
    /// To write to the _default stream, populate this field with a string in the
    /// format `projects/{project}/datasets/{dataset}/tables/{table}/_default`.
    #[prost(string, tag = "1")]
    pub write_stream: ::prost::alloc::string::String,
    /// If present, the write is only performed if the next append offset is same
    /// as the provided value. If not present, the write is performed at the
    /// current end of stream. Specifying a value for this field is not allowed
    /// when calling AppendRows for the '_default' stream.
    #[prost(message, optional, tag = "2")]
    pub offset: ::core::option::Option<i64>,
    /// Id set by client to annotate its identity. Only initial request setting is
    /// respected.
    #[prost(string, tag = "6")]
    pub trace_id: ::prost::alloc::string::String,
    /// Input rows. The `writer_schema` field must be specified at the initial
    /// request and currently, it will be ignored if specified in following
    /// requests. Following requests must have data in the same format as the
    /// initial request.
    #[prost(oneof = "append_rows_request::Rows", tags = "4")]
    pub rows: ::core::option::Option<append_rows_request::Rows>,
}
/// Nested message and enum types in `AppendRowsRequest`.
pub mod append_rows_request {
    /// Proto schema and data.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProtoData {
        /// Proto schema used to serialize the data.
        #[prost(message, optional, tag = "1")]
        pub writer_schema: ::core::option::Option<super::ProtoSchema>,
        /// Serialized row data in protobuf message format.
        #[prost(message, optional, tag = "2")]
        pub rows: ::core::option::Option<super::ProtoRows>,
    }
    /// Input rows. The `writer_schema` field must be specified at the initial
    /// request and currently, it will be ignored if specified in following
    /// requests. Following requests must have data in the same format as the
    /// initial request.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Rows {
        /// Rows in proto format.
        #[prost(message, tag = "4")]
        ProtoRows(ProtoData),
    }
}
/// Response message for `AppendRows`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppendRowsResponse {
    /// If backend detects a schema update, pass it to user so that user can
    /// use it to input new type of message. It will be empty when there is no
    /// schema updates.
    #[prost(message, optional, tag = "3")]
    pub updated_schema: ::core::option::Option<TableSchema>,
    #[prost(oneof = "append_rows_response::Response", tags = "1, 2")]
    pub response: ::core::option::Option<append_rows_response::Response>,
}
/// Nested message and enum types in `AppendRowsResponse`.
pub mod append_rows_response {
    /// A success append result.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AppendResult {
        /// The row offset at which the last append occurred. The offset will not be
        /// set if appending using default streams.
        #[prost(message, optional, tag = "1")]
        pub offset: ::core::option::Option<i64>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        /// Result if the append is successful.
        #[prost(message, tag = "1")]
        AppendResult(AppendResult),
        /// Error in case of request failed. If set, it means rows are not accepted
        /// into the system. Users can retry or continue with other requests within
        /// the same connection.
        /// ALREADY_EXISTS: happens when offset is specified, it means the entire
        ///   request is already appended, it is safe to ignore this error.
        /// OUT_OF_RANGE: happens when offset is specified, it means the specified
        ///   offset is beyond the end of the stream.
        /// INVALID_ARGUMENT: error caused by malformed request or data.
        /// RESOURCE_EXHAUSTED: request rejected due to throttling. Only happens when
        ///   append without offset.
        /// ABORTED: request processing is aborted because of prior failures, request
        ///   can be retried if previous failure is fixed.
        /// INTERNAL: server side errors that can be retried.
        #[prost(message, tag = "2")]
        Error(super::super::super::super::super::rpc::Status),
    }
}
/// Request message for `GetWriteStreamRequest`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWriteStreamRequest {
    /// Required. Name of the stream to get, in the form of
    /// `projects/{project}/datasets/{dataset}/tables/{table}/streams/{stream}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for `BatchCommitWriteStreams`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCommitWriteStreamsRequest {
    /// Required. Parent table that all the streams should belong to, in the form
    /// of `projects/{project}/datasets/{dataset}/tables/{table}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The group of streams that will be committed atomically.
    #[prost(string, repeated, tag = "2")]
    pub write_streams: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Response message for `BatchCommitWriteStreams`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCommitWriteStreamsResponse {
    /// The time at which streams were committed in microseconds granularity.
    /// This field will only exist when there is no stream errors.
    #[prost(message, optional, tag = "1")]
    pub commit_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Stream level error if commit failed. Only streams with error will be in
    /// the list.
    #[prost(message, repeated, tag = "2")]
    pub stream_errors: ::prost::alloc::vec::Vec<StorageError>,
}
/// Request message for invoking `FinalizeWriteStream`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinalizeWriteStreamRequest {
    /// Required. Name of the stream to finalize, in the form of
    /// `projects/{project}/datasets/{dataset}/tables/{table}/streams/{stream}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Response message for `FinalizeWriteStream`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinalizeWriteStreamResponse {
    /// Number of rows in the finalized stream.
    #[prost(int64, tag = "1")]
    pub row_count: i64,
}
/// Request message for `FlushRows`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlushRowsRequest {
    /// Required. The stream that is the target of the flush operation.
    #[prost(string, tag = "1")]
    pub write_stream: ::prost::alloc::string::String,
    /// Ending offset of the flush operation. Rows before this offset(including
    /// this offset) will be flushed.
    #[prost(message, optional, tag = "2")]
    pub offset: ::core::option::Option<i64>,
}
/// Respond message for `FlushRows`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlushRowsResponse {
    /// The rows before this offset (including this offset) are flushed.
    #[prost(int64, tag = "1")]
    pub offset: i64,
}
/// Structured custom BigQuery Storage error message. The error can be attached
/// as error details in the returned rpc Status. User can use the info to process
/// errors in a structural way, rather than having to parse error messages.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageError {
    /// BigQuery Storage specific error code.
    #[prost(enumeration = "storage_error::StorageErrorCode", tag = "1")]
    pub code: i32,
    /// Name of the failed entity.
    #[prost(string, tag = "2")]
    pub entity: ::prost::alloc::string::String,
    /// Message that describes the error.
    #[prost(string, tag = "3")]
    pub error_message: ::prost::alloc::string::String,
}
/// Nested message and enum types in `StorageError`.
pub mod storage_error {
    /// Error code for `StorageError`.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum StorageErrorCode {
        /// Default error.
        Unspecified = 0,
        /// Table is not found in the system.
        TableNotFound = 1,
        /// Stream is already committed.
        StreamAlreadyCommitted = 2,
        /// Stream is not found.
        StreamNotFound = 3,
        /// Invalid Stream type.
        /// For example, you try to commit a stream that is not pending.
        InvalidStreamType = 4,
        /// Invalid Stream state.
        /// For example, you try to commit a stream that is not fianlized or is
        /// garbaged.
        InvalidStreamState = 5,
    }
}
#[doc = r" Generated client implementations."]
pub mod big_query_read_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " BigQuery Read API."]
    #[doc = ""]
    #[doc = " The Read API can be used to read data from BigQuery."]
    #[doc = ""]
    #[doc = " New code should use the v1 Read API going forward, if they don't use Write"]
    #[doc = " API at the same time."]
    pub struct BigQueryReadClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BigQueryReadClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> BigQueryReadClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Creates a new read session. A read session divides the contents of a"]
        #[doc = " BigQuery table into one or more streams, which can then be used to read"]
        #[doc = " data from the table. The read session also specifies properties of the"]
        #[doc = " data to be read, such as a list of columns or a push-down filter describing"]
        #[doc = " the rows to be returned."]
        #[doc = ""]
        #[doc = " A particular row can be read by at most one stream. When the caller has"]
        #[doc = " reached the end of each stream in the session, then all the data in the"]
        #[doc = " table has been read."]
        #[doc = ""]
        #[doc = " Data is assigned to each stream such that roughly the same number of"]
        #[doc = " rows can be read from each stream. Because the server-side unit for"]
        #[doc = " assigning data is collections of rows, the API does not guarantee that"]
        #[doc = " each stream will return the same number or rows. Additionally, the"]
        #[doc = " limits are enforced based on the number of pre-filtered rows, so some"]
        #[doc = " filters can lead to lopsided assignments."]
        #[doc = ""]
        #[doc = " Read sessions automatically expire 24 hours after they are created and do"]
        #[doc = " not require manual clean-up by the caller."]
        pub async fn create_read_session(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateReadSessionRequest>,
        ) -> Result<tonic::Response<super::ReadSession>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.storage.v1beta2.BigQueryRead/CreateReadSession",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Reads rows from the stream in the format prescribed by the ReadSession."]
        #[doc = " Each response contains one or more table rows, up to a maximum of 100 MiB"]
        #[doc = " per response; read requests which attempt to read individual rows larger"]
        #[doc = " than 100 MiB will fail."]
        #[doc = ""]
        #[doc = " Each request also returns a set of stream statistics reflecting the current"]
        #[doc = " state of the stream."]
        pub async fn read_rows(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadRowsRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::ReadRowsResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.storage.v1beta2.BigQueryRead/ReadRows",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Splits a given `ReadStream` into two `ReadStream` objects. These"]
        #[doc = " `ReadStream` objects are referred to as the primary and the residual"]
        #[doc = " streams of the split. The original `ReadStream` can still be read from in"]
        #[doc = " the same manner as before. Both of the returned `ReadStream` objects can"]
        #[doc = " also be read from, and the rows returned by both child streams will be"]
        #[doc = " the same as the rows read from the original stream."]
        #[doc = ""]
        #[doc = " Moreover, the two child streams will be allocated back-to-back in the"]
        #[doc = " original `ReadStream`. Concretely, it is guaranteed that for streams"]
        #[doc = " original, primary, and residual, that original[0-j] = primary[0-j] and"]
        #[doc = " original[j-n] = residual[0-m] once the streams have been read to"]
        #[doc = " completion."]
        pub async fn split_read_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::SplitReadStreamRequest>,
        ) -> Result<tonic::Response<super::SplitReadStreamResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.storage.v1beta2.BigQueryRead/SplitReadStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for BigQueryReadClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for BigQueryReadClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BigQueryReadClient {{ ... }}")
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod big_query_write_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " BigQuery Write API."]
    #[doc = ""]
    #[doc = " The Write API can be used to write data to BigQuery."]
    pub struct BigQueryWriteClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BigQueryWriteClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> BigQueryWriteClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Creates a write stream to the given table."]
        #[doc = " Additionally, every table has a special COMMITTED stream named '_default'"]
        #[doc = " to which data can be written. This stream doesn't need to be created using"]
        #[doc = " CreateWriteStream. It is a stream that can be used simultaneously by any"]
        #[doc = " number of clients. Data written to this stream is considered committed as"]
        #[doc = " soon as an acknowledgement is received."]
        pub async fn create_write_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateWriteStreamRequest>,
        ) -> Result<tonic::Response<super::WriteStream>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.storage.v1beta2.BigQueryWrite/CreateWriteStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Appends data to the given stream."]
        #[doc = ""]
        #[doc = " If `offset` is specified, the `offset` is checked against the end of"]
        #[doc = " stream. The server returns `OUT_OF_RANGE` in `AppendRowsResponse` if an"]
        #[doc = " attempt is made to append to an offset beyond the current end of the stream"]
        #[doc = " or `ALREADY_EXISTS` if user provids an `offset` that has already been"]
        #[doc = " written to. User can retry with adjusted offset within the same RPC"]
        #[doc = " stream. If `offset` is not specified, append happens at the end of the"]
        #[doc = " stream."]
        #[doc = ""]
        #[doc = " The response contains the offset at which the append happened. Responses"]
        #[doc = " are received in the same order in which requests are sent. There will be"]
        #[doc = " one response for each successful request. If the `offset` is not set in"]
        #[doc = " response, it means append didn't happen due to some errors. If one request"]
        #[doc = " fails, all the subsequent requests will also fail until a success request"]
        #[doc = " is made again."]
        #[doc = ""]
        #[doc = " If the stream is of `PENDING` type, data will only be available for read"]
        #[doc = " operations after the stream is committed."]
        pub async fn append_rows(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::AppendRowsRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::AppendRowsResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.storage.v1beta2.BigQueryWrite/AppendRows",
            );
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
        #[doc = " Gets a write stream."]
        pub async fn get_write_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::GetWriteStreamRequest>,
        ) -> Result<tonic::Response<super::WriteStream>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.storage.v1beta2.BigQueryWrite/GetWriteStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Finalize a write stream so that no new data can be appended to the"]
        #[doc = " stream. Finalize is not supported on the '_default' stream."]
        pub async fn finalize_write_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::FinalizeWriteStreamRequest>,
        ) -> Result<tonic::Response<super::FinalizeWriteStreamResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.storage.v1beta2.BigQueryWrite/FinalizeWriteStream",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Atomically commits a group of `PENDING` streams that belong to the same"]
        #[doc = " `parent` table."]
        #[doc = " Streams must be finalized before commit and cannot be committed multiple"]
        #[doc = " times. Once a stream is committed, data in the stream becomes available"]
        #[doc = " for read operations."]
        pub async fn batch_commit_write_streams(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchCommitWriteStreamsRequest>,
        ) -> Result<tonic::Response<super::BatchCommitWriteStreamsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.storage.v1beta2.BigQueryWrite/BatchCommitWriteStreams",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Flushes rows to a BUFFERED stream."]
        #[doc = " If users are appending rows to BUFFERED stream, flush operation is"]
        #[doc = " required in order for the rows to become available for reading. A"]
        #[doc = " Flush operation flushes up to any previously flushed offset in a BUFFERED"]
        #[doc = " stream, to the offset specified in the request."]
        #[doc = " Flush is not supported on the _default stream, since it is not BUFFERED."]
        pub async fn flush_rows(
            &mut self,
            request: impl tonic::IntoRequest<super::FlushRowsRequest>,
        ) -> Result<tonic::Response<super::FlushRowsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.storage.v1beta2.BigQueryWrite/FlushRows",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for BigQueryWriteClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for BigQueryWriteClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BigQueryWriteClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod big_query_read_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with BigQueryReadServer."]
    #[async_trait]
    pub trait BigQueryRead: Send + Sync + 'static {
        #[doc = " Creates a new read session. A read session divides the contents of a"]
        #[doc = " BigQuery table into one or more streams, which can then be used to read"]
        #[doc = " data from the table. The read session also specifies properties of the"]
        #[doc = " data to be read, such as a list of columns or a push-down filter describing"]
        #[doc = " the rows to be returned."]
        #[doc = ""]
        #[doc = " A particular row can be read by at most one stream. When the caller has"]
        #[doc = " reached the end of each stream in the session, then all the data in the"]
        #[doc = " table has been read."]
        #[doc = ""]
        #[doc = " Data is assigned to each stream such that roughly the same number of"]
        #[doc = " rows can be read from each stream. Because the server-side unit for"]
        #[doc = " assigning data is collections of rows, the API does not guarantee that"]
        #[doc = " each stream will return the same number or rows. Additionally, the"]
        #[doc = " limits are enforced based on the number of pre-filtered rows, so some"]
        #[doc = " filters can lead to lopsided assignments."]
        #[doc = ""]
        #[doc = " Read sessions automatically expire 24 hours after they are created and do"]
        #[doc = " not require manual clean-up by the caller."]
        async fn create_read_session(
            &self,
            request: tonic::Request<super::CreateReadSessionRequest>,
        ) -> Result<tonic::Response<super::ReadSession>, tonic::Status>;
        #[doc = "Server streaming response type for the ReadRows method."]
        type ReadRowsStream: futures_core::Stream<Item = Result<super::ReadRowsResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = " Reads rows from the stream in the format prescribed by the ReadSession."]
        #[doc = " Each response contains one or more table rows, up to a maximum of 100 MiB"]
        #[doc = " per response; read requests which attempt to read individual rows larger"]
        #[doc = " than 100 MiB will fail."]
        #[doc = ""]
        #[doc = " Each request also returns a set of stream statistics reflecting the current"]
        #[doc = " state of the stream."]
        async fn read_rows(
            &self,
            request: tonic::Request<super::ReadRowsRequest>,
        ) -> Result<tonic::Response<Self::ReadRowsStream>, tonic::Status>;
        #[doc = " Splits a given `ReadStream` into two `ReadStream` objects. These"]
        #[doc = " `ReadStream` objects are referred to as the primary and the residual"]
        #[doc = " streams of the split. The original `ReadStream` can still be read from in"]
        #[doc = " the same manner as before. Both of the returned `ReadStream` objects can"]
        #[doc = " also be read from, and the rows returned by both child streams will be"]
        #[doc = " the same as the rows read from the original stream."]
        #[doc = ""]
        #[doc = " Moreover, the two child streams will be allocated back-to-back in the"]
        #[doc = " original `ReadStream`. Concretely, it is guaranteed that for streams"]
        #[doc = " original, primary, and residual, that original[0-j] = primary[0-j] and"]
        #[doc = " original[j-n] = residual[0-m] once the streams have been read to"]
        #[doc = " completion."]
        async fn split_read_stream(
            &self,
            request: tonic::Request<super::SplitReadStreamRequest>,
        ) -> Result<tonic::Response<super::SplitReadStreamResponse>, tonic::Status>;
    }
    #[doc = " BigQuery Read API."]
    #[doc = ""]
    #[doc = " The Read API can be used to read data from BigQuery."]
    #[doc = ""]
    #[doc = " New code should use the v1 Read API going forward, if they don't use Write"]
    #[doc = " API at the same time."]
    #[derive(Debug)]
    pub struct BigQueryReadServer<T: BigQueryRead> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: BigQueryRead> BigQueryReadServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for BigQueryReadServer<T>
    where
        T: BigQueryRead,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/google.cloud.bigquery.storage.v1beta2.BigQueryRead/CreateReadSession" => {
                    #[allow(non_camel_case_types)]
                    struct CreateReadSessionSvc<T: BigQueryRead>(pub Arc<T>);
                    impl<T: BigQueryRead>
                        tonic::server::UnaryService<super::CreateReadSessionRequest>
                        for CreateReadSessionSvc<T>
                    {
                        type Response = super::ReadSession;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateReadSessionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_read_session(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateReadSessionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.bigquery.storage.v1beta2.BigQueryRead/ReadRows" => {
                    #[allow(non_camel_case_types)]
                    struct ReadRowsSvc<T: BigQueryRead>(pub Arc<T>);
                    impl<T: BigQueryRead>
                        tonic::server::ServerStreamingService<super::ReadRowsRequest>
                        for ReadRowsSvc<T>
                    {
                        type Response = super::ReadRowsResponse;
                        type ResponseStream = T::ReadRowsStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReadRowsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).read_rows(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = ReadRowsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.bigquery.storage.v1beta2.BigQueryRead/SplitReadStream" => {
                    #[allow(non_camel_case_types)]
                    struct SplitReadStreamSvc<T: BigQueryRead>(pub Arc<T>);
                    impl<T: BigQueryRead> tonic::server::UnaryService<super::SplitReadStreamRequest>
                        for SplitReadStreamSvc<T>
                    {
                        type Response = super::SplitReadStreamResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SplitReadStreamRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).split_read_stream(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SplitReadStreamSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: BigQueryRead> Clone for BigQueryReadServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: BigQueryRead> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: BigQueryRead> tonic::transport::NamedService for BigQueryReadServer<T> {
        const NAME: &'static str = "google.cloud.bigquery.storage.v1beta2.BigQueryRead";
    }
}
#[doc = r" Generated server implementations."]
pub mod big_query_write_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with BigQueryWriteServer."]
    #[async_trait]
    pub trait BigQueryWrite: Send + Sync + 'static {
        #[doc = " Creates a write stream to the given table."]
        #[doc = " Additionally, every table has a special COMMITTED stream named '_default'"]
        #[doc = " to which data can be written. This stream doesn't need to be created using"]
        #[doc = " CreateWriteStream. It is a stream that can be used simultaneously by any"]
        #[doc = " number of clients. Data written to this stream is considered committed as"]
        #[doc = " soon as an acknowledgement is received."]
        async fn create_write_stream(
            &self,
            request: tonic::Request<super::CreateWriteStreamRequest>,
        ) -> Result<tonic::Response<super::WriteStream>, tonic::Status>;
        #[doc = "Server streaming response type for the AppendRows method."]
        type AppendRowsStream: futures_core::Stream<Item = Result<super::AppendRowsResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = " Appends data to the given stream."]
        #[doc = ""]
        #[doc = " If `offset` is specified, the `offset` is checked against the end of"]
        #[doc = " stream. The server returns `OUT_OF_RANGE` in `AppendRowsResponse` if an"]
        #[doc = " attempt is made to append to an offset beyond the current end of the stream"]
        #[doc = " or `ALREADY_EXISTS` if user provids an `offset` that has already been"]
        #[doc = " written to. User can retry with adjusted offset within the same RPC"]
        #[doc = " stream. If `offset` is not specified, append happens at the end of the"]
        #[doc = " stream."]
        #[doc = ""]
        #[doc = " The response contains the offset at which the append happened. Responses"]
        #[doc = " are received in the same order in which requests are sent. There will be"]
        #[doc = " one response for each successful request. If the `offset` is not set in"]
        #[doc = " response, it means append didn't happen due to some errors. If one request"]
        #[doc = " fails, all the subsequent requests will also fail until a success request"]
        #[doc = " is made again."]
        #[doc = ""]
        #[doc = " If the stream is of `PENDING` type, data will only be available for read"]
        #[doc = " operations after the stream is committed."]
        async fn append_rows(
            &self,
            request: tonic::Request<tonic::Streaming<super::AppendRowsRequest>>,
        ) -> Result<tonic::Response<Self::AppendRowsStream>, tonic::Status>;
        #[doc = " Gets a write stream."]
        async fn get_write_stream(
            &self,
            request: tonic::Request<super::GetWriteStreamRequest>,
        ) -> Result<tonic::Response<super::WriteStream>, tonic::Status>;
        #[doc = " Finalize a write stream so that no new data can be appended to the"]
        #[doc = " stream. Finalize is not supported on the '_default' stream."]
        async fn finalize_write_stream(
            &self,
            request: tonic::Request<super::FinalizeWriteStreamRequest>,
        ) -> Result<tonic::Response<super::FinalizeWriteStreamResponse>, tonic::Status>;
        #[doc = " Atomically commits a group of `PENDING` streams that belong to the same"]
        #[doc = " `parent` table."]
        #[doc = " Streams must be finalized before commit and cannot be committed multiple"]
        #[doc = " times. Once a stream is committed, data in the stream becomes available"]
        #[doc = " for read operations."]
        async fn batch_commit_write_streams(
            &self,
            request: tonic::Request<super::BatchCommitWriteStreamsRequest>,
        ) -> Result<tonic::Response<super::BatchCommitWriteStreamsResponse>, tonic::Status>;
        #[doc = " Flushes rows to a BUFFERED stream."]
        #[doc = " If users are appending rows to BUFFERED stream, flush operation is"]
        #[doc = " required in order for the rows to become available for reading. A"]
        #[doc = " Flush operation flushes up to any previously flushed offset in a BUFFERED"]
        #[doc = " stream, to the offset specified in the request."]
        #[doc = " Flush is not supported on the _default stream, since it is not BUFFERED."]
        async fn flush_rows(
            &self,
            request: tonic::Request<super::FlushRowsRequest>,
        ) -> Result<tonic::Response<super::FlushRowsResponse>, tonic::Status>;
    }
    #[doc = " BigQuery Write API."]
    #[doc = ""]
    #[doc = " The Write API can be used to write data to BigQuery."]
    #[derive(Debug)]
    pub struct BigQueryWriteServer<T: BigQueryWrite> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: BigQueryWrite> BigQueryWriteServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for BigQueryWriteServer<T>
    where
        T: BigQueryWrite,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/google.cloud.bigquery.storage.v1beta2.BigQueryWrite/CreateWriteStream" => {
                    #[allow(non_camel_case_types)]
                    struct CreateWriteStreamSvc<T: BigQueryWrite>(pub Arc<T>);
                    impl<T: BigQueryWrite>
                        tonic::server::UnaryService<super::CreateWriteStreamRequest>
                        for CreateWriteStreamSvc<T>
                    {
                        type Response = super::WriteStream;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateWriteStreamRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_write_stream(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateWriteStreamSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.bigquery.storage.v1beta2.BigQueryWrite/AppendRows" => {
                    #[allow(non_camel_case_types)]
                    struct AppendRowsSvc<T: BigQueryWrite>(pub Arc<T>);
                    impl<T: BigQueryWrite> tonic::server::StreamingService<super::AppendRowsRequest>
                        for AppendRowsSvc<T>
                    {
                        type Response = super::AppendRowsResponse;
                        type ResponseStream = T::AppendRowsStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::AppendRowsRequest>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).append_rows(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = AppendRowsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.bigquery.storage.v1beta2.BigQueryWrite/GetWriteStream" => {
                    #[allow(non_camel_case_types)]
                    struct GetWriteStreamSvc<T: BigQueryWrite>(pub Arc<T>);
                    impl<T: BigQueryWrite> tonic::server::UnaryService<super::GetWriteStreamRequest>
                        for GetWriteStreamSvc<T>
                    {
                        type Response = super::WriteStream;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetWriteStreamRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_write_stream(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetWriteStreamSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.bigquery.storage.v1beta2.BigQueryWrite/FinalizeWriteStream" => {
                    #[allow(non_camel_case_types)]
                    struct FinalizeWriteStreamSvc<T: BigQueryWrite>(pub Arc<T>);
                    impl<T: BigQueryWrite>
                        tonic::server::UnaryService<super::FinalizeWriteStreamRequest>
                        for FinalizeWriteStreamSvc<T>
                    {
                        type Response = super::FinalizeWriteStreamResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FinalizeWriteStreamRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).finalize_write_stream(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = FinalizeWriteStreamSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.bigquery.storage.v1beta2.BigQueryWrite/BatchCommitWriteStreams" => {
                    #[allow(non_camel_case_types)]
                    struct BatchCommitWriteStreamsSvc<T: BigQueryWrite>(pub Arc<T>);
                    impl<T: BigQueryWrite>
                        tonic::server::UnaryService<super::BatchCommitWriteStreamsRequest>
                        for BatchCommitWriteStreamsSvc<T>
                    {
                        type Response = super::BatchCommitWriteStreamsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BatchCommitWriteStreamsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).batch_commit_write_streams(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = BatchCommitWriteStreamsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.bigquery.storage.v1beta2.BigQueryWrite/FlushRows" => {
                    #[allow(non_camel_case_types)]
                    struct FlushRowsSvc<T: BigQueryWrite>(pub Arc<T>);
                    impl<T: BigQueryWrite> tonic::server::UnaryService<super::FlushRowsRequest> for FlushRowsSvc<T> {
                        type Response = super::FlushRowsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FlushRowsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).flush_rows(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = FlushRowsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: BigQueryWrite> Clone for BigQueryWriteServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: BigQueryWrite> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: BigQueryWrite> tonic::transport::NamedService for BigQueryWriteServer<T> {
        const NAME: &'static str = "google.cloud.bigquery.storage.v1beta2.BigQueryWrite";
    }
}
