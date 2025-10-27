use crate::errors::KPSError;
use crate::soap::build_verify_soap;
use crate::sts::acquire_token;
use crate::xml::extract_first_tag_text;
use reqwest::Client as HttpClient;
use std::collections::HashMap;
use tracing::debug;

#[derive(Debug, Clone)]
pub struct NviClientConfig {
    pub username: String,
    pub password: String,
    pub sts_url: String,
    pub service_url: String,
    pub timeout_secs: u64,
}

impl Default for NviClientConfig {
    fn default() -> Self {
        Self {
            username: String::new(),
            password: String::new(),
            sts_url: String::new(),
            service_url: String::new(),
            timeout_secs: 30,
        }
    }
}

#[derive(Clone)]
pub struct NviClient {
    cfg: NviClientConfig,
    http: HttpClient,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PersonType {
    TcVatandasi,
    Yabanci,
    MaviKart,
}

#[derive(Debug, Clone)]
pub struct QueryResult {
    pub status: bool,
    pub code: u8,
    pub description: Option<String>,
    pub person: Option<PersonType>,
    pub extra: HashMap<String, String>,
    pub raw: String,
}

impl NviClient {
    pub fn new(cfg: NviClientConfig) -> Self {
        let http = HttpClient::builder()
            .timeout(std::time::Duration::from_secs(cfg.timeout_secs))
            .build()
            .expect("failed to build http client");
        Self { cfg, http }
    }

    pub async fn verify(
        &self,
        national_id: &str,
        first_name: &str,
        last_name: &str,
        birth_year: &str,
        birth_month: Option<&str>,
        birth_day: Option<&str>,
    ) -> Result<QueryResult, KPSError> {
        crate::soap::validate_inputs(national_id, first_name, last_name, birth_year)?;
        let token = acquire_token(
            &self.http,
            &self.cfg.sts_url,
            &self.cfg.username,
            &self.cfg.password,
            &self.cfg.service_url,
            self.cfg.timeout_secs,
        )
        .await?;

        let body = build_verify_soap(
            national_id,
            first_name,
            last_name,
            birth_year,
            birth_month,
            birth_day,
        );

        debug!(service = %self.cfg.service_url, "Sending Verify SOAP request to configured service endpoint");

        let resp = self
            .http
            .post(&self.cfg.service_url)
            .header(
                reqwest::header::CONTENT_TYPE,
                "application/soap+xml; charset=utf-8",
            )
            .header("Authorization", format!("Bearer {}", token))
            .body(body.clone())
            .send()
            .await
            .map_err(KPSError::Network)?;

        let text = resp.text().await.map_err(KPSError::Network)?;

        self.parse_response(&text)
    }

    fn parse_response(&self, doc: &str) -> Result<QueryResult, KPSError> {
        let code = extract_first_tag_text(doc, "SonucKodu")
            .ok()
            .flatten()
            .and_then(|s| s.parse::<u8>().ok())
            .unwrap_or(0);

        let description = extract_first_tag_text(doc, "SonucMesaji")
            .ok()
            .flatten()
            .or_else(|| extract_first_tag_text(doc, "Aciklama").ok().flatten());

        let mut extra = HashMap::new();
        if let Ok(Some(person_type)) = extract_first_tag_text(doc, "KisiTipi") {
            extra.insert("KisiTipi".into(), person_type);
        }

        let status = code == 1;
        let person = extra.get("KisiTipi").map(|s| match s.as_str() {
            "TC_VATANDASI" | "tc_vatandasi" => PersonType::TcVatandasi,
            "YABANCI" | "yabanci" => PersonType::Yabanci,
            "MAVI_KART" | "mavi_kart" => PersonType::MaviKart,
            _ => PersonType::TcVatandasi,
        });

        Ok(QueryResult {
            status,
            code,
            description,
            person,
            extra,
            raw: doc.to_string(),
        })
    }
}

pub type KPSClient = NviClient;
pub type KPSClientConfig = NviClientConfig;
pub type QueryResultAlias = QueryResult;
pub type PersonTypeAlias = PersonType;
