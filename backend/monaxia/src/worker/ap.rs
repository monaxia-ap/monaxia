use super::JobState;
use crate::worker::user::retrieve_public_key;

use monaxia_data::ap::RequestValidation;

use anyhow::Result;
use tracing::debug;

pub(super) async fn validate_request(
    state: JobState,
    json_text: String,
    validation: RequestValidation,
) -> Result<()> {
    debug!(
        "validating signature with headers: {:?}",
        validation.signature_header.headers
    );

    let public_key = retrieve_public_key(&state, &validation.signature_header.key_id).await?;
    debug!("public key is: {public_key:?}");

    Ok(())
}
