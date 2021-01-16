use email_template::EmailMessage;

fn main() {
    let email_message = EmailMessage::create("./examples/email.txt", "./examples/data.json");
    let emails = email_message.build_emails();
    println!("{:?}", emails);
}