use nvi_rs::{KPSClient, KPSClientConfig};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn verify_flow_happy_path() {
    let sts = MockServer::start().await;
    let kps = MockServer::start().await;

    let sts_response = r#"<s:Envelope><s:Body><RequestedSecurityToken><BinarySecurityToken>fake-token</BinarySecurityToken></RequestedSecurityToken></s:Body></s:Envelope>"#;
    Mock::given(method("POST"))
        .and(path("/"))
        .respond_with(ResponseTemplate::new(200).set_body_string(sts_response))
        .mount(&sts)
        .await;

    let kps_response = r#"<s:Envelope><s:Body><VerifyResponse><SonucKodu>1</SonucKodu><SonucMesaji>OK</SonucMesaji><KisiTipi>TC_VATANDASI</KisiTipi></VerifyResponse></s:Body></s:Envelope>"#;
    Mock::given(method("POST"))
        .and(path("/"))
        .respond_with(ResponseTemplate::new(200).set_body_string(kps_response))
        .mount(&kps)
        .await;

    let cfg = KPSClientConfig {
        username: "username".into(),
        password: "password".into(),
        sts_url: sts.uri(),
        service_url: kps.uri(),
        ..Default::default()
    };

    let client = KPSClient::new(cfg);
    let res = client
        .verify("12345678901", "UĞUR", "PEKESEN", "1995", None, None)
        .await
        .expect("Verification should succeed");
    assert!(res.status);
    assert_eq!(res.code, 1);
}
