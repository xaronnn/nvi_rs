use crate::errors::KPSError;
use crate::xml::extract_first_tag_text;
use reqwest::Client;
use tracing::debug;

pub fn build_rst(username: &str, password: &str, applies_to: &str) -> String {
    let id = uuid::Uuid::new_v4();
    format!(
        r#"<?xml version=\"1.0\" encoding=\"utf-8\"?>
<s:Envelope xmlns:s=\"http://www.w3.org/2003/05/soap-envelope\" xmlns:u=\"http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-utility-1.0.xsd\">
<s:Header>
<o:Security xmlns:o=\"http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-secext-1.0.xsd\">
<o:UsernameToken u:Id=\"uuid-{id}\">
<o:Username>{username}</o:Username>
<o:Password>{password}</o:Password>
</o:UsernameToken>
</o:Security>
</s:Header>
<s:Body>
<RequestSecurityToken xmlns=\"http://docs.oasis-open.org/ws-sx/ws-trust/200512\">
<AppliesTo xmlns=\"http://schemas.xmlsoap.org/ws/2004/09/policy\">
<EndpointReference xmlns=\"http://www.w3.org/2005/08/addressing\">
<Address>{applies_to}</Address>
</EndpointReference>
</AppliesTo>
<KeyType>http://docs.oasis-open.org/ws-sx/ws-trust/200512/Bearer</KeyType>
<RequestType>http://docs.oasis-open.org/ws-sx/ws-trust/200512/Issue</RequestType>
</RequestSecurityToken>
</s:Body>
</s:Envelope>"#
    )
}

pub async fn acquire_token(
    client: &Client,
    sts_url: &str,
    username: &str,
    password: &str,
    applies_to: &str,
    timeout_secs: u64,
) -> Result<String, KPSError> {
    let rst = build_rst(username, password, applies_to);
    debug!(
        "Sending WS-Trust RequestSecurityToken to STS endpoint: {}",
        sts_url
    );

    let resp = client
        .post(sts_url)
        .header(
            reqwest::header::CONTENT_TYPE,
            "application/soap+xml; charset=utf-8",
        )
        .body(rst)
        .send()
        .await
        .map_err(KPSError::Network)?;

    let text = resp.text().await.map_err(KPSError::Network)?;
    debug!(
        "Received STS response; payload length = {} bytes",
        text.len()
    );

    if let Ok(Some(token_xml)) = extract_first_tag_text(&text, "RequestedSecurityToken") {
        Ok(token_xml)
    } else {
        Err(KPSError::STS(
            "failed to extract RequestedSecurityToken".into(),
        ))
    }
}
