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
data.json
```json
{
  "header": {
    "from": "user1 <user1@gmail.com>",
    "recipients": [
      "user2 <user2@gmail.com>",
      "user3 <user3@gmail.com>"
    ],
    "subject": "hello everyone"
  },
  "bodies": [{
    "name": "user2",
    "phone": "123-456-789",
    "company": "company1.",
    "text": "hi there, I hope you are doing well."
  },{
    "name": "user3",
    "phone": "777-888-999",
    "company": "company2.",
    "text": "hi there, I hope you are doing well."
  }]
}
```

email.txt
```text
Dear {name},

{text} {name}
I am glad to invite you to our community.
Here's details, and let me know if you have any questions.

Looking forward to hearing from you soon.

Regards,
Somebody name,
{phone}
{company}
```

#### License

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

