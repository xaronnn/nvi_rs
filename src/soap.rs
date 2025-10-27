use crate::errors::KPSError;
use hmac::{Hmac, Mac};
use sha1::Sha1;

type HmacSha1 = Hmac<Sha1>;
pub fn build_verify_soap(
    national_id: &str,
    first_name: &str,
    last_name: &str,
    birth_year: &str,
    birth_month: Option<&str>,
    birth_day: Option<&str>,
) -> String {
    let birth_month = birth_month.unwrap_or("");
    let birth_day = birth_day.unwrap_or("");
    format!(
        r#"<?xml version=\"1.0\" encoding=\"utf-8\"?>
<s:Envelope xmlns:s=\"http://www.w3.org/2003/05/soap-envelope\">\n<s:Header/>\n<s:Body>\n<Verify xmlns=\"http://kps.nvi.gov.tr/IKPS/Service\">\n<TCKimlikNo>{}</TCKimlikNo>\n<Ad>{}</Ad>\n<Soyad>{}</Soyad>\n<DogumYili>{}</DogumYili>\n<DogumAy>{}</DogumAy>\n<DogumGun>{}</DogumGun>\n</Verify>\n</s:Body>\n</s:Envelope>"#,
        national_id, first_name, last_name, birth_year, birth_month, birth_day
    )
}

pub fn compute_hmac_base64(key: &str, payload: &[u8]) -> String {
    let mut mac =
        HmacSha1::new_from_slice(key.as_bytes()).expect("HMAC can accept key of any size");
    mac.update(payload);
    let result = mac.finalize().into_bytes();
    base64::engine::general_purpose::STANDARD.encode(result.as_slice())
}

pub fn validate_inputs(
    national_id: &str,
    first_name: &str,
    last_name: &str,
    birth_year: &str,
) -> Result<(), KPSError> {
    if national_id.len() != 11 || !national_id.chars().all(|c| c.is_ascii_digit()) {
        return Err(KPSError::Validation(
            "TCKimlikNo (national id) must be 11 digits".into(),
        ));
    }
    if first_name.trim().is_empty() || last_name.trim().is_empty() {
        return Err(KPSError::Validation(
            "First name and last name cannot be empty".into(),
        ));
    }
    if birth_year.len() != 4 || !birth_year.chars().all(|c| c.is_ascii_digit()) {
        return Err(KPSError::Validation(
            "Birth year must be a 4-digit year".into(),
        ));
    }
    Ok(())
}
