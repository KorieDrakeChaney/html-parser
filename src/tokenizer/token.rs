#[derive(Debug, PartialEq, Clone)]
pub struct Attribute {
    pub name: String,
    pub value: String,
    pub prefix: String,
    pub namespace: String,
}

impl Attribute {
    pub fn new() -> Self {
        Attribute {
            name: String::new(),
            value: String::new(),
            prefix: String::new(),
            namespace: String::new(),
        }
    }
}

impl std::fmt::Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} : {}", self.name, self.value)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Doctype {
    pub name: String,
    pub public_id: Option<String>,
    pub system_id: Option<String>,
    pub force_quirks: bool,
}

impl Doctype {
    pub fn new() -> Self {
        Doctype {
            name: String::new(),
            public_id: None,
            system_id: None,
            force_quirks: false,
        }
    }

    pub fn new_with_name(name: String) -> Self {
        Doctype {
            name,
            public_id: None,
            system_id: None,
            force_quirks: false,
        }
    }

    pub fn set_quirks_flag_to_on(&mut self) {
        self.force_quirks = true;
    }

    pub fn append_character_to_name(&mut self, c: char) {
        self.name.push(c);
    }

    pub fn append_character_to_public_identifier(&mut self, c: char) {
        if let Some(public_id) = &mut self.public_id {
            public_id.push(c);
        }
    }

    pub fn append_character_to_system_identifier(&mut self, c: char) {
        if let Some(system_id) = &mut self.system_id {
            system_id.push(c);
        }
    }

    pub fn set_public_identifier_to_empty_string(&mut self) {
        self.public_id = Some(String::new());
    }

    pub fn set_system_identifier_to_empty_string(&mut self) {
        self.system_id = Some(String::new());
    }
}

impl std::fmt::Display for Doctype {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<!DOCTYPE html>")
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Tag {
    pub tag_name: String,
    pub self_closing: bool,
    pub attributes: Vec<Attribute>,
    pub is_end_tag: bool,
    pub self_closing_acknowledged: bool,
}

impl Tag {
    pub fn new_start_tag() -> Self {
        Tag {
            tag_name: String::new(),
            self_closing: false,
            attributes: Vec::new(),
            is_end_tag: false,
            self_closing_acknowledged: false,
        }
    }

    pub fn new_end_tag() -> Self {
        Tag {
            tag_name: String::new(),
            self_closing: false,
            attributes: Vec::new(),
            is_end_tag: true,
            self_closing_acknowledged: false,
        }
    }

    pub fn new_end_tag_with_name(name: String) -> Self {
        Tag {
            tag_name: name,
            self_closing: false,
            attributes: Vec::new(),
            is_end_tag: true,
            self_closing_acknowledged: false,
        }
    }

    pub fn new_start_tag_with_name(name: String) -> Self {
        Tag {
            tag_name: name,
            self_closing: false,
            attributes: Vec::new(),
            is_end_tag: false,
            self_closing_acknowledged: false,
        }
    }

    pub fn new_attribute(&mut self) {
        self.attributes.push(Attribute::new());
    }

    pub fn append_character_to_attribute_name(&mut self, c: char) {
        if let Some(attr) = self.attributes.last_mut() {
            attr.name.push(c);
        }
    }

    pub fn append_character_to_attribute_value(&mut self, c: char) {
        if let Some(attr) = self.attributes.last_mut() {
            attr.value.push(c);
        }
    }

    pub fn append_character_to_name(&mut self, c: char) {
        self.tag_name.push(c);
    }

    pub fn set_self_closing(&mut self) {
        self.self_closing = true;
    }
}

impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if !self.is_end_tag {
            let _ = write!(f, "<{} ", self.tag_name);

            for attr in &self.attributes {
                let _ = write!(f, "{} ", attr);
            }

            if self.self_closing {
                write!(f, "/>")
            } else {
                write!(f, ">")
            }
        } else {
            write!(f, "</{}>", self.tag_name)
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    DOCTYPE(Doctype),
    Tag(Tag),
    Comment(String),
    Char(char),
    EOF,
}

impl Token {
    pub fn new_start_tag() -> Self {
        Token::Tag(Tag::new_start_tag())
    }

    pub fn new_end_tag() -> Self {
        Token::Tag(Tag::new_end_tag())
    }

    pub fn new_comment() -> Self {
        Token::Comment(String::new())
    }

    pub fn new_doctype() -> Self {
        Token::DOCTYPE(Doctype::new())
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::DOCTYPE(doctype) => {
                write!(f, "{}", doctype)
            }
            Token::Tag(tag) => write!(f, "{}", tag),
            Token::Comment(comment) => write!(f, "<!--{}-->", comment),
            Token::Char(c) => write!(f, "{}", c),
            Token::EOF => write!(f, ""),
        }
    }
}
