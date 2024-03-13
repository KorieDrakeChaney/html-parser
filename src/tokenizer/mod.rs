mod state;
mod token;

use state::State;
use token::Token;

use std::{collections::VecDeque, iter::Peekable, str::Chars};

use crate::entity::{
    get_entity, is_numeric_control, is_numeric_noncharacter, is_numeric_surrogate,
    is_numeric_whitespace, replace_control, MAX_NUMBER_OF_CHARACTERS_POSSIBLE,
    MAX_NUMBER_OF_CHARACTERS_WITHOUT_SEMICOLON, MIN_NUMBER_OF_CHARACTERS_POSSIBLE,
};

// https://html.spec.whatwg.org/#tokenization
pub struct Scanner<'a> {
    html: Peekable<Chars<'a>>,

    tokens: VecDeque<Token>,

    current_state: State,
    // Character reference state uses a return state to return to the state that invoked it.
    return_state: State,

    current_token: Option<Token>,

    current_character: Option<char>,

    temporary_buffer: Option<String>,

    character_reference_code: u32,

    reconsume: bool,
}

impl<'a> Scanner<'a> {
    pub fn new(html: &'a str) -> Self {
        let mut scanner = Self {
            html: html.chars().peekable(),
            tokens: VecDeque::with_capacity(html.len()),
            current_state: State::Data,
            return_state: State::Data,
            current_token: None,
            current_character: None,
            temporary_buffer: None,
            character_reference_code: 0,
            reconsume: false,
        };

        scanner.scan();

        scanner
    }

    pub fn scan(&mut self) {
        while self.tokens.back() != Some(&Token::EOF) {
            println!("state: {:?}", self.current_state);
            match self.current_state {
                State::Data => self.data_state(),
                State::TagOpen => self.tag_open_state(),
                State::TagName => self.tag_name_state(),
                State::BeforeAttributeName => self.before_attribute_name_state(),
                State::AttributeName => self.attribute_name_state(),
                State::AfterAttributeName => self.after_attribute_name_state(),
                State::BeforeAttributeValue => self.before_attribute_value_state(),
                State::AttributeValueDoubleQuoted => self.attribute_value_double_quoted_state(),
                State::AttributeValueSingleQuoted => self.attribute_value_single_quoted_state(),
                State::AttributeValueUnquoted => self.attribute_value_unquoted_state(),
                State::AfterAttributeValueQuoted => self.after_attribute_value_quoted_state(),
                State::EndTagOpen => self.end_tag_open_state(),
                State::SelfClosingStartTag => self.self_closing_start_tag_state(),
                State::CharacterReference => self.character_reference_state(),
                State::NamedCharacterReference => self.named_character_reference_state(),
                State::NumericCharacterReference => self.numeric_character_reference_state(),
                State::HexadecimalCharacterReferenceStart => {
                    self.hexadecimal_character_reference_start_state()
                }
                State::DecimalCharacterReferenceStart => {
                    self.decimal_character_reference_start_state()
                }
                State::DecimalCharacterReference => self.decimal_character_reference_state(),
                State::HexadecimalCharacterReference => {
                    self.hexadecimal_character_reference_state()
                }
                State::NumericCharacterReferenceEnd => self.numeric_character_reference_end_state(),
                State::BogusComment => self.bogus_comment_state(),
                State::AmbiguousAmpersand => self.ambiguous_ampersand_state(),
                State::MarkupDeclarationOpen => self.markup_declaration_open_state(),
                State::CommentStart => self.comment_start_state(),
                State::CommentStartDash => self.comment_start_dash_state(),
                State::Comment => self.comment_state(),
                State::CommentLessThanSign => self.comment_less_than_sign_state(),
                State::CommentLessThanSignBang => self.comment_less_than_sign_bang_state(),
                State::CommentLessThanSignBangDash => self.comment_less_than_sign_bang_dash_state(),
                State::CommentLessThanSignBangDashDash => {
                    self.comment_less_than_sign_bang_dash_dash_state()
                }
                State::CommentEndDash => self.comment_end_dash_state(),
                State::CommentEnd => self.comment_end_state(),
                State::CommentEndBang => self.comment_end_bang_state(),
                State::DOCTYPE => self.doctype_state(),
                State::BeforeDOCTYPEName => self.before_doctype_name_state(),
                State::DOCTYPEName => self.doctype_name_state(),
                State::AfterDOCTYPEName => self.after_doctype_name_state(),
                State::AfterDOCTYPEPublicKeyword => self.after_doctype_public_keyword_state(),
                State::BeforeDOCTYPEPublicIdentifier => {
                    self.before_doctype_public_identifier_state()
                }
                State::DOCTYPEPublicIdentifierDoubleQuoted => {
                    self.doctype_public_identifier_double_quoted_state()
                }
                State::DOCTYPEPublicIdentifierSingleQuoted => {
                    self.doctype_public_identifier_single_quoted_state()
                }
                State::AfterDOCTYPEPublicIdentifier => self.after_doctype_public_identifier_state(),
                State::BetweenDOCTYPEPublicAndSystemIdentifiers => {
                    self.between_doctype_public_and_system_identifiers_state()
                }
                State::AfterDOCTYPESystemKeyword => self.after_doctype_system_keyword_state(),
                State::BeforeDOCTYPESystemIdentifier => {
                    self.before_doctype_system_identifier_state()
                }
                State::DOCTYPESystemIdentifierDoubleQuoted => {
                    self.doctype_system_identifier_double_quoted_state()
                }
                State::DOCTYPESystemIdentifierSingleQuoted => {
                    self.doctype_system_identifier_single_quoted_state()
                }
                State::AfterDOCTYPESystemIdentifier => self.after_doctype_system_identifier_state(),
                State::BogusDOCTYPE => self.bogus_doctype_state(),
                State::CDATASection => self.cdata_section_state(),
                State::CDATASectionBracket => self.cdata_section_bracket_state(),
                State::CDATASectionEnd => self.cdata_section_end_state(),

                _ => {}
            }
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.tokens.pop_front()
    }

    // https://html.spec.whatwg.org/#data-state
    fn data_state(&mut self) {
        // Consume the next input character:
        if let Some(c) = self.consume() {
            match c {
                // U+0026 AMPERSAND (&)
                // Set the return state to the data state. Switch to the character reference state.
                '&' => {
                    self.set_return_state(State::Data);
                    self.switch_to(State::CharacterReference);
                }
                // U+003C LESS-THAN SIGN (<)
                // Switch to the tag open state.
                '<' => {
                    self.switch_to(State::TagOpen);
                }
                // U+0000 NULL
                // This is an unexpected-null-character parse error. Emit the current input character as a character token.
                '\u{0000}' => self.emit_current_input_character(),
                // Anything else
                // Emit the current input character as a character token.
                _ => self.emit_current_input_character(),
            }
        } else {
            // EOF
            // Emit an end-of-file token.
            self.emit_end_of_file_token();
        }
    }

    // https://html.spec.whatwg.org/#tag-open-state
    fn tag_open_state(&mut self) {
        // Consume the next input character:
        if let Some(c) = self.consume() {
            match c {
                // U+0021 EXCLAMATION MARK (!)
                // Switch to the markup declaration open state.
                '!' => {
                    self.switch_to(State::MarkupDeclarationOpen);
                }
                // U+002F SOLIDUS (/)
                // Switch to the end tag open state.
                '/' => {
                    self.switch_to(State::EndTagOpen);
                }

                // U+003F QUESTION MARK (?)
                // This is a unexpected-question-mark-instead-of-tag-name parse error. Create a comment token whose data is the empty string. Reconsume in the bogus comment state.
                '?' => {
                    self.reconsume_in(State::BogusComment);
                    self.create_new_comment_token();
                }
                _ => {
                    if c.is_alphabetic() {
                        // ASCII alpha
                        // Create a new start tag Token, set its tag name to the empty string. Reconsume in the tag name state.
                        self.create_new_start_tag_token();
                        self.reconsume_in(State::TagName);
                    } else {
                        // Anything else
                        // This is an invalid-first-character-of-tag-name parse error. Emit a U+003C LESS-THAN SIGN character token. Reconsume in the data state.
                        self.reconsume_in(State::Data);
                        self.emit_character_token('\u{003C}');
                    }
                }
            }
        } else {
            // EOF
            // This is an eof-before-tag-parse error. Emit a U+003C LESS-THAN SIGN character token and an end-of-file token.
            self.emit_character_token('\u{003c}');
            self.emit_end_of_file_token();
        }
    }

    // https://html.spec.whatwg.org/#end-tag-open-state
    fn end_tag_open_state(&mut self) {
        // Consume the next input character:
        if let Some(c) = self.consume() {
            match c {
                // U+003E GREATER-THAN SIGN (>)
                // This is a missing-end-tag-name parse error. Switch to the data state.
                '>' => {
                    self.switch_to(State::Data);
                }
                _ => {
                    if c.is_alphabetic() {
                        // ASCII alpha
                        // Create a new end tag token, set its tag name to the empty string. Reconsume in the tag name state.
                        self.create_new_end_tag_token();
                        self.reconsume_in(State::TagName);
                    } else {
                        // Anything else
                        // This is an invalid-first-character-of-tag-name parse error. Create a comment token whose data is the empty string. Reconsume in the bogus comment state.
                        self.reconsume_in(State::BogusComment);
                        self.create_new_comment_token();
                    }
                }
            }
        } else {
            // EOF
            // This is an eof-before-tag-name parse error. Emit a U+003C LESS-THAN SIGN character token, a U+002F SOLIDUS character token, and an end-of-file token.
            self.emit_character_token('\u{00C}');
            self.emit_character_token('\u{002F}');
            self.emit_end_of_file_token();
        }
    }

    // https://html.spec.whatwg.org/#tag-name-state
    fn tag_name_state(&mut self) {
        // Consume the next input character:
        if let Some(c) = self.consume() {
            match c {
                // U+0009 CHARACTER TABULATION (tab)
                // U+000A LINE FEED (LF)
                // U+000C FORM FEED (FF)
                // U+0020 SPACE
                // Switch to the before attribute name state.
                '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {
                    self.switch_to(State::BeforeAttributeName);
                }
                // U+002F SOLIDUS (/)
                // Switch to the self-closing start tag state.
                '/' => {
                    self.switch_to(State::SelfClosingStartTag);
                }
                // U+003E GREATER-THAN SIGN (>)
                // Switch to the data state. Emit the current tag token.
                '>' => {
                    self.switch_to(State::Data);
                    self.emit_current_tag_token()
                }
                // ASCII upper alpha
                // Append the lowercase version of the current input character (add 0x0020 to the character’s code point) to the current tag token’s tag name. Append the current input character to the current tag token’s tag name.
                'A'..='Z' => {
                    self.append_character_to_current_tag_token((c as u8 + 0x0020) as char);
                }
                // U+0000 NULL
                // This is an unexpected-null-character parse error. Append a U+FFFD REPLACEMENT CHARACTER character to the current tag token’s tag name.
                '\u{0000}' => {
                    self.append_character_to_current_tag_token('\u{FFFD}');
                }
                // Anything else
                // Append the current input character to the current tag token’s tag name.
                _ => {
                    self.append_character_to_current_tag_token(c);
                }
            }
        } else {
            // EOF
            // This is an eof-in-tag parse error. Emit an end-of-file token.
            self.emit_end_of_file_token();
        }
    }

    // https://html.spec.whatwg.org/#before-attribute-name-state
    fn before_attribute_name_state(&mut self) {
        // Consume the next input character:
        if let Some(c) = self.consume() {
            match c {
                // U+0009 CHARACTER TABULATION (tab)
                // U+000A LINE FEED (LF)
                // U+000C FORM FEED (FF)
                // U+0020 SPACE
                // Ignore the character.
                '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {}
                // U+002F SOLIDUS (/)
                // U+003E GREATER-THAN SIGN (>)
                // Reconsume in the after attribute name state.
                '/' | '>' => {
                    self.reconsume_in(State::AfterAttributeName);
                }
                // U+003D EQUALS SIGN (=)
                // This is an unexpected-equals-sign-before-attribute-name parse error. Start a new attribute in the current tag token.  Set that attribute's name to the current input character,
                // and its value to the empty string. Switch to the attribute name state.
                '=' => {
                    self.switch_to(State::AttributeName);
                    self.start_a_new_attribute();
                    self.append_character_to_attribute_name(c);
                }
                // Anything else
                // Start a new attribute in the current tag token. Set that attribute's name and value to the empty string. Reconsume in the attribute name state.
                _ => {
                    self.start_a_new_attribute();
                    self.reconsume_in(State::AttributeName);
                }
            }
        } else {
            // EOF
            // Reconsume in the after attribute name state.
            self.reconsume_in(State::AfterAttributeName);
        }
    }

    // https://html.spec.whatwg.org/#attribute-name-state
    /*
       When the user agent leaves the attribute name state (and before emitting the tag token, if appropriate), the complete attribute's name must be compared
       to the other attributes on the same token; if there is already an attribute on the token with the exact same name,
       then this is a parse error and the new attribute must be removed from the token.
    */
    fn attribute_name_state(&mut self) {
        // Consume the next input character:
        if let Some(c) = self.consume() {
            match c {
                // u+0009 CHARACTER TABULATION (tab)
                // U+000A LINE FEED (LF)
                // U+000C FORM FEED (FF)
                // U+0020 SPACE
                // U+002F SOLIDUS (/)
                // U+003E GREATER-THAN SIGN (>)
                // Reconsume in the after attribute name state.
                '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' | '/' | '>' => {
                    self.reconsume_in(State::AfterAttributeName);
                }
                // U+003D EQUALS SIGN (=)
                // Switch to the before attribute value state.
                '=' => {
                    self.switch_to(State::BeforeAttributeValue);
                }
                // ASCII upper alpha
                // Append the lowercase version of the current input character (add 0x0020 to the character’s code point) to the current attribute’s name.
                'A'..='Z' => {
                    self.append_character_to_attribute_name((c as u8 + 0x0020) as char);
                }
                // U+0000 NULL
                // This is an unexpected-null-character parse error. Append a U+FFFD REPLACEMENT CHARACTER character to the current attribute’s name.
                '\u{0000}' => {
                    self.append_character_to_attribute_name(char::REPLACEMENT_CHARACTER);
                }
                // U+0022 QUOTATION MARK (")
                // U+0027 APOSTROPHE (')
                // U+003C LESS-THAN SIGN (<)
                // This is an unexpected-character-in-attribute-name parse error. Treat it as per the "anything else" entry below.
                // Anything else
                // Append the current input character to the current attribute’s name.
                _ => {
                    self.append_character_to_attribute_name(c);
                }
            }
        } else {
            // EOF
            // Reconsume in the after attribute name state.
            self.reconsume_in(State::AfterAttributeName);
        }
    }

    // https://html.spec.whatwg.org/#after-attribute-name-state
    fn after_attribute_name_state(&mut self) {
        // Consume the next input character:
        if let Some(c) = self.consume() {
            match c {
                // U+0009 CHARACTER TABULATION (tab)
                // U+000A LINE FEED (LF)
                // U+000C FORM FEED (FF)
                // U+0020 SPACE
                // Ignore the character.
                '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {}
                // U+002F SOLIDUS (/)
                // Switch to the self-closing start tag state.
                '/' => {
                    self.switch_to(State::SelfClosingStartTag);
                }
                // U+003D EQUALS SIGN (=)
                // Switch to the before attribute value state.
                '=' => {
                    self.switch_to(State::BeforeAttributeValue);
                }
                // U+003E GREATER-THAN SIGN (>)
                // Switch to the data state. Emit the current tag token.
                '>' => {
                    self.switch_to(State::Data);
                    self.emit_current_tag_token()
                }
                // Anything else
                // Start a new attribute in the current tag token. Set that attribute's name and value to the empty string. Reconsume in the attribute name state.
                _ => {
                    self.start_a_new_attribute();
                    self.reconsume_in(State::AttributeName);
                }
            }
        } else {
            // EOF
            // This is an eof-in-tag parse error. Emit an end-of-file token.
            self.emit_end_of_file_token();
        }
    }

    // https://html.spec.whatwg.org/#before-attribute-value-state
    fn before_attribute_value_state(&mut self) {
        // Consume the next input character:
        if let Some(c) = self.consume() {
            match c {
                // U+0009 CHARACTER TABULATION (tab)
                // U+000A LINE FEED (LF)
                // U+000C FORM FEED (FF)
                // U+0020 SPACE
                // Ignore the character.
                '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {}
                // U+0022 QUOTATION MARK (")
                // Switch to the attribute value (double-quoted) state.
                '"' => {
                    self.switch_to(State::AttributeValueDoubleQuoted);
                }
                // U+0027 APOSTROPHE (')
                // Switch to the attribute value (single-quoted) state.
                '\'' => {
                    self.switch_to(State::AttributeValueSingleQuoted);
                }
                // U+003E GREATER-THAN SIGN (>)
                // This is a missing-attribute-value parse error. Switch to the data state. Emit the current tag token.
                '>' => {
                    self.switch_to(State::Data);
                    self.emit_current_tag_token()
                }
                // Anything else
                // Reconsume in the attribute value (unquoted) state.
                _ => {
                    self.reconsume_in(State::AttributeValueUnquoted);
                }
            }
        } else {
            // Anything else
            // Reconsume in the attribute value (unquoted) state.
            self.reconsume_in(State::AttributeValueUnquoted);
        }
    }

    // https://html.spec.whatwg.org/#attribute-value-(double-quoted)-state
    fn attribute_value_double_quoted_state(&mut self) {
        if let Some(c) = self.consume() {
            match c {
                // U+0022 QUOTATION MARK (")
                // Switch to the after attribute value (quoted) state.
                '"' => {
                    self.switch_to(State::AfterAttributeValueQuoted);
                }
                // U+0026 AMPERSAND (&)
                // Set the return state to the attribute value (double-quoted) state. Switch to the character reference state.
                '&' => {
                    self.set_return_state(State::AttributeValueDoubleQuoted);
                    self.switch_to(State::CharacterReference);
                }
                // U+0000 NULL
                // This is an unexpected-null-character parse error. Append a U+FFFD REPLACEMENT CHARACTER character to the current attribute’s value.
                '\u{0000}' => {
                    self.append_character_to_attribute_value(char::REPLACEMENT_CHARACTER);
                }
                // Anything else
                // Append the current input character to the current attribute’s value.
                _ => {
                    self.append_character_to_attribute_value(c);
                }
            }
        } else {
            // EOF
            // This is an eof-in-tag parse error. Emit an end-of-file token.
            self.emit_end_of_file_token();
        }
    }

    // https://html.spec.whatwg.org/#attribute-value-(single-quoted)-state
    fn attribute_value_single_quoted_state(&mut self) {
        // Consume the next input character:
        if let Some(c) = self.consume() {
            match c {
                // U+0027 APOSTROPHE (')
                // Switch to the after attribute value (quoted) state.
                '\'' => {
                    self.switch_to(State::AfterAttributeValueQuoted);
                }
                // U+0026 AMPERSAND (&)
                // Set the return state to the attribute value (single-quoted) state. Switch to the character reference state.
                '&' => {
                    self.set_return_state(State::AttributeValueSingleQuoted);
                    self.switch_to(State::CharacterReference);
                }
                // U+0000 NULL
                // This is an unexpected-null-character parse error. Append a U+FFFD REPLACEMENT CHARACTER character to the current attribute’s value.
                '\u{0000}' => {
                    self.append_character_to_attribute_value(char::REPLACEMENT_CHARACTER);
                }
                // Anything else
                // Append the current input character to the current attribute’s value.
                _ => {
                    self.append_character_to_attribute_value(c);
                }
            }
        } else {
            // EOF
            // This is an eof-in-tag parse error. Emit an end-of-file token.
            self.emit_end_of_file_token();
        }
    }

    // https://html.spec.whatwg.org/#attribute-value-(unquoted)-state
    fn attribute_value_unquoted_state(&mut self) {
        // Consume the next input character:
        if let Some(c) = self.consume() {
            match c {
                // U+0009 CHARACTER TABULATION (tab)
                // U+000A LINE FEED (LF)
                // U+000C FORM FEED (FF)
                // U+0020 SPACE
                // Switch to the before attribute name state.
                '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {
                    self.switch_to(State::BeforeAttributeName);
                }
                // U+0026 AMPERSAND (&)
                // Set the return state to the attribute value (unquoted) state. Switch to the character reference state.
                '&' => {
                    self.set_return_state(State::AttributeValueUnquoted);
                    self.switch_to(State::CharacterReference);
                }
                // U+003E GREATER-THAN SIGN (>)
                // Switch to the data state. Emit the current tag token.
                '>' => {
                    self.switch_to(State::Data);
                    self.emit_current_tag_token()
                }
                // U+0000 NULL
                // This is an unexpected-null-character parse error. Append a U+FFFD REPLACEMENT CHARACTER character to the current attribute’s value.
                '\u{0000}' => {
                    self.append_character_to_attribute_value(char::REPLACEMENT_CHARACTER);
                }
                // U+0022 QUOTATION MARK (")
                // U+0027 APOSTROPHE (')
                // U+003C LESS-THAN SIGN (<)
                // U+003D EQUALS SIGN (=)
                // U+0060 GRAVE ACCENT (`)
                // This is an unexpected-character-in-unquoted-attribute-value parse error. Treat it as per the "anything else" entry below.
                // Anything else
                // Append the current input character to the current attribute’s value.
                _ => {
                    self.append_character_to_attribute_value(c);
                }
            }
        } else {
            // EOF
            // This is an eof-in-tag parse error. Emit an end-of-file token.
            self.emit_end_of_file_token();
        }
    }

    // https://html.spec.whatwg.org/#after-attribute-value-(quoted)-state
    fn after_attribute_value_quoted_state(&mut self) {
        // Consume the next input character:
        if let Some(c) = self.consume() {
            match c {
                // U+0009 CHARACTER TABULATION (tab)
                // U+000A LINE FEED (LF)
                // U+000C FORM FEED (FF)
                // U+0020 SPACE
                // Switch to the before attribute name state.
                '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {
                    self.switch_to(State::BeforeAttributeName);
                }
                // U+002F SOLIDUS (/)
                // Switch to the self-closing start tag state.
                '/' => {
                    self.switch_to(State::SelfClosingStartTag);
                }
                // U+003E GREATER-THAN SIGN (>)
                // Switch to the data state. Emit the current tag token.
                '>' => {
                    self.switch_to(State::Data);
                    self.emit_current_tag_token()
                }
                // Anything else
                // This is a missing-whitespace-between-attributes parse error. Reconsume in the before attribute name state.
                _ => {
                    self.reconsume_in(State::BeforeAttributeName);
                }
            }
        } else {
            // EOF
            // This is an eof-in-tag parse error. Emit an end-of-file token.
            self.emit_end_of_file_token();
        }
    }

    // https://html.spec.whatwg.org/#self-closing-start-tag-state
    fn self_closing_start_tag_state(&mut self) {
        if let Some(c) = self.consume() {
            match c {
                // U+003E GREATER-THAN SIGN (>)
                // Set the self-closing flag of the current tag token. Switch to the data state. Emit the current tag token.
                '>' => {
                    if let Some(token) = &mut self.current_token {
                        if let Token::Tag(tag) = token {
                            tag.set_self_closing();
                        }
                    }
                    self.switch_to(State::Data);
                    self.emit_current_tag_token()
                }
                // Anything else
                // This is an unexpected-solidus-in-tag parse error. Reconsume in the before attribute name state.
                _ => {
                    self.reconsume_in(State::BeforeAttributeName);
                }
            }
        } else {
            // EOF
            // This is an eof-in-tag parse error. Emit an end-of-file token.
            self.emit_end_of_file_token();
        }
    }

    // https://html.spec.whatwg.org/#bogus-comment-state
    fn bogus_comment_state(&mut self) {
        // Consume the next input character:
        if let Some(c) = self.consume() {
            match c {
                // U+003E GREATER-THAN SIGN (>)
                // Switch to the data state. Emit the comment token.
                '>' => {
                    self.switch_to(State::Data);
                    self.emit_current_comment_token();
                }
                // U+0000 NULL
                // Append a U+FFFD REPLACEMENT CHARACTER character to the comment token's data.
                '\u{0000}' => {
                    self.append_character_to_current_comment_token(char::REPLACEMENT_CHARACTER);
                }
                // Anything else
                // Append the current input character to the comment token's data.
                _ => {
                    self.append_character_to_current_comment_token(c);
                }
            }
        } else {
            // EOF
            // Emit the comment token.
            self.reconsume_in(State::Data);
            self.emit_current_comment_token()
        }
    }

    // https://html.spec.whatwg.org/#character-reference-state
    fn character_reference_state(&mut self) {
        // Set the temporary buffer to the empty string. Append a U+0026 AMPERSAND character (&) to the temporary buffer. Reconsume in the named character reference state.
        self.set_temporary_buffer_to_empty_string();
        self.append_character_to_temporary_buffer('&');

        if let Some(c) = self.consume() {
            match c {
                // U+0023 NUMBER SIGN (#)
                // Append the current input character to the temporary buffer. Switch to the numeric character reference state.
                '#' => {
                    self.append_character_to_temporary_buffer(c);
                    self.switch_to(State::NumericCharacterReference);
                }

                _ => {
                    if c.is_alphanumeric() {
                        // ASCII alphanumeric
                        // Reconsume in the named character reference state.
                        self.reconsume_in(State::NamedCharacterReference);
                    } else {
                        // Anything else
                        // Flush code points consumed as a character reference. Reconsume in the return state.
                        self.reconsume_in_return_state();
                    }
                }
            }
        } else {
            // EOF
            // Reconsume in the return state.
            self.reconsume_in_return_state();
        }
    }

    // https://html.spec.whatwg.org/#named-character-reference-state
    fn named_character_reference_state(&mut self) {
        // Consume the maximum number of characters possible, where the consumed characters are one of the identifiers in teh first column of the named character references table.
        // Append each character to the temporary buffer when it's consumed.

        let mut i = 0;

        while i < MAX_NUMBER_OF_CHARACTERS_POSSIBLE {
            if let Some(c) = self.consume() {
                match c {
                    // U+003B SEMICOLON (;)
                    // Append the current input character to the temporary buffer.
                    ';' => {
                        self.append_character_to_temporary_buffer(c);
                        break;
                    }
                    _ => {
                        if c.is_alphabetic() {
                            // ASCII alphabetic
                            // Append the current input character to the temporary buffer.

                            self.append_character_to_temporary_buffer(c);
                        } else {
                            // Anything else
                            break;
                        }
                    }
                }
                i += 1;
            } else {
                break;
            }
        }

        let temporary_buffer = self.get_temporary_buffer();
        let buffer = temporary_buffer.as_str();
        let is_in_attribute = self.is_in_attribute_value();
        let return_state = self.get_return_state();

        if let Some(character_reference) = get_entity(&buffer) {
            // If there is a match
            // If the character reference was consumed as part of an attribute, and the last character matched is not a U+003B SEMICOLON character (;),
            // and the next input character is either a U+003D EQUALS SIGN character (=) or an ASCII alphanumeric, then,
            // for historical reason, flush code points consumed as a character reference and switch to the return state.

            if is_in_attribute
                && self.current_character != Some('=')
                && !self.current_character.is_some_and(|c| c.is_alphanumeric())
            {
                for c in character_reference.chars() {
                    self.append_character_to_attribute_value(c);
                }

                self.switch_to(return_state);
            } else if !is_in_attribute {
                // Otherwise:

                // 1. If the last character matched is not a U+003B SEMICOLON character (;), this is a missing-semicolon-after-character-reference parse error.
                if self.current_character == Some(';') {
                    // todo!()
                    // emit-missing-semicolon-after-character-reference
                }

                // 2. Set the temporary buffer to the empty string. Append one or two character corresponding to the cracter reference name
                //     (as given by the second column of the named character references table) to the temporary buffer.

                for c in character_reference.chars() {
                    self.tokens.push_back(Token::Char(c));
                }

                // 3. Flush code points consumed as a character reference. Switch to the return state.
            } else {
                self.flush_code_points_consumed_as_a_character_reference();
            }
            if self.current_character == Some(';') {
                self.switch_to(return_state);
            } else {
                self.reconsume_in(return_state);
            }
        } else {
            if is_in_attribute {
                self.flush_code_points_consumed_as_a_character_reference();
                if self.current_character == Some(';') {
                    self.switch_to(return_state);
                } else {
                    self.reconsume_in(return_state);
                }
            } else {
                let mut max = MAX_NUMBER_OF_CHARACTERS_WITHOUT_SEMICOLON + 1;

                if max > buffer.len() {
                    max = buffer.len();
                }

                while max > MIN_NUMBER_OF_CHARACTERS_POSSIBLE {
                    if let Some(character_reference) = get_entity(&buffer[..max]) {
                        for c in character_reference.chars() {
                            if is_in_attribute {
                                self.append_character_to_attribute_value(c);
                            } else {
                                self.tokens.push_back(Token::Char(c));
                            }
                        }
                        break;
                    }
                    max -= 1;
                }

                for (index, char) in buffer.chars().enumerate() {
                    if index >= max || max == MIN_NUMBER_OF_CHARACTERS_POSSIBLE {
                        self.tokens.push_back(Token::Char(char));
                    }
                }

                if max != MIN_NUMBER_OF_CHARACTERS_POSSIBLE {
                    if self.current_character == Some(';') {
                        self.switch_to(return_state);
                    } else {
                        self.reconsume_in(return_state);
                    }
                } else {
                    if self.current_character == Some(';') {
                        self.switch_to(State::AmbiguousAmpersand);
                    } else {
                        self.reconsume_in(State::AmbiguousAmpersand);
                    }
                }
            }
        }
    }

    // https://html.spec.whatwg.org/#ambiguous-ampersand-state
    fn ambiguous_ampersand_state(&mut self) {
        if let Some(c) = self.consume() {
            match c {
                // ASCII alphanumeric
                // If the character reference was consumed as part of an attribute, then append the current input character to the current attribute’s value.
                // Otherwise, emit the current input character as a character token.
                'a'..='z' | 'A'..='Z' => match self.return_state {
                    State::AttributeValueUnquoted
                    | State::AttributeValueDoubleQuoted
                    | State::AttributeValueSingleQuoted => {
                        self.append_character_to_attribute_value(c);
                    }
                    _ => {
                        self.emit_character_token(c);
                    }
                },
                // U+003B SEMICOLON (;)
                // This is an ambiguous-ampersand parse error. Switch to the character reference state. Reconsume in the character reference state.
                ';' => {
                    self.reconsume_in_return_state();
                }
                // Anything else
                // Flush code points consumed as a character reference. Reconsume in the return state.
                _ => {
                    self.reconsume_in_return_state();
                }
            }
        } else {
            // EOF
            // Reconsume in the return state.
            self.reconsume_in_return_state();
        }
    }

    // https://html.spec.whatwg.org/#numeric-character-reference-state
    fn numeric_character_reference_state(&mut self) {
        // Set the character reference code to zero (0).
        self.set_character_reference_code_to_zero();

        // Consume the next input character:

        if let Some(c) = self.consume() {
            match c {
                // U+0078 LATIN SMALL LETTER X
                // U+0058 LATIN CAPITAL LETTER X
                // Append the current input character to the temporary buffer. Switch to the hexadecimal character reference start state.
                'x' | 'X' => {
                    self.append_character_to_temporary_buffer(c);
                    self.switch_to(State::HexadecimalCharacterReferenceStart);
                }
                // Anything else
                // Reconsume in the decimal character reference start state.
                _ => {
                    self.reconsume_in(State::DecimalCharacterReferenceStart);
                }
            }
        } else {
            self.reconsume_in(State::DecimalCharacterReferenceStart);
        }
    }

    // https://html.spec.whatwg.org/#hexadecimal-character-reference-start-state
    fn hexadecimal_character_reference_start_state(&mut self) {
        // Consume the next input character:
        if let Some(c) = self.consume() {
            if c.is_ascii_hexdigit() {
                // ASCII hexadecimal digit
                // Reconsume in the hexadecimal character reference state.
                self.reconsume_in(State::HexadecimalCharacterReference);
            } else {
                // Anything else
                // This is an absence-of-digit-in-numeric-character-reference parse error. Flush code points consumed as a character reference. Reconsume in the return state.
                self.flush_code_points_consumed_as_a_character_reference();
                self.reconsume_in_return_state();
            }
        } else {
            self.reconsume_in_return_state();
        }
    }

    // https://html.spec.whatwg.org/#decimal-character-reference-start-state
    fn decimal_character_reference_start_state(&mut self) {
        // Consume the next input character:
        if let Some(c) = self.consume() {
            if c.is_ascii_digit() {
                // ASCII digit
                // Reconsume in the decimal character reference state.
                self.reconsume_in(State::DecimalCharacterReference);
            } else {
                // Anything else
                // This is an absence-of-digit-in-numeric-character-reference parse error. Flush code points consumed as a character reference. Reconsume in the return state.
                self.flush_code_points_consumed_as_a_character_reference();
                self.reconsume_in_return_state();
            }
        } else {
            self.flush_code_points_consumed_as_a_character_reference();
            self.reconsume_in_return_state();
        }
    }

    // https://html.spec.whatwg.org/#hexadecimal-character-reference-state
    fn hexadecimal_character_reference_state(&mut self) {
        // Consume the next input character:

        if let Some(c) = self.consume() {
            // ASCII digit
            // Multiply the character reference code by 16(0x10). Add a numeric version of the current input character (subtract 0x0030 from the character’s code point)
            // to the character reference code.
            if c.is_ascii_digit() {
                self.character_reference_code =
                    self.character_reference_code * 16 + (c as u8 - 0x0030) as u32;
            } else if c.is_ascii_hexdigit() {
                if c >= 'A' && c <= 'F' {
                    // ASCII upper hex digit
                    // Multiply the character reference code by 16(0x10). Add a numeric version of the current input character (subtract 0x0037 from the character’s code point)
                    // to the character reference code.
                    self.character_reference_code =
                        self.character_reference_code * 16 + (c as u8 - 0x0037) as u32;
                } else {
                    // ASCII lower hex digit
                    // Multiply the character reference code by 16(0x10). Add a numeric version of the current input character (subtract 0x0057 from the character’s code point)
                    // to the character reference code.
                    self.character_reference_code =
                        self.character_reference_code * 16 + (c as u8 - 0x0057) as u32;
                }
            } else if c == ';' {
                // U+003B SEMICOLON (;)
                // Switch to the numeric character reference end state.
                self.switch_to(State::NumericCharacterReferenceEnd);
            } else {
                // Anything else
                // This is a missing-semicolon-after-character-reference parse error. Reconsume in the numeric character reference end state.
                self.reconsume_in(State::NumericCharacterReferenceEnd);
            }
        } else {
            self.reconsume_in(State::NumericCharacterReferenceEnd);
        }
    }

    // https://html.spec.whatwg.org/#decimal-character-reference-state
    fn decimal_character_reference_state(&mut self) {
        // Consume the next input character:

        if let Some(c) = self.consume() {
            if c.is_ascii_digit() {
                // ASCII digit
                // Multiply the character reference code by 10(0x0A). Add a numeric version of the current input character (subtract 0x0030 from the character’s code point)
                // to the character reference code.
                self.character_reference_code =
                    self.character_reference_code * 10 + (c as u8 - 0x0030) as u32;
            } else if c == ';' {
                // U+003B SEMICOLON (;)
                // Switch to the numeric character reference end state.
                self.switch_to(State::NumericCharacterReferenceEnd);
            } else {
                // Anything else
                // This is a missing-semicolon-after-character-reference parse error. Reconsume in the numeric character reference end state.
                self.reconsume_in(State::NumericCharacterReferenceEnd);
            }
        } else {
            self.reconsume_in(State::NumericCharacterReferenceEnd);
        }
    }

    // https://html.spec.whatwg.org/#numeric-character-reference-end-state
    fn numeric_character_reference_end_state(&mut self) {
        // Check the character_reference_code :

        // If the number is 0x00, then this is a null-character-reference parse error. Set the character reference code to 0xFFFD
        if self.character_reference_code == 0x00 {
            self.character_reference_code = 0xFFFD;
        }

        // If the number is greater than 0x10FFFF, then this is a character-reference-outside-unicode-range parse error. Set the character reference code to OxFFFD
        if self.character_reference_code > 0x10FFFF {
            self.character_reference_code = 0xFFFD;
        }

        // If the number is a surrogate, then this is a surrogate-character-reference parse error. Set the character reference code to 0xFFFD
        if is_numeric_surrogate(self.character_reference_code) {
            self.character_reference_code = 0xFFFD;
        }

        // If the number is a noncharacter, then this is a noncharacter-character-reference parse error. Set the character reference code to 0xFFFD
        if is_numeric_noncharacter(self.character_reference_code) {
            self.character_reference_code = 0xFFFD;
        }

        // If the number is 0x0D, or a control that's not ASCII whitespace, then this is a control-character-reference parse error. If the number is one of the numbers
        // in the first column of the following table, then find the row with that number in the first column, and set the character reference code to the number
        // in the second column that row.

        if self.character_reference_code == 0x0D
            || is_numeric_control(self.character_reference_code)
                && !is_numeric_whitespace(self.character_reference_code)
        {
            self.character_reference_code = replace_control(self.character_reference_code);
        }

        // Set the temporary buffer to the empty string.
        self.set_temporary_buffer_to_empty_string();

        // Append a code point equal to the character reference code to the temporary buffer.
        self.append_character_to_temporary_buffer(
            std::char::from_u32(self.character_reference_code)
                .unwrap_or(char::REPLACEMENT_CHARACTER),
        );

        // Flush code points consumed as a character reference. Switch to the return state.
        self.flush_code_points_consumed_as_a_character_reference();

        self.switch_to_return_state();
    }

    // https://html.spec.whatwg.org/#markup-declaration-open-state
    fn markup_declaration_open_state(&mut self) {
        // if the next few characters are:

        if let Some(c) = self.consume() {
            match c {
                // Two U+002D HYPHEN-MINUS characters (-)
                '-' => {
                    self.reconsume();
                    self.consume_double_hyphen();
                }
                // ASCII case-insensitive match for the word "DOCTYPE"
                'D' => {
                    self.reconsume();
                    self.consume_doctype();
                }
                // The string "[CDATA[" (the five uppercase letters "CDATA" with a U+005B LEFT SQUARE BRACKET character ([) before and after)
                '[' => {
                    self.reconsume();
                    self.consume_cdata();
                }
                // Anything else
                // This is an incorrectly-opened-comment parse error. Create a comment token whose data is the empty string. Switch to the bogus comment state (don't consume anything in the current state).
                _ => {
                    self.create_new_comment_token();
                    self.reconsume_in(State::BogusComment);
                }
            }
        }
    }

    // https://html.spec.whatwg.org/#comment-start-state
    fn comment_start_state(&mut self) {
        // Consume the next input character:
        if let Some(c) = self.consume() {
            match c {
                // U+002D HYPHEN-MINUS (-)
                // Switch to the comment start dash state.
                '-' => {
                    self.switch_to(State::CommentStartDash);
                }
                // U+003E GREATER-THAN SIGN (>)
                // This is an abrupt-closing-of-empty-comment parse error. Switch to the data state. Emit the current comment token.
                '>' => {
                    self.switch_to(State::Data);
                    self.emit_current_comment_token();
                }
                // Anything else
                // Reconsume in the comment state.
                _ => {
                    self.reconsume_in(State::Comment);
                }
            }
        } else {
            self.reconsume_in(State::Comment);
        }
    }

    // https://html.spec.whatwg.org/#comment-start-dash-state
    fn comment_start_dash_state(&mut self) {
        // Consume the next input character:
        if let Some(c) = self.consume() {
            match c {
                // U+002D HYPHEN-MINUS (-)
                // Switch to the comment end state.
                '-' => {
                    self.switch_to(State::CommentEnd);
                }

                // U+003E GREATER-THAN SIGN (>)
                // This is an abrupt-closing-of-empty-comment parse error. Switch to the data state. Emit the current comment token.
                '>' => {
                    self.switch_to(State::Data);
                    self.emit_current_comment_token();
                }
                // Anything else
                // Append a U+002D HYPHEN-MINUS character (-) to the comment token's data. Reconsume in the comment state.
                _ => {
                    self.append_character_to_current_comment_token('-');
                    self.reconsume_in(State::Comment);
                }
            }
        } else {
            // EOF
            // This is a eof-in-comment parse error. Emit the comment token. Emit an end-of-file token.
            self.emit_current_comment_token();
            self.emit_end_of_file_token();
        }
    }

    // https://html.spec.whatwg.org/#comment-state
    fn comment_state(&mut self) {
        // Consume the next input character:

        if let Some(c) = self.consume() {
            match c {
                // U+003C LESS-THAN SIGN (<)
                // Append the current input character to the comment token's data. Switch to the comment less-than sign state.
                '<' => {
                    self.append_character_to_current_comment_token(c);
                    self.switch_to(State::CommentLessThanSign);
                }
                // U+002D HYPHEN-MINUS (-)
                // Switch to the comment end dash state.
                '-' => {
                    self.switch_to(State::CommentEndDash);
                }
                // U+0000 NULL
                // This is an unexpected-null-character parse error. Append a U+FFFD REPLACEMENT CHARACTER character to the comment token's data.
                '\u{0000}' => {
                    self.append_character_to_current_comment_token(char::REPLACEMENT_CHARACTER);
                }
                // Anything else
                // Append the current input character to the comment token's data.
                _ => {
                    self.append_character_to_current_comment_token(c);
                }
            }
        } else {
            // EOF
            // This is an eof-in-comment parse error. Emit the comment token. Emit an end-of-file token.
            self.emit_current_comment_token();
            self.emit_end_of_file_token();
        }
    }

    // https://html.spec.whatwg.org/#comment-end-state
    fn comment_end_state(&mut self) {
        // Consume the next input character:

        if let Some(c) = self.consume() {
            match c {
                // U+003E GREATER-THAN SIGN (>)
                // Switch to the data state. Emit the comment token.
                '>' => {
                    self.switch_to(State::Data);
                    self.emit_current_comment_token();
                }

                // U+0021 EXCLAMATION MARK (!)
                // Switch to the comment end bang state.
                '!' => {
                    self.switch_to(State::CommentEndBang);
                }

                // U+002D HYPHEN-MINUS (-)
                // Append a U+002D HYPHEN-MINUS character (-) to the comment token's data.
                '-' => {
                    self.append_character_to_current_comment_token('-');
                }

                // Anything else
                // Append two U+002D HYPHEN-MINUS characters (-) to the comment token's data. Reconsume in the comment state.
                _ => {
                    self.append_character_to_current_comment_token('-');
                    self.append_character_to_current_comment_token('-');
                    self.reconsume_in(State::Comment);
                }
            }
        } else {
            // EOF
            // This is an eof-in-comment parse error. Emit the current comment token. Emit an end-of-file token.
            self.emit_current_comment_token();
            self.emit_end_of_file_token();
        }
    }

    // https://html.spec.whatwg.org/#comment-end-bang-state
    fn comment_end_bang_state(&mut self) {
        // Consume the next input character.

        if let Some(c) = self.consume() {
            match c {
                // U+002D HYPHEN-MINUS (-)
                // Append two U+002D HYPHEN-MINUS characters (-) and a U+0021 EXCLAMATION MARK character (!) to the comment token's data. Switch to the comment end dash state.
                '-' => {
                    self.append_character_to_current_comment_token('-');
                    self.append_character_to_current_comment_token('-');
                    self.append_character_to_current_comment_token('!');
                    self.switch_to(State::CommentEndDash);
                }

                // U+003E GREATER-THAN SIGN (>)
                // This is an incorrectly-closed-comment parse error. Switch to the data state. Emit the current comment token.
                '>' => {
                    self.switch_to(State::Data);
                    self.emit_current_comment_token();
                }

                // Anything else
                // Append two U+002D HYPHEN-MINUS characters (-) and a U+0021 EXCLAMATION MARK character (!) to the comment token's data. Reconsume in the comment state.
                _ => {
                    self.append_character_to_current_comment_token('-');
                    self.append_character_to_current_comment_token('-');
                    self.append_character_to_current_comment_token('!');
                    self.reconsume_in(State::Comment);
                }
            }
        } else {
            // EOF
            // This is an eof-in-comment parse error. Emit the current comment token. Emit an end-of-file token.
            self.emit_current_comment_token();
            self.emit_end_of_file_token();
        }
    }

    // https://html.spec.whatwg.org/#comment-less-than-sign-state
    fn comment_less_than_sign_state(&mut self) {
        // Consume the next input character:

        if let Some(c) = self.consume() {
            match c {
                // U+0021 EXCLAMATION MARK (!)
                // Append the current input character to the comment token's data. Switch to the comment less-than sign bang state.
                '!' => {
                    self.append_character_to_current_comment_token(c);
                    self.switch_to(State::CommentLessThanSignBang);
                }
                // U+003C LESS-THAN SIGN (<)
                // Append the current input character to the comment token's data.
                '<' => {
                    self.append_character_to_current_comment_token(c);
                }
                // Anything else
                // Reconsume in the comment state.
                _ => {
                    self.reconsume_in(State::Comment);
                }
            }
        } else {
            self.reconsume_in(State::Comment);
        }
    }

    // https://html.spec.whatwg.org/#comment-less-than-sign-bang-state
    fn comment_less_than_sign_bang_state(&mut self) {
        // Consume the next input character:

        if let Some(c) = self.consume() {
            match c {
                // U+002D HYPHEN-MINUS (-)
                // Switch to the comment less-than sign bang dash state.
                '-' => {
                    self.switch_to(State::CommentLessThanSignBangDash);
                }
                // Anything else
                // Reconsume in the comment state.
                _ => {
                    self.reconsume_in(State::Comment);
                }
            }
        } else {
            self.reconsume_in(State::Comment);
        }
    }

    // https://html.spec.whatwg.org/#comment-less-than-sign-bang-dash-state
    fn comment_less_than_sign_bang_dash_state(&mut self) {
        // Consume the next input character:

        if let Some(c) = self.consume() {
            match c {
                // U+002D HYPHEN-MINUS (-)
                // Switch to the comment less-than sign bang dash dash state.
                '-' => {
                    self.switch_to(State::CommentLessThanSignBangDashDash);
                }
                // Anything else
                // Reconsume in the commend end dash state
                _ => {
                    self.reconsume_in(State::CommentEndDash);
                }
            }
        } else {
            self.reconsume_in(State::CommentEndDash);
        }
    }

    // https://html.spec.whatwg.org/#comment-less-than-sign-bang-dash-dash-state
    fn comment_less_than_sign_bang_dash_dash_state(&mut self) {
        // Consume the next input character:

        if let Some(c) = self.consume() {
            match c {
                // U+003E GREATER-THAN SIGN (>)
                // Reconsume in the comment end state.
                '>' => {
                    self.reconsume_in(State::CommentEnd);
                }
                // Anything else
                // This is a nested-comment parse error. Reconsume in the comment end state.
                _ => {
                    self.reconsume_in(State::CommentEnd);
                }
            }
        } else {
            self.reconsume_in(State::CommentEnd);
        }
    }

    // https://html.spec.whatwg.org/#comment-end-dash-state
    fn comment_end_dash_state(&mut self) {
        // Consume the next input character:

        if let Some(c) = self.consume() {
            match c {
                // U+002D HYPHEN-MINUS (-)
                // Switch to the comment end state.
                '-' => {
                    self.switch_to(State::CommentEnd);
                }
                // Anything else
                // Append a U+002D HYPHEN-MINUS character (-) to the comment token's data. Reconsume in the comment state.
                _ => {
                    self.append_character_to_current_comment_token('-');
                    self.reconsume_in(State::Comment);
                }
            }
        }
    }

    // https://html.spec.whatwg.org/#doctype-state
    fn doctype_state(&mut self) {
        // Consume the next input character:

        if let Some(c) = self.consume() {
            match c {
                // U+0009 CHARACTER TABULATION (tab)
                // U+000A LINE FEED (LF)
                // U+000C FORM FEED (FF)
                // U+0020 SPACE
                // Switch to the before DOCTYPE name state.
                '\u{0009}' | '\u{000A}' | '\u{000C}' | ' ' => {
                    self.switch_to(State::BeforeDOCTYPEName);
                }

                // U+003E GREATER-THAN SIGN (>)
                // Reconsume in the before DOCTYPE name state.
                '>' => {
                    self.reconsume_in(State::BeforeDOCTYPEName);
                }

                // Anything else
                // This is a missing-whitespace-before-doctype-name parse error. Reconsume in the before DOCTYPE name state.
                _ => {
                    self.reconsume_in(State::BeforeDOCTYPEName);
                }
            }
        } else {
            // EOF
            // This is an eof-in-doctype parse error. Create a new DOCTYPE token. Set its force-quirks flag to on. Emit the current token. Emit an end-of-file token.
            self.create_new_doctype_token();
            self.set_current_doctype_quirks_flag_to_on();
            self.emit_current_token();
            self.emit_end_of_file_token();
        }
    }

    // https://html.spec.whatwg.org/#before-doctype-name-state
    fn before_doctype_name_state(&mut self) {
        // Consume the next input character:

        if let Some(c) = self.consume() {
            match c {
                // U+0009 CHARACTER TABULATION (tab)
                // U+000A LINE FEED (LF)
                // U+000C FORM FEED (FF)
                // U+0020 SPACE
                // Ignore the character.
                '\u{0009}' | '\u{000A}' | '\u{000C}' | ' ' => {
                    // Ignore the character.
                }

                // ASCII upper alpha
                // Create a new DOCTYPE token. Set its name to the lowercase version of the current input character (add 0x0020 to the character’s code point).
                // Switch to the DOCTYPE name state.
                'A'..='Z' => {
                    self.create_new_doctype_token();
                    self.append_character_to_current_doctype_name(c.to_ascii_lowercase());
                    self.switch_to(State::DOCTYPEName);
                }

                // U+0000 NULL
                // This is an unexpected-null-character parse error. Create a new DOCTYPE token. Set its name to a U+FFFD REPLACEMENT CHARACTER character.
                // Switch to the DOCTYPE name state.
                '\u{0000}' => {
                    self.create_new_doctype_token();
                    self.append_character_to_current_doctype_name(char::REPLACEMENT_CHARACTER);
                    self.switch_to(State::DOCTYPEName);
                }

                // U+003E GREATER-THAN SIGN (>)
                // This is a missing-doctype-name parse error. Create a new DOCTYPE token. Set its force-quirks flag to on. Switch to the data state. Emit the current token.
                '>' => {
                    self.create_new_doctype_token();
                    self.set_current_doctype_quirks_flag_to_on();
                    self.switch_to(State::Data);
                    self.emit_current_token();
                }

                // Anything else
                // Create a new DOCTYPE token. Set the token's name to the current input character. Switch to the DOCTYPE name state.
                _ => {
                    self.create_new_doctype_token();
                    self.append_character_to_current_doctype_name(c);
                    self.switch_to(State::DOCTYPEName);
                }
            }
        } else {
            // EOF
            // This is an eof-in-doctype parse error. Create a new DOCTYPE token. Set its force-quirks flag to on. Emit the current token. Emit an end-of-file token.
            self.create_new_doctype_token();
            self.set_current_doctype_quirks_flag_to_on();
            self.emit_current_token();
            self.emit_end_of_file_token();
        }
    }

    // https://html.spec.whatwg.org/#doctype-name-state
    fn doctype_name_state(&mut self) {
        // Consume teh next input character:

        if let Some(c) = self.consume() {
            match c {
                // U+0009 CHARACTER TABULATION (tab)
                // U+000A LINE FEED (LF)
                // U+000C FORM FEED (FF)
                // U+0020 SPACE
                // Switch to the after DOCTYPE name state.
                '\u{0009}' | '\u{000A}' | '\u{000C}' | ' ' => {
                    self.switch_to(State::AfterDOCTYPEName);
                }

                // U+003E GREATER-THAN SIGN (>)
                // Switch to the data state. Emit the current DOCTYPE token.
                '>' => {
                    self.switch_to(State::Data);
                    self.emit_current_token();
                }

                // ASCII upper alpha
                // Append the lowercase version of the current input character (add 0x0020 to the character’s code point) to the current DOCTYPE token's name.
                'A'..='Z' => {
                    self.append_character_to_current_doctype_name(c.to_ascii_lowercase());
                }

                // U+0000 NULL
                // This is an unexpected-null-character parse error. Append a U+FFFD REPLACEMENT CHARACTER character to the current DOCTYPE token's name.
                '\u{0000}' => {
                    self.append_character_to_current_doctype_name(char::REPLACEMENT_CHARACTER);
                }

                // Anything else
                // Append the current input character to the current DOCTYPE token's name.
                _ => {
                    self.append_character_to_current_doctype_name(c);
                }
            }
        } else {
            // EOF
            // This is an eof-in-doctype parse error. Set the current DOCTYPE token's force-quirks flag to on. Emit the current DOCTYPE token. Emit an end-of-file token.
            self.set_current_doctype_quirks_flag_to_on();
            self.emit_current_token();
            self.emit_end_of_file_token();
        }
    }

    // https://html.spec.whatwg.org/#after-doctype-name-state
    fn after_doctype_name_state(&mut self) {
        // Consume the next input character:

        if let Some(c) = self.consume() {
            match c {
                // U+0009 CHARACTER TABULATION (tab)
                // U+000A LINE FEED (LF)0
                // U+000C FORM FEED (FF)
                // U+0020 SPACE
                // Ignore the character.
                '\u{0009}' | '\u{000A}' | '\u{000C}' | ' ' => {
                    // Ignore the character.
                }

                // U+003E GREATER-THAN SIGN (>)
                // Switch to the data state. Emit the current DOCTYPE token.
                '>' => {
                    self.switch_to(State::Data);
                    self.emit_current_token();
                }

                // Anything else
                _ => {
                    match c.to_ascii_uppercase() {
                        // If the six characters starting from the current input character are an ASCII case-insensitive match for the word "PUBLIC",
                        // then consume those characters and switch to the after DOCTYPE public keyword state.
                        'P' => {
                            self.reconsume();
                            self.consume_public_keyword();
                        }

                        // If the six characters starting from the current input character are an ASCII case-insensitive match for the word "SYSTEM",
                        // then consume those characters and switch to the after DOCTYPE system keyword state.
                        'S' => {
                            self.reconsume();
                            self.consume_system_keyword();
                        }
                        // Otherwise, this is an invalid-character-sequence-after-doctype-name parse error. Set the DOCTYPE token's force-quirks flag to on.
                        // Reconsume in the bogus DOCTYPE state.
                        _ => {
                            self.set_current_doctype_quirks_flag_to_on();
                            self.reconsume_in(State::BogusDOCTYPE);
                        }
                    }
                }
            }
        }
    }

    // https://html.spec.whatwg.org/#after-doctype-public-keyword-state
    fn after_doctype_public_keyword_state(&mut self) {
        // Consume the next input character:

        if let Some(c) = self.consume() {
            match c {
                // U+0009 CHARACTER TABULATION (tab)
                // U+000A LINE FEED (LF)
                // U+000C FORM FEED (FF)
                // U+0020 SPACE
                // Switch to the before DOCTYPE public identifier state.
                '\u{0009}' | '\u{000A}' | '\u{000C}' | ' ' => {
                    self.switch_to(State::BeforeDOCTYPEPublicIdentifier);
                }

                // U+0022 QUOTATION MARK (")
                // This is a missing-whitespace-after-doctype-public-keyword parse error. Set the current DOCTYPE token's public identifier to the empty string (not missing),
                // then switch to the DOCTYPE public identifier double-quoted state.
                '"' => {
                    self.set_current_doctype_public_identifier_to_empty_string();
                    self.switch_to(State::DOCTYPEPublicIdentifierDoubleQuoted);
                }

                // U+0027 APOSTROPHE (')
                // This is a missing-whitespace-after-doctype-public-keyword parse error. Set the current DOCTYPE token's public identifier to the empty string (not missing),
                // then switch to the DOCTYPE public identifier single-quoted state.
                '\'' => {
                    self.set_current_doctype_public_identifier_to_empty_string();
                    self.switch_to(State::DOCTYPEPublicIdentifierSingleQuoted);
                }

                // U+003E GREATER-THAN SIGN (>)
                // This is a missing-doctype-public-identifier parse error. Set the current DOCTYPE token's force-quirks flag to on.
                // Switch to the data state. Emit the current DOCTYPE token.
                '>' => {
                    self.set_current_doctype_quirks_flag_to_on();
                    self.switch_to(State::Data);
                    self.emit_current_token();
                }

                // Anything else
                // This is a missing-quote-before-doctype-public-identifier parse error. Set the current DOCTYPE token's force-quirks flag to on.
                // Reconsume in the bogus DOCTYPE state.
                _ => {
                    self.set_current_doctype_quirks_flag_to_on();
                    self.reconsume_in(State::BogusDOCTYPE);
                }
            }
        } else {
            // EOF
            // This is an eof-in-doctype parse error. Set the current DOCTYPE token's force-quirks flag to on. Emit the current DOCTYPE token. Emit an end-of-file token.
            self.set_current_doctype_quirks_flag_to_on();
            self.emit_current_token();
            self.emit_end_of_file_token();
        }
    }

    // https://html.spec.whatwg.org/#after-doctype-public-keyword-state
    fn bogus_doctype_state(&mut self) {
        // Consume the next input character:

        if let Some(c) = self.consume() {
            match c {
                // U+003E GREATER-THAN SIGN (>)
                // Switch to the data state. Emit the current DOCTYPE token.
                '>' => {
                    self.switch_to(State::Data);
                    self.emit_current_token();
                }

                // U+0000 NULL
                // This is an unexpected-null-character parse error. Ignore the character.
                '\u{0000}' => {
                    // Ignore the character.
                }

                // Anything else
                // Ignore the character.
                _ => {
                    // Ignore the character.
                }
            }
        } else {
            // EOF
            // Emit the DOCTYPE token. Emit an end-of-file token.
            self.emit_current_token();
            self.emit_end_of_file_token();
        }
    }

    // https://html.spec.whatwg.org/#before-doctype-public-identifier-state
    fn before_doctype_public_identifier_state(&mut self) {
        // Consume the next input character:

        if let Some(c) = self.consume() {
            match c {
                // U+0009 CHARACTER TABULATION (tab)
                // U+000A LINE FEED (LF)
                // U+000C FORM FEED (FF)
                // U+0020 SPACE
                // Ignore the character.
                '\u{0009}' | '\u{000A}' | '\u{000C}' | ' ' => {
                    // Ignore the character.
                }

                // U+0022 QUOTATION MARK (")
                // Set the current DOCTYPE token's public identifier to the empty string (not missing), then switch to the DOCTYPE public identifier (double-quoted) state.
                '"' => {
                    self.set_current_doctype_public_identifier_to_empty_string();
                    self.switch_to(State::DOCTYPEPublicIdentifierDoubleQuoted);
                }

                // U+0027 APOSTROPHE (')
                // Set the current DOCTYPE token's public identifier to the empty string (not missing), then switch to the DOCTYPE public identifier (single-quoted) state.
                '\'' => {
                    self.set_current_doctype_public_identifier_to_empty_string();
                    self.switch_to(State::DOCTYPEPublicIdentifierSingleQuoted);
                }

                // U+003E GREATER-THAN SIGN (>)
                // This is a missing-doctype-public-identifier parse error. Set the current DOCTYPE token's force-quirks flag to on.
                '>' => {
                    self.set_current_doctype_quirks_flag_to_on();
                    self.switch_to(State::Data);
                    self.emit_current_token();
                }

                // Anything else
                // This is a missing-quote-before-doctype-public-identifier parse error. Set the current DOCTYPE token's force-quirks flag to on.
                // Reconsume in the bogus DOCTYPE state.
                _ => {
                    self.set_current_doctype_quirks_flag_to_on();
                    self.reconsume_in(State::BogusDOCTYPE);
                }
            }
        } else {
            // EOF
            // This is an eof-in-doctype parse error. Set the current DOCTYPE token's force-quirks flag to on. Emit the current DOCTYPE token. Emit an end-of-file token.
            self.set_current_doctype_quirks_flag_to_on();
            self.emit_current_token();
            self.emit_end_of_file_token();
        }
    }

    // https://html.spec.whatwg.org/#doctype-public-identifier-(double-quoted)-state
    fn doctype_public_identifier_double_quoted_state(&mut self) {
        // Consume the next input character:
        if let Some(c) = self.consume() {
            match c {
                // U+0022 QUOTATION MARK (")
                // Switch to the after DOCTYPE public identifier state.
                '"' => {
                    self.switch_to(State::AfterDOCTYPEPublicIdentifier);
                }

                // U+0000 NULL
                // This is an unexpected-null-character parse error. Append a U+FFFD REPLACEMENT CHARACTER character to the current DOCTYPE token's public identifier.
                '\u{0000}' => {
                    self.append_character_to_current_doctype_public_identifier(
                        char::REPLACEMENT_CHARACTER,
                    );
                }

                // U+003E GREATER-THAN SIGN (>)
                // This is an abrupt-doctype-public-identifier parse error. Set the current DOCTYPE token's force-quirks flag to on.
                // Switch to the data state. Emit the current DOCTYPE token.
                '>' => {
                    self.set_current_doctype_quirks_flag_to_on();
                    self.switch_to(State::Data);
                    self.emit_current_token();
                }

                // Anything else
                // Append the current input character to the current DOCTYPE token's public identifier.
                _ => {
                    self.append_character_to_current_doctype_public_identifier(c);
                }
            }
        } else {
            // EOF
            // This is an eof-in-doctype parse error. Set the current DOCTYPE token's force-quirks flag to on. Emit the current DOCTYPE token. Emit an end-of-file token.
            self.set_current_doctype_quirks_flag_to_on();
            self.emit_current_token();
            self.emit_end_of_file_token();
        }
    }

    // https://html.spec.whatwg.org/#doctype-public-identifier-(single-quoted)-state
    fn doctype_public_identifier_single_quoted_state(&mut self) {
        // Consume the next input character:
        if let Some(c) = self.consume() {
            match c {
                // U+0027 APOSTROPHE (')
                // Switch to the after DOCTYPE public identifier state.
                '\'' => {
                    self.switch_to(State::AfterDOCTYPEPublicIdentifier);
                }

                // U+0000 NULL
                // This is an unexpected-null-character parse error. Append a U+FFFD REPLACEMENT CHARACTER character to the current DOCTYPE token's public identifier.
                '\u{0000}' => {
                    self.append_character_to_current_doctype_public_identifier(
                        char::REPLACEMENT_CHARACTER,
                    );
                }

                // U+003E GREATER-THAN SIGN (>)
                // This is an abrupt-doctype-public-identifier parse error. Set the current DOCTYPE token's force-quirks flag to on.
                // Switch to the data state. Emit the current DOCTYPE token.
                '>' => {
                    self.set_current_doctype_quirks_flag_to_on();
                    self.switch_to(State::Data);
                    self.emit_current_token();
                }

                // Anything else
                // Append the current input character to the current DOCTYPE token's public identifier.
                _ => {
                    self.append_character_to_current_doctype_public_identifier(c);
                }
            }
        } else {
            // EOF
            // This is an eof-in-doctype parse error. Set the current DOCTYPE token's force-quirks flag to on. Emit the current DOCTYPE token. Emit an end-of-file token.
            self.set_current_doctype_quirks_flag_to_on();
            self.emit_current_token();
            self.emit_end_of_file_token();
        }
    }

    // https://html.spec.whatwg.org/#after-doctype-public-identifier-state
    fn after_doctype_public_identifier_state(&mut self) {
        // Consume the next input character:
        if let Some(c) = self.consume() {
            match c {
                // U+0009 CHARACTER TABULATION (tab)
                // U+000A LINE FEED (LF)
                // U+000C FORM FEED (FF)
                // U+0020 SPACE
                // Switch to the between DOCTYPE public and system identifiers state.
                '\u{0009}' | '\u{000A}' | '\u{000C}' | ' ' => {
                    self.switch_to(State::BetweenDOCTYPEPublicAndSystemIdentifiers);
                }

                // U+003E GREATER-THAN SIGN (>)
                // Switch to the data state. Emit the current DOCTYPE token.
                '>' => {
                    self.switch_to(State::Data);
                    self.emit_current_token();
                }

                // U+0022 QUOTATION MARK (")
                // This is a missing-whitespace-between-doctype-public-and-system-identifiers parse error.
                // Set the current DOCTYPE token's system identifier to the empty string (not missing),
                // then switch to the DOCTYPE system identifier (double-quoted) state.
                '"' => {
                    self.set_current_doctype_system_identifier_to_empty_string();
                    self.switch_to(State::DOCTYPESystemIdentifierDoubleQuoted);
                }

                // U+0027 APOSTROPHE (')
                // This is a missing-whitespace-between-doctype-public-and-system-identifiers parse error.
                // Set the current DOCTYPE token's system identifier to the empty string (not missing),
                // then switch to the DOCTYPE system identifier (single-quoted) state.
                '\'' => {
                    self.set_current_doctype_system_identifier_to_empty_string();
                    self.switch_to(State::DOCTYPESystemIdentifierSingleQuoted);
                }

                // Anything else
                // This is a missing-quote-before-doctype-system-identifier parse error.
                // Set the current DOCTYPE token's force-quirks flag to on.
                // Reconsume in the bogus DOCTYPE state.
                _ => {
                    self.set_current_doctype_quirks_flag_to_on();
                    self.reconsume_in(State::BogusDOCTYPE);
                }
            }
        } else {
            // EOF
            // This is an eof-in-doctype parse error. Set the current DOCTYPE token's force-quirks flag to on. Emit the current DOCTYPE token. Emit an end-of-file token.
            self.set_current_doctype_quirks_flag_to_on();
            self.emit_current_token();
            self.emit_end_of_file_token();
        }
    }

    // https://html.spec.whatwg.org/#between-doctype-public-and-system-identifiers-state
    fn between_doctype_public_and_system_identifiers_state(&mut self) {
        // Consume the next input character:

        if let Some(c) = self.consume() {
            match c {
                // U+0009 CHARACTER TABULATION (tab)
                // U+000A LINE FEED (LF)
                // U+000C FORM FEED (FF)
                // U+0020 SPACE
                // Ignore the character.
                '\u{0009}' | '\u{000A}' | '\u{000C}' | ' ' => {
                    // Ignore the character.
                }

                // U+003E GREATER-THAN SIGN (>)
                // Switch to the data state. Emit the current DOCTYPE token.
                '>' => {
                    self.switch_to(State::Data);
                    self.emit_current_token();
                }

                // U+0022 QUOTATION MARK (")
                // Set the current DOCTYPE token's system identifier to the empty string (not missing),
                // then switch to the DOCTYPE system identifier (double-quoted) state.
                '"' => {
                    self.set_current_doctype_system_identifier_to_empty_string();
                    self.switch_to(State::DOCTYPESystemIdentifierDoubleQuoted);
                }

                // U+0027 APOSTROPHE (')
                // Set the current DOCTYPE token's system identifier to the empty string (not missing),
                // then switch to the DOCTYPE system identifier (single-quoted) state.
                '\'' => {
                    self.set_current_doctype_system_identifier_to_empty_string();
                    self.switch_to(State::DOCTYPESystemIdentifierSingleQuoted);
                }

                // Anything else
                // This is a missing-quote-before-doctype-system-identifier parse error.
                // Set the current DOCTYPE token's force-quirks flag to on.
                // Reconsume in the bogus DOCTYPE state.
                _ => {
                    self.set_current_doctype_quirks_flag_to_on();
                    self.reconsume_in(State::BogusDOCTYPE);
                }
            }
        } else {
            // EOF
            // This is an eof-in-doctype parse error. Set the current DOCTYPE token's force-quirks flag to on. Emit the current DOCTYPE token. Emit an end-of-file token.
            self.set_current_doctype_quirks_flag_to_on();
            self.emit_current_token();
            self.emit_end_of_file_token();
        }
    }

    // https://html.spec.whatwg.org/#after-doctype-system-keyword-state
    fn after_doctype_system_keyword_state(&mut self) {
        // Consume the next input character:

        if let Some(c) = self.consume() {
            match c {
                // U+0009 CHARACTER TABULATION (tab)
                // U+000A LINE FEED (LF)
                // U+000C FORM FEED (FF)
                // U+0020 SPACE
                // Switch to the before DOCTYPE system identifier state.
                '\u{0009}' | '\u{000A}' | '\u{000C}' | ' ' => {
                    self.switch_to(State::BeforeDOCTYPESystemIdentifier);
                }

                // U+0022 QUOTATION MARK (")
                // This is a missing-whitespace-after-doctype-system-keyword parse error.
                // Set the current DOCTYPE token's system identifier to the empty string (not missing),
                // then switch to the DOCTYPE system identifier (double-quoted) state.
                '"' => {
                    self.set_current_doctype_system_identifier_to_empty_string();
                    self.switch_to(State::DOCTYPESystemIdentifierDoubleQuoted);
                }

                // U+0027 APOSTROPHE (')
                // This is a missing-whitespace-after-doctype-system-keyword parse error.
                // Set the current DOCTYPE token's system identifier to the empty string (not missing),
                // then switch to the DOCTYPE system identifier (single-quoted) state.
                '\'' => {
                    self.set_current_doctype_system_identifier_to_empty_string();
                    self.switch_to(State::DOCTYPESystemIdentifierSingleQuoted);
                }

                // U+003E GREATER-THAN SIGN (>)
                // This is a missing-doctype-system-identifier parse error.
                // Set the current DOCTYPE token's force-quirks flag to on.
                // Switch to the data state. Emit the current DOCTYPE token.
                '>' => {
                    self.set_current_doctype_quirks_flag_to_on();
                    self.switch_to(State::Data);
                    self.emit_current_token();
                }

                // Anything else
                // This is a missing-quote-before-doctype-system-identifier parse error.
                // Set the current DOCTYPE token's force-quirks flag to on.
                // Reconsume in the bogus DOCTYPE state.
                _ => {
                    self.set_current_doctype_quirks_flag_to_on();
                    self.reconsume_in(State::BogusDOCTYPE);
                }
            }
        } else {
            // EOF
            // This is an eof-in-doctype parse error. Set the current DOCTYPE token's force-quirks flag to on. Emit the current DOCTYPE token. Emit an end-of-file token.
            self.set_current_doctype_quirks_flag_to_on();
            self.emit_current_token();
            self.emit_end_of_file_token();
        }
    }

    // https://html.spec.whatwg.org/#before-doctype-system-identifier-state
    fn before_doctype_system_identifier_state(&mut self) {
        // Consume the next input character:
        if let Some(c) = self.consume() {
            match c {
                // U+0009 CHARACTER TABULATION (tab)
                // U+000A LINE FEED (LF)
                // U+000C FORM FEED (FF)
                // U+0020 SPACE
                // Ignore the character.
                '\u{0009}' | '\u{000A}' | '\u{000C}' | ' ' => {
                    // Ignore the character.
                }

                // U+0022 QUOTATION MARK (")
                // Set the current DOCTYPE token's system identifier to the empty string (not missing),
                // then switch to the DOCTYPE system identifier (double-quoted) state.
                '"' => {
                    self.set_current_doctype_system_identifier_to_empty_string();
                    self.switch_to(State::DOCTYPESystemIdentifierDoubleQuoted);
                }

                // U+0027 APOSTROPHE (')
                // Set the current DOCTYPE token's system identifier to the empty string (not missing),
                // then switch to the DOCTYPE system identifier (single-quoted) state.
                '\'' => {
                    self.set_current_doctype_system_identifier_to_empty_string();
                    self.switch_to(State::DOCTYPESystemIdentifierSingleQuoted);
                }

                // U+003E GREATER-THAN SIGN (>)
                // This is a missing-doctype-system-identifier parse error.
                // Set the current DOCTYPE token's force-quirks flag to on.
                // Switch to the data state. Emit the current DOCTYPE token.
                '>' => {
                    self.set_current_doctype_quirks_flag_to_on();
                    self.switch_to(State::Data);
                    self.emit_current_token();
                }

                // Anything else
                // This is a missing-quote-before-doctype-system-identifier parse error.
                // Set the current DOCTYPE token's force-quirks flag to on.
                // Reconsume in the bogus DOCTYPE state.
                _ => {
                    self.set_current_doctype_quirks_flag_to_on();
                    self.reconsume_in(State::BogusDOCTYPE);
                }
            }
        } else {
            // EOF
            // This is an eof-in-doctype parse error. Set the current DOCTYPE token's force-quirks flag to on. Emit the current DOCTYPE token. Emit an end-of-file token.
            self.set_current_doctype_quirks_flag_to_on();
            self.emit_current_token();
            self.emit_end_of_file_token();
        }
    }

    // https://html.spec.whatwg.org/#doctype-system-identifier-(double-quoted)-state
    fn doctype_system_identifier_double_quoted_state(&mut self) {
        // Consume the next input character:
        if let Some(c) = self.consume() {
            match c {
                // U+0022 QUOTATION MARK (")
                // Switch to the after DOCTYPE system identifier state.
                '"' => {
                    self.switch_to(State::AfterDOCTYPESystemIdentifier);
                }

                // U+0000 NULL
                // This is an unexpected-null-character parse error. Append a U+FFFD REPLACEMENT CHARACTER character to the current DOCTYPE token's system identifier.
                '\u{0000}' => {
                    self.append_character_to_current_doctype_system_identifier(
                        char::REPLACEMENT_CHARACTER,
                    );
                }

                // U+003E GREATER-THAN SIGN (>)
                // This is an abrupt-doctype-system-identifier parse error. Set the current DOCTYPE token's force-quirks flag to on.
                // Switch to the data state. Emit the current DOCTYPE token.
                '>' => {
                    self.set_current_doctype_quirks_flag_to_on();
                    self.switch_to(State::Data);
                    self.emit_current_token();
                }

                // Anything else
                // Append the current input character to the current DOCTYPE token's system identifier.
                _ => {
                    self.append_character_to_current_doctype_system_identifier(c);
                }
            }
        } else {
            // EOF
            // This is an eof-in-doctype parse error. Set the current DOCTYPE token's force-quirks flag to on. Emit the current DOCTYPE token. Emit an end-of-file token.
            self.set_current_doctype_quirks_flag_to_on();
            self.emit_current_token();
            self.emit_end_of_file_token();
        }
    }

    // https://html.spec.whatwg.org/#doctype-system-identifier-(single-quoted)-state
    fn doctype_system_identifier_single_quoted_state(&mut self) {
        // Consume the next input character:
        if let Some(c) = self.consume() {
            match c {
                // U+0027 APOSTROPHE (')
                // Switch to the after DOCTYPE system identifier state.
                '\'' => {
                    self.switch_to(State::AfterDOCTYPESystemIdentifier);
                }

                // U+0000 NULL
                // This is an unexpected-null-character parse error. Append a U+FFFD REPLACEMENT CHARACTER character to the current DOCTYPE token's system identifier.
                '\u{0000}' => {
                    self.append_character_to_current_doctype_system_identifier(
                        char::REPLACEMENT_CHARACTER,
                    );
                }

                // U+003E GREATER-THAN SIGN (>)
                // This is an abrupt-doctype-system-identifier parse error. Set the current DOCTYPE token's force-quirks flag to on.
                // Switch to the data state. Emit the current DOCTYPE token.
                '>' => {
                    self.set_current_doctype_quirks_flag_to_on();
                    self.switch_to(State::Data);
                    self.emit_current_token();
                }

                // Anything else
                // Append the current input character to the current DOCTYPE token's system identifier.
                _ => {
                    self.append_character_to_current_doctype_system_identifier(c);
                }
            }
        } else {
            // EOF
            // This is an eof-in-doctype parse error. Set the current DOCTYPE token's force-quirks flag to on. Emit the current DOCTYPE token. Emit an end-of-file token.
            self.set_current_doctype_quirks_flag_to_on();
            self.emit_current_token();
            self.emit_end_of_file_token();
        }
    }

    // https://html.spec.whatwg.org/#after-doctype-system-identifier-state
    fn after_doctype_system_identifier_state(&mut self) {
        // Consume the next input character:
        if let Some(c) = self.consume() {
            match c {
                // U+0009 CHARACTER TABULATION (tab)
                // U+000A LINE FEED (LF)
                // U+000C FORM FEED (FF)
                // U+0020 SPACE
                // Ignore the character.
                '\u{0009}' | '\u{000A}' | '\u{000C}' | ' ' => {
                    // Ignore the character.
                }

                // U+003E GREATER-THAN SIGN (>)
                // Switch to the data state. Emit the current DOCTYPE token.
                '>' => {
                    self.switch_to(State::Data);
                    self.emit_current_token();
                }

                // Anything else
                // This is a missing-quote-after-doctype-system-identifier parse error.
                // Reconsume in the bogus DOCTYPE state.
                _ => {
                    self.reconsume_in(State::BogusDOCTYPE);
                }
            }
        } else {
            // EOF
            // This is an eof-in-doctype parse error. Emit the current DOCTYPE token. Emit an end-of-file token.
            self.emit_current_token();
            self.emit_end_of_file_token();
        }
    }

    // https://html.spec.whatwg.org/#cdata-section-state
    fn cdata_section_state(&mut self) {
        // Consume the next input character:

        if let Some(c) = self.consume() {
            match c {
                // U+005D RIGHT SQUARE BRACKET (])
                // Switch to the CDATA section bracket state.
                ']' => {
                    self.switch_to(State::CDATASectionBracket);
                }

                // Anything else
                // Emit the current input character as a character token.
                _ => {
                    self.emit_character_token(c);
                } // NOTE:
                  // U+0000 NULL characters are handled in the tree construction stage, as part of the in foreign content insertion mode, which is the only place where
                  // CDATA sections can appear.
            }
        } else {
            // EOF
            // Emit an end-of-file token.
            self.emit_end_of_file_token();
        }
    }

    // https://html.spec.whatwg.org/#cdata-section-bracket-state
    fn cdata_section_bracket_state(&mut self) {
        // Consume the next input character:
        if let Some(c) = self.consume() {
            match c {
                // U+005D RIGHT SQUARE BRACKET (])
                // Switch to the CDATA section end state.
                ']' => {
                    self.switch_to(State::CDATASectionEnd);
                }

                // Anything else
                // Emit a U+005D RIGHT SQUARE BRACKET character token. Reconsume in the CDATA section state.
                _ => {
                    self.emit_character_token(']');
                    self.reconsume_in(State::CDATASection);
                }
            }
        } else {
            // EOF
            // Emit a U+005D RIGHT SQUARE BRACKET character token. Reconsume in the CDATA section state.
            self.emit_character_token(']');
            self.reconsume_in(State::CDATASection);
        }
    }

    // https://html.spec.whatwg.org/#cdata-section-end-state
    fn cdata_section_end_state(&mut self) {
        // Consume the next input character:

        if let Some(c) = self.consume() {
            match c {
                // U+005D RIGHT SQUARE BRACKET (])
                // Emit a U+005D RIGHT SQUARE BRACKET character token.
                ']' => {
                    self.emit_character_token(']');
                }

                // U+003E GREATER-THAN SIGN (>)
                // Switch to the data state.
                '>' => {
                    self.switch_to(State::Data);
                }

                // Anything else
                // Emit two U+005D RIGHT SQUARE BRACKET character tokens. Reconsume in the CDATA section state.
                _ => {
                    self.emit_character_token(']');
                    self.emit_character_token(']');
                    self.reconsume_in(State::CDATASection);
                }
            }
        } else {
            // EOF
            // Emit two U+005D RIGHT SQUARE BRACKET character tokens. Emit an end-of-file token.
            self.emit_character_token(']');
            self.emit_character_token(']');
            self.emit_end_of_file_token();
        }
    }

    fn consume_system_keyword(&mut self) {
        let goal = vec!['S', 'Y', 'S', 'T', 'E', 'M'];

        let mut index = 0;

        while index < goal.len() {
            if let Some(c) = self.consume() {
                if c.to_ascii_uppercase() == goal[index] {
                    index += 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        if index == goal.len() {
            self.switch_to(State::AfterDOCTYPESystemKeyword);
        } else {
            self.reconsume_in(State::BogusDOCTYPE);
        }
    }

    fn consume_public_keyword(&mut self) {
        let goal = vec!['P', 'U', 'B', 'L', 'I', 'C'];

        let mut index = 0;

        while index < goal.len() {
            if let Some(c) = self.consume() {
                if c.to_ascii_uppercase() == goal[index] {
                    index += 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        if index == goal.len() {
            self.switch_to(State::AfterDOCTYPEPublicKeyword);
        } else {
            self.reconsume_in(State::BogusDOCTYPE);
        }
    }

    fn consume_double_hyphen(&mut self) {
        let mut hypen_count = 0;

        while hypen_count < 2 {
            if let Some(c) = self.consume() {
                if c == '-' {
                    hypen_count += 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        // Consume those two characters, create a comment token whose data is the empty string, and switch to the comment start state.
        if hypen_count == 2 {
            self.create_new_comment_token();

            self.switch_to(State::CommentStart);
        } else {
            self.create_new_comment_token();

            for _ in 0..hypen_count {
                self.append_character_to_current_comment_token('-');
            }

            self.reconsume_in(State::BogusComment);
        }
    }

    fn consume_doctype(&mut self) {
        let goal = vec!['D', 'O', 'C', 'T', 'Y', 'P', 'E'];

        let mut index = 0;

        while index < goal.len() {
            if let Some(c) = self.consume() {
                if c.to_ascii_uppercase() == goal[index] {
                    index += 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        if index == goal.len() {
            // Switch to the DOCTYPE state.
            self.switch_to(State::DOCTYPE);
        } else {
            self.create_new_comment_token();

            for c in 0..index {
                self.append_character_to_current_comment_token(goal[c]);
            }

            self.reconsume_in(State::BogusComment);
        }
    }

    fn consume_cdata(&mut self) {
        let goal = vec!['[', 'C', 'D', 'A', 'T', 'A', '['];

        let mut index = 0;

        while index < goal.len() {
            if let Some(c) = self.consume() {
                if c == goal[index] {
                    index += 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        if index == goal.len() {
            // TODO: If there is an adjusted current node and it is not an element in the HTML namespace, then switch to the CDATA section state.
            // Otherwise, this is a cdata-in-html-content parse error. Create a comment token whose data is the "[CDATA[" string. Switch to the bogus comment state.

            // Switch to the CDATA section state.
            self.switch_to(State::CDATASection);
        } else {
            self.create_new_comment_token();

            for c in 0..index {
                self.append_character_to_current_comment_token(goal[c]);
            }

            self.reconsume_in(State::BogusComment);
        }
    }

    fn set_character_reference_code_to_zero(&mut self) {
        self.character_reference_code = 0;
    }

    fn get_return_state(&self) -> State {
        self.return_state
    }

    fn switch_to_return_state(&mut self) {
        self.switch_to(self.return_state);
    }

    fn reconsume(&mut self) {
        self.reconsume = true;
    }

    fn reconsume_in_return_state(&mut self) {
        self.reconsume_in(self.return_state);
    }

    fn set_temporary_buffer_to_empty_string(&mut self) {
        self.temporary_buffer = Some(String::new());
    }

    fn get_temporary_buffer(&self) -> String {
        match self.temporary_buffer {
            Some(ref buffer) => buffer.clone(),
            None => String::new(),
        }
    }

    fn append_character_to_temporary_buffer(&mut self, c: char) {
        match self.temporary_buffer {
            Some(ref mut buffer) => buffer.push(c),
            None => {
                self.temporary_buffer = Some(String::from(c));
            }
        }
    }

    fn append_character_to_current_tag_token(&mut self, c: char) {
        if let Some(token) = &mut self.current_token {
            if let Token::Tag(tag) = token {
                tag.append_character_to_name(c);
            }
        }
    }

    fn append_character_to_attribute_name(&mut self, c: char) {
        if let Some(token) = &mut self.current_token {
            if let Token::Tag(tag) = token {
                tag.append_character_to_attribute_name(c);
            }
        }
    }

    fn append_character_to_attribute_value(&mut self, c: char) {
        if let Some(token) = &mut self.current_token {
            if let Token::Tag(tag) = token {
                tag.append_character_to_attribute_value(c);
            }
        }
    }

    fn append_character_to_current_comment_token(&mut self, c: char) {
        if let Some(token) = &mut self.current_token {
            if let Token::Comment(comment) = token {
                comment.push(c);
            }
        }
    }

    fn append_character_to_current_doctype_name(&mut self, c: char) {
        if let Some(token) = &mut self.current_token {
            if let Token::DOCTYPE(doctype) = token {
                doctype.append_character_to_name(c);
            }
        }
    }

    fn append_character_to_current_doctype_public_identifier(&mut self, c: char) {
        if let Some(token) = &mut self.current_token {
            if let Token::DOCTYPE(doctype) = token {
                doctype.append_character_to_public_identifier(c);
            }
        }
    }

    fn append_character_to_current_doctype_system_identifier(&mut self, c: char) {
        if let Some(token) = &mut self.current_token {
            if let Token::DOCTYPE(doctype) = token {
                doctype.append_character_to_system_identifier(c);
            }
        }
    }

    fn flush_temporary_buffer(&mut self) -> Option<String> {
        std::mem::replace(&mut self.temporary_buffer, None)
    }

    fn flush_code_points_consumed_as_a_character_reference(&mut self) {
        let temporary_buffer = self.flush_temporary_buffer();

        if let Some(buffer) = temporary_buffer {
            for c in buffer.chars() {
                if self.is_in_attribute_value() {
                    self.append_character_to_attribute_value(c);
                } else {
                    self.tokens.push_back(Token::Char(c));
                }
            }
        }
    }

    fn create_new_comment_token(&mut self) {
        self.current_token = Some(Token::new_comment());
    }

    fn create_new_doctype_token(&mut self) {
        self.current_token = Some(Token::new_doctype());
    }

    fn set_current_doctype_quirks_flag_to_on(&mut self) {
        if let Some(token) = &mut self.current_token {
            if let Token::DOCTYPE(doctype) = token {
                doctype.set_quirks_flag_to_on();
            }
        }
    }

    fn set_current_doctype_public_identifier_to_empty_string(&mut self) {
        if let Some(token) = &mut self.current_token {
            if let Token::DOCTYPE(doctype) = token {
                doctype.set_public_identifier_to_empty_string();
            }
        }
    }

    fn set_current_doctype_system_identifier_to_empty_string(&mut self) {
        if let Some(token) = &mut self.current_token {
            if let Token::DOCTYPE(doctype) = token {
                doctype.set_system_identifier_to_empty_string();
            }
        }
    }

    fn emit_current_input_character(&mut self) {
        match self.current_character.take() {
            Some(c) => self.emit_character_token(c),
            None => {}
        }
    }

    fn emit_end_of_file_token(&mut self) {
        self.tokens.push_back(Token::EOF);
    }

    fn emit_character_token(&mut self, c: char) {
        self.tokens.push_back(Token::Char(c));
    }

    fn create_new_start_tag_token(&mut self) {
        self.current_token = Some(Token::new_start_tag());
    }

    fn create_new_end_tag_token(&mut self) {
        self.current_token = Some(Token::new_end_tag());
    }

    fn emit_current_token(&mut self) {
        self.emit_current_tag_token();
    }

    fn emit_current_comment_token(&mut self) {
        self.emit_current_tag_token();
    }

    fn emit_current_tag_token(&mut self) {
        match self.current_token.take() {
            Some(token) => self.tokens.push_back(token),
            None => {}
        }
    }

    fn start_a_new_attribute(&mut self) {
        if let Some(token) = &mut self.current_token {
            if let Token::Tag(tag) = token {
                tag.new_attribute();
            }
        }
    }

    fn is_in_attribute_value(&mut self) -> bool {
        match self.return_state {
            State::AttributeValueUnquoted
            | State::AttributeValueDoubleQuoted
            | State::AttributeValueSingleQuoted => true,
            _ => false,
        }
    }

    fn consume(&mut self) -> Option<char> {
        if self.reconsume {
            self.reconsume = false;
            self.current_character
        } else {
            self.current_character = self.html.next();
            self.current_character
        }
    }

    fn reconsume_in(&mut self, state: State) {
        self.reconsume = true;
        self.switch_to(state);
    }

    fn set_return_state(&mut self, state: State) {
        self.return_state = state;
    }

    fn switch_to(&mut self, state: State) {
        self.current_state = state;
    }
}

#[cfg(test)]
mod tests {
    use crate::Scanner;

    #[test]
    fn test_basic_html() {
        let test = "<!--Hello World-->";
        let scanner = Scanner::new(test);

        println!("{:?}", scanner.tokens);
    }

    #[test]
    fn test_comment_with_less_than_sign() {
        let test = "<!--Hello < World-->";
        let scanner = Scanner::new(test);

        println!("{:?}", scanner.tokens);
    }

    #[test]
    fn test_comment_with_ampersand() {
        let test = "<!--Hello & World-->";
        let scanner = Scanner::new(test);

        println!("{:?}", scanner.tokens);
    }
}