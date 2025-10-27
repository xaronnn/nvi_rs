pub mod kps_urls {
    pub const STS: &str = "https://kimlikdogrulama.nvi.gov.tr/Services/Issuer.svc/IWSTrust13";
    pub const QUERY: &str = "https://kpsv2.nvi.gov.tr/Services/RoutingService.svc";
}

pub mod namespaces {
    pub const SOAP_NS_12: &str = "http://www.w3.org/2003/05/soap-envelope";
    pub const WSU_NS: &str =
        "http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-utility-1.0.xsd";
    pub const WSSE_NS: &str =
        "http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-secext-1.0.xsd";
    pub const WSA_NS: &str = "http://www.w3.org/2005/08/addressing";
    pub const DSIG_NS: &str = "http://www.w3.org/2000/09/xmldsig#";
    pub const TRUST_NS: &str = "http://docs.oasis-open.org/ws-sx/ws-trust/200512";
    pub const WSP_NS: &str = "http://schemas.xmlsoap.org/ws/2004/09/policy";
    pub const BODY_NS: &str = "http://kps.nvi.gov.tr/2025/08/01";
}

pub mod service_uris {
    pub const METHOD_URI: &str = "http://kps.nvi.gov.tr/2025/08/01/TumKutukDogrulaServis/Sorgula";
    pub const RST_ACTION: &str = "http://docs.oasis-open.org/ws-sx/ws-trust/200512/RST/Issue";
    pub const TOKEN_TYPE: &str =
        "http://docs.oasis-open.org/wss/oasis-wss-saml-token-profile-1.1#SAMLV1.1";
    pub const REQUEST_TYPE: &str = "http://docs.oasis-open.org/ws-sx/ws-trust/200512/Issue";
    pub const KEY_TYPE: &str = "http://docs.oasis-open.org/ws-sx/ws-trust/200512/SymmetricKey";
    pub const PASSWORD_TYPE: &str = "http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-username-token-profile-1.0#PasswordText";
}

pub mod algorithms {
    pub const C14N: &str = "http://www.w3.org/2001/10/xml-exc-c14n#";
    pub const HMAC_SHA1: &str = "http://www.w3.org/2000/09/xmldsig#hmac-sha1";
    pub const SHA1: &str = "http://www.w3.org/2000/09/xmldsig#sha1";
    pub const SAML_ASSERTION_ID: &str =
        "http://docs.oasis-open.org/wss/oasis-wss-saml-token-profile-1.0#SAMLAssertionID";
}

pub mod xml_tags {
    pub const RESULT_CODE: &str = "SonucKodu";
    pub const RESULT_MESSAGE: &str = "SonucMesaji";
    pub const DESCRIPTION: &str = "Aciklama";
    pub const PERSON_TYPE: &str = "KisiTipi";

    pub const REQUESTED_SECURITY_TOKEN: &str = "RequestedSecurityToken";

    pub const NATIONAL_ID: &str = "TCKimlikNo";
    pub const FIRST_NAME: &str = "Ad";
    pub const LAST_NAME: &str = "Soyad";
    pub const BIRTH_YEAR: &str = "DogumYili";
    pub const BIRTH_MONTH: &str = "DogumAy";
    pub const BIRTH_DAY: &str = "DogumGun";
}

pub const SOAP_CONTENT_TYPE: &str = "application/soap+xml; charset=utf-8";

pub const DEFAULT_TIMEOUT_MS: u64 = 30_000;
