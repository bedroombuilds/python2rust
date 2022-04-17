#![allow(unused)]

#[derive(Debug)]
struct Email {
    date: String,
    from: Vec<String>,
    sender: Option<String>,
    subject: Option<String>,
    cc: Vec<String>,
    bcc: Vec<String>,
    body: String,
}

impl Email {
    fn builder() -> EmailBuilder<NoDate, NoFrom, NoCcBcc> {
        EmailBuilder::new()
    }

    fn send(&self) {
        dbg!(self);
    }
}

struct Date(String);
struct NoDate;

struct OneFrom(String);
struct ManyFrom(Vec<String>);
struct NoFrom;

struct NoCcBcc;
struct Cc(Vec<String>);
struct Bcc(Vec<String>);

struct EmailBuilder<D, F, C> {
    date: D,
    from: F,
    sender: Option<String>,
    copies: C,
    subject: Option<String>,
    body: Option<String>,
}

impl EmailBuilder<NoDate, NoFrom, NoCcBcc> {
    fn new() -> Self {
        Self {
            date: NoDate,
            from: NoFrom,
            sender: None,
            copies: NoCcBcc,
            subject: None,
            body: None,
        }
    }
}

impl<F, C> EmailBuilder<NoDate, F, C> {
    fn date(self, date: impl Into<String>) -> EmailBuilder<Date, F, C> {
        let Self {
            from,
            sender,
            copies,
            subject,
            body,
            ..
        } = self;
        EmailBuilder {
            date: Date(date.into()),
            from,
            sender,
            copies,
            subject,
            body,
        }
    }
}

impl<D, C> EmailBuilder<D, NoFrom, C> {
    fn from(self, new_from: impl Into<String>) -> EmailBuilder<D, OneFrom, C> {
        let Self {
            date,
            sender,
            copies,
            subject,
            body,
            ..
        } = self;
        EmailBuilder {
            date,
            from: OneFrom(new_from.into()),
            sender,
            copies,
            subject,
            body,
        }
    }
}

impl EmailBuilder<Date, OneFrom, NoCcBcc> {
    fn from(self, new_from: impl Into<String>) -> EmailBuilder<Date, ManyFrom, NoCcBcc> {
        let Self {
            date,
            from: OneFrom(first_from),
            sender,
            copies,
            subject,
            body,
        } = self;
        let all_from = vec![first_from, new_from.into()];
        EmailBuilder {
            date,
            from: ManyFrom(all_from),
            sender,
            copies,
            subject,
            body,
        }
    }

    fn cc(self, new_cc: impl Into<String>) -> EmailBuilder<Date, OneFrom, Cc> {
        let Self {
            date,
            from,
            sender,
            copies,
            subject,
            body,
        } = self;
        EmailBuilder {
            date,
            from,
            sender,
            copies: Cc(vec![new_cc.into()]),
            subject,
            body,
        }
    }

    fn subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = Some(subject.into());
        self
    }

    fn body(mut self, txt: impl Into<String>) -> Self {
        self.body = Some(txt.into());
        self
    }

    fn build(self) -> Email {
        let Self {
            date: Date(date),
            from: OneFrom(from),
            sender,
            copies,
            subject,
            body,
        } = self;
        let body = match body {
            Some(b) => b,
            None => "".to_owned(),
        };
        Email {
            date,
            from: vec![from],
            sender,
            cc: vec![],
            bcc: vec![],
            subject,
            body,
        }
    }
}

impl EmailBuilder<Date, ManyFrom, NoCcBcc> {
    fn sender(self, sender: impl Into<String>) -> EmailBuilder<Date, ManyFrom, NoCcBcc> {
        let Self {
            date,
            from,
            copies,
            subject,
            body,
            ..
        } = self;
        EmailBuilder {
            date,
            from,
            sender: Some(sender.into()),
            copies,
            subject,
            body,
        }
    }

    fn from(self, new_from: impl Into<String>) -> EmailBuilder<Date, ManyFrom, NoCcBcc> {
        let Self {
            date,
            from: ManyFrom(mut all_from),
            sender,
            copies,
            subject,
            body,
        } = self;
        all_from.push(new_from.into());
        EmailBuilder {
            date,
            from: ManyFrom(all_from),
            sender,
            copies,
            subject,
            body,
        }
    }

    fn subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = Some(subject.into());
        self
    }

    fn body(mut self, txt: impl Into<String>) -> Self {
        self.body = Some(txt.into());
        self
    }

    fn build(self) -> Email {
        let Self {
            date: Date(date),
            from: ManyFrom(from),
            sender,
            copies,
            subject,
            body,
        } = self;
        if let Some(ref sender) = sender {
            if !from.iter().any(|f| f == sender) {
                panic!("Sender must be an email found in From");
            }
        } else {
            panic!("Sender header obligatory");
        }
        let body = match body {
            Some(b) => b,
            None => "".to_owned(),
        };
        Email {
            date,
            from,
            sender,
            cc: vec![],
            bcc: vec![],
            subject,
            body,
        }
    }
}

/// only copies branch for demonstration
impl EmailBuilder<Date, OneFrom, Cc> {
    fn from(self, new_from: impl Into<String>) -> EmailBuilder<Date, ManyFrom, Cc> {
        let Self {
            date,
            from: OneFrom(first_from),
            sender,
            copies,
            subject,
            body,
        } = self;
        let all_from = vec![first_from, new_from.into()];
        EmailBuilder {
            date,
            from: ManyFrom(all_from),
            sender,
            copies,
            subject,
            body,
        }
    }

    fn cc(self, new_cc: impl Into<String>) -> EmailBuilder<Date, OneFrom, Cc> {
        let Self {
            date,
            from,
            sender,
            copies: Cc(mut copies),
            subject,
            body,
        } = self;
        copies.push(new_cc.into());
        EmailBuilder {
            date,
            from,
            sender,
            copies: Cc(copies),
            subject,
            body,
        }
    }

    fn subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = Some(subject.into());
        self
    }

    fn body(mut self, txt: impl Into<String>) -> Self {
        self.body = Some(txt.into());
        self
    }

    fn build(self) -> Email {
        let Self {
            date: Date(date),
            from: OneFrom(from),
            sender,
            copies: Cc(cc),
            subject,
            body,
        } = self;
        let body = match body {
            Some(b) => b,
            None => "".to_owned(),
        };
        Email {
            date,
            from: vec![from],
            sender,
            cc,
            bcc: vec![],
            subject,
            body,
        }
    }
}

fn main() {
    let e = Email::builder()
        .from("one@example.com")
        .date("2022-04-01")
        .from("two@example.com")
        .sender("notfrom@example.com")
        .from("notfrom@example.com")
        .subject("Optionally usable")
        .body("Message Body")
        .build();
    e.send();
    // one from but with CC
    let e = Email::builder()
        .date("2022-04-01")
        .from("one@example.com")
        .cc("also.to@example.com")
        .subject("One copy")
        .body("Important message")
        .build();
    e.send();
}
