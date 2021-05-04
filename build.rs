fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().out_dir("./src/google").compile(
        &[
            "googleapis/google/cloud/bigquery/storage/v1beta2/arrow.proto",
            "googleapis/google/cloud/bigquery/storage/v1beta2/avro.proto",
            "googleapis/google/cloud/bigquery/storage/v1beta2/storage.proto",
            "googleapis/google/cloud/bigquery/storage/v1beta2/stream.proto",
        ],
        &["googleapis"],
    )?;
    Ok(())
}
