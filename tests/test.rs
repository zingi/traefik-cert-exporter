use traefik_cert_exporter::export;

#[tokio::test]
async fn read_acme_json() {
    std::env::set_var("ACME_JSON_PATH", "tests/acme.json");
    let acme = export::read_acme_json().await.unwrap();

    let certs: Vec<String> = acme
        .get_http()
        .get_certificates()
        .iter()
        .map(|c| c.get_certificate().unwrap())
        .collect();

    println!("{:?}", certs);
}
