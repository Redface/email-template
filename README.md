# Email-template

**Email template generator**

```rust
use email_template::EmailMessage;

fn main() {
    let email_message = EmailMessage::create("./examples/email.txt", "./examples/data.json");
    let emails = email_message.build_emails();
    println!("{:?}", emails);
}
```

#### License

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

