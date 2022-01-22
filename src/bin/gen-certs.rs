fn main() {
    let mut params = rcgen::CertificateParams::new(vec!["pyxis".into()]);
    params
        .subject_alt_names
        .push(rcgen::SanType::IpAddress("127.0.0.1".parse().unwrap()));
    let cert = rcgen::Certificate::from_params(params).unwrap();
    let key = cert.serialize_private_key_der();
    let cert = cert.serialize_der().unwrap();
    std::fs::write("pyxis-key.pub", &cert).unwrap();
    std::fs::write("pyxis-key", &key).unwrap();
}
