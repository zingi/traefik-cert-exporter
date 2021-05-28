use base64::{decode, DecodeError};
use serde::Deserialize;

quick_error! {
    #[derive(Debug)]
    pub enum AcmeError {
        DecodeError(err: base64::DecodeError) {
            from()
            display("{}", err)
        }
        Utf8Error(err: std::str::Utf8Error) {
            from()
            display("{}", err)
        }
        VarError(err: std::env::VarError) {
            from()
            display("{}", err)
        }
        IOError(err: std::io::Error) {
            from()
            display("{}", err)
        }
        SerdeError(err: serde_json::Error) {
            from()
            display("{}", err)
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Acme {
    http: Http,
}

#[derive(Deserialize, Debug)]
pub struct Http {
    #[serde(rename(deserialize = "Certificates"))]
    certificates: Vec<Certificate>,
}

#[derive(Deserialize, Debug)]
pub struct Certificate {
    domain: Domain,
    certificate: String,
    key: String,
    #[serde(rename(deserialize = "Store"))]
    store: String,
}

#[derive(Deserialize, Debug)]
pub struct Domain {
    main: String,
}

impl Acme {
    /// returns the Http instance
    pub fn get_http(&self) -> &Http {
        &self.http
    }
}

impl Http {
    /// returns a vec of all available domains
    pub fn get_domains(&self) -> Vec<String> {
        self.certificates.iter().map(|c| c.get_domain()).collect()
    }

    /// returns all Certificate instances
    pub fn get_certificates(&self) -> &Vec<Certificate> {
        &self.certificates
    }
}

impl Certificate {
    /// returns the corresponding domain as string
    pub fn get_domain(&self) -> String {
        self.domain.main.clone()
    }

    /// returns the tls certificate as plain text string
    pub fn get_certificate(&self) -> Result<String, AcmeError> {
        // decode base64 to byte vec
        let decoded = base64::decode(&self.certificate)?;
        // convert byte vec to string
        let string = std::str::from_utf8(&decoded)?;
        Ok(string.to_string())
    }

    /// returns the name of the certificate file
    pub fn get_certificate_file_name(&self) -> String {
        format!("{}.crt", self.get_domain())
    }

    /// returns the tls key as plain text string
    pub fn get_key(&self) -> Result<String, AcmeError> {
        // decode base64 to byte vec
        let decoded = base64::decode(&self.key)?;
        // convert byte vec to string
        let string = std::str::from_utf8(&decoded)?;
        Ok(string.to_string())
    }

    /// returns the name of the certificate file
    pub fn get_key_file_name(&self) -> String {
        format!("{}.key", self.get_domain())
    }
}
