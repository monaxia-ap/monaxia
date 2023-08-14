use super::JobState;
use crate::worker::user::retrieve_public_key;

use monaxia_data::{ap::RequestValidation, http::DigestAlgorithm};

use anyhow::{bail, Result};
use rsa::{
    pkcs1v15::{Signature, VerifyingKey},
    pkcs8::DecodePublicKey,
    signature::Verifier,
    RsaPublicKey,
};
use sha2::{Digest, Sha256, Sha512};
use tracing::{debug, error, instrument};

#[instrument(skip(state, json_text, validation), fields(key = validation.signature_header.key_id))]
pub(super) async fn validate_request(
    state: JobState,
    json_text: String,
    validation: RequestValidation,
) -> Result<()> {
    debug!("validating signature and digest");

    // body digest
    let request_digest = match validation.digest_header.algorithm {
        DigestAlgorithm::Sha256 => Sha256::digest(&json_text).to_vec(),
        DigestAlgorithm::Sha512 => Sha512::digest(&json_text).to_vec(),
    };
    if request_digest != validation.digest_header.digest_bytes {
        error!("digest does not match");
        bail!("request validation error");
    }

    // header signature
    let verifing_content = {
        let headers = validation.signature_header.headers;
        let mut header_values = validation.header_values;
        let mut header_lines = vec![];
        for header in headers {
            let value = header_values.remove(&header).expect("should be contained");
            header_lines.push(format!("{header}: {value}"));
        }
        header_lines.join("\n")
    };
    let signature: Signature = validation.signature_header.signature[..]
        .try_into()
        .expect("malformed signature");

    let public_key = retrieve_public_key(&state, &validation.signature_header.key_id).await?;
    let rsa_pubkey = RsaPublicKey::from_public_key_pem(&public_key.key_pem)?;
    let verification = match validation.signature_header.algorithm.as_str() {
        "rsa-sha256" => {
            let key = VerifyingKey::<Sha256>::new(rsa_pubkey);
            key.verify(verifing_content.as_bytes(), &signature)
        }
        "hs2019" => {
            let key = VerifyingKey::<Sha512>::new(rsa_pubkey);
            key.verify(verifing_content.as_bytes(), &signature)
        }
        algorithm => {
            error!("unsupported signature algorithm: {algorithm}");
            bail!("request validation error");
        }
    };
    match verification {
        Ok(()) => {
            debug!("signature verified");
        }
        Err(e) => {
            error!("signature failed: {e}");
            bail!("request validation error");
        }
    }

    Ok(())
}
