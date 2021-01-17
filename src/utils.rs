use regex::Regex;
use crate::{Header, JsonData};
use std::fs;
use lettre::{Message};
use serde_json::Value;

const PREFIX: &str = "{";
const SUFFIX: &str = "}";

/// strip template string syntax
fn strip_prefix_suffix(str: String) -> String {
    str.strip_prefix(PREFIX)
        .unwrap()
        .strip_suffix(SUFFIX)
        .unwrap()
        .to_string()
}

/// replace template string with matched value in json
fn get_replaced_str(s: &str, json: &serde_json::Value) -> String {
    let _re = Regex::new(r"[{][a-zA-Z]+[}]").unwrap();
    let mut result: String = String::from(s);

    _re.captures_iter(s).for_each(|caps| {
        let capture = caps.get(0);
        let replaced_str = match capture {
            Some(matched_str) => {
                let unstripped_str = matched_str.as_str();
                let stripped_str = strip_prefix_suffix(unstripped_str.to_string());
                let value = json.get(stripped_str).unwrap().as_str().unwrap();
                result.replace(unstripped_str, value)
            }
            _ => String::from(""),
        };
        result = replaced_str;
    });
    result
}


/// load json data from path
pub fn get_json_data(file_name: &str) -> JsonData {
    let json_file = fs::File::open(file_name).expect("file should open read only");
    serde_json::from_reader(json_file).expect("file should be proper JSON")
}

/// load email data from path
pub fn get_email_contents(email_file_path: &str) -> String {
    fs::read_to_string(email_file_path)
        .expect("Something went wrong reading the template config file")
}

/// get multiple headers by multiple recipients, from, and subject
pub fn get_headers(recipients: Vec<String>, from: String, subject: String) -> Vec<Header> {
    recipients
        .iter()
        .map(|recipient| {
            Header::create(from.to_string(), recipient.to_string(), subject.to_string())
        })
        .collect()
}

/// get body contents by matching text contents and json
pub fn get_bodies(contents: String, json_bodies: Vec<Value>) -> Vec<String> {
    json_bodies
        .iter()
        .map(|body| {
            contents
                .lines()
                .map(|s| get_replaced_str(s, &body))
                .into_iter()
                .fold(String::new(), |mut acc, v| {
                    let next_line = String::from("\n");
                    let concat_str = [next_line, v].concat();
                    acc.push_str(concat_str.as_str());
                    acc
                })
        })
        .collect()
}

// generate emails combining header and body
pub fn generate_emails(headers: Vec<Header>, bodies: Vec<String>) -> Vec<Message> {
    let mut emails: Vec<Message> = Vec::new();
    for (i, body) in bodies.iter().enumerate() {
        let header = headers.get(i).unwrap();
        let message = Message::builder()
            .from(header.from.parse().unwrap())
            .to(header.recipient.parse().unwrap())
            .subject(header.subject.parse::<String>().unwrap())
            .body(body)
            .unwrap();
        emails.push(message);
    }
    emails
}

#[cfg(test)]
mod tests {
    use crate::utils::{strip_prefix_suffix, get_replaced_str, get_headers, get_bodies, generate_emails};

    #[test]
    fn strip_template_wrapper() {
        assert_eq!(strip_prefix_suffix(String::from("{abc}")), String::from("abc"));
    }

    #[test]
    fn strip_template_wrapper_only_outter() {
        assert_eq!(strip_prefix_suffix(String::from("{{abc}}")), String::from("{abc}"));
    }

    #[test]
    fn replace_template_with_matched_value() {
        let s = "dance is {feeling}";
        let json = serde_json::json!({
            "feeling": "fun"
        });
        assert_eq!(get_replaced_str(s, &json), String::from("dance is fun"));
    }

    #[test]
    fn get_header_formats() {
        let from = "me <abc@gmail.com>";
        let subject = "hello";
        let someone1 = "someone1 <someone1@gmail.com>";
        let someone2 = "someone2 <someone2@gmail.com>";
        let recipients = vec![String::from(someone1), String::from(someone2)];
        let headers = get_headers(recipients.clone(), String::from(from), String::from(subject));
        for (i, header) in headers.iter().enumerate() {
            assert_eq!(header.from, from);
            assert_eq!(header.subject, subject);
            assert_eq!(header.recipient, *recipients.get(i).unwrap());
        }
    }

    #[test]
    fn get_body_formats() {
        let contents = String::from("Hello, {name}.\nI am so {feeling}.");
        let json1 = serde_json::json!({
            "name": "buddy",
            "feeling": "great"
        });
        let json2 = serde_json::json!({
            "name": "mate",
            "feeling": "happy"
        });
        let json_bodies = vec![json1, json2];
        let bodies = get_bodies(contents, json_bodies);
        let expected_strs = vec!["\nHello, buddy.\nI am so great.", "\nHello, mate.\nI am so happy."];
        for (i, body) in bodies.iter().enumerate() {
            assert_eq!(body, expected_strs.get(i).unwrap());
        }
    }

    #[test]
    fn get_email_formats() {
        let from = "me <abc@gmail.com>";
        let subject = "hello";
        let someone1 = "someone1 <someone1@gmail.com>";
        let someone2 = "someone2 <someone2@gmail.com>";
        let recipients = vec![String::from(someone1), String::from(someone2)];
        let headers = get_headers(recipients.clone(), String::from(from), String::from(subject));

        let contents = String::from("Hello, {name}.\nI am so {feeling}.");
        let json1 = serde_json::json!({
            "name": "buddy",
            "feeling": "great"
        });
        let json2 = serde_json::json!({
            "name": "mate",
            "feeling": "happy"
        });
        let json_bodies = vec![json1, json2];
        let bodies = get_bodies(contents, json_bodies);

        let emails = generate_emails(headers.clone(), bodies);
        for (i, email) in emails.iter().enumerate() {
            let header = headers.get(i).unwrap();
            email.headers().iter().for_each(|header_view| {
                let value = header_view.value_string();
                match header_view.name() {
                    "From" => assert_eq!(value, header.from),
                    "To" => assert_eq!(value, header.recipient),
                    "Subject" => assert_eq!(value, header.subject),
                    _ => ()
                }
            })
        }
    }
}

