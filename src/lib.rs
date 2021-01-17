mod utils;

use serde_json::Value;
use serde::{Deserialize, Serialize};
use lettre::{Message};
use crate::utils::{get_headers, get_bodies, generate_emails, get_json_data, get_email_contents};

#[derive(Debug, Deserialize)]
pub struct JsonData {
    header: JsonHeader,
    bodies: Vec<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonHeader {
    from: String,
    recipients: Vec<String>,
    subject: String,
}

#[derive(Debug, Clone)]
pub struct Header {
    from: String,
    recipient: String,
    subject: String,
}

impl Header {
    fn create(from: String, recipient: String, subject: String) -> Header {
        Header {
            from,
            recipient,
            subject,
        }
    }
}

/// EmailMessage struct.
/// It is mandatory to instantiate EmailMessage by EmailMessage::create
pub struct EmailMessage<'a> {
    email_path: &'a str,
    json_path: &'a str,
}

impl<'a> EmailMessage<'a> {
    /// Instantiate EmailMessage
    pub fn create(email_path: &'a str, json_path: &'a str) -> EmailMessage<'a> {
        EmailMessage { email_path, json_path }
    }

    /// Build emails
    pub fn build_emails(&self) -> Vec<Message> {
        let json_data = get_json_data(self.json_path);
        let contents = get_email_contents(self.email_path);

        let json_data_header = json_data.header;
        let headers = get_headers(
            json_data_header.recipients,
            json_data_header.from,
            json_data_header.subject,
        );

        let bodies = get_bodies(contents, json_data.bodies);
        generate_emails(headers, bodies)
    }
}
