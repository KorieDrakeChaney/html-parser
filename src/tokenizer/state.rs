// https://html.spec.whatwg.org/
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum State {
    // The Data State
    // Consume the next input character:
    // U+0026 AMPERSAND (&)
    //     Switch to the character reference in data state.
    // U+003C LESS-THAN SIGN (<)
    //     Switch to the tag open state.
    // U+0000 NULL
    //     Parse error. Emit the current input character as a character token.
    // EOF
    //     Emit an end-of-file token.
    // Anything else
    Data,

    // The Replaceable Character Data (RCDATA) state
    // Consume the next input character:
    // U+0026 AMPERSAND (&)
    //     Set the return state to the RCDATA state. Switch to the character reference state.
    // U+003C LESS-THAN SIGN (<)
    //     Switch to the RCDATA less-than sign state.
    // EOF
    //     Emit an end-of-file token.
    // Anything else
    //     Emit the current input character as a character token.
    // https://html.spec.whatwg.org/#rcdata-state
    RCDATA,

    // The RAWTEXT state
    // Consume the next input character:
    // U+003C LESS-THAN SIGN (<)
    //     Switch to the RAWTEXT less-than sign state.
    // U+0000 NULL
    //     This is an unexpected-null-character parse erroe. Emit a U+FFFD REPLACEMENT CHARACTER character token.
    // EOF
    //     Emit an end-of-file token.
    // Anything else
    //     Emit the current input character as a character token.
    // https://html.spec.whatwg.org/#rawtext-state
    RAWTEXT,

    // The Script data state
    // Consume the next input character:
    // U+003C LESS-THAN SIGN (<)
    //     Switch to the script data less-than sign state.
    // U+0000 NULL
    //     This is an unexpected-null-character parse erroe. Emit a U+FFFD REPLACEMENT CHARACTER character token.
    // EOF
    //     Emit an end-of-file token.
    // Anything else
    //     Emit the current input character as a character token.
    // https://html.spec.whatwg.org/#script-data-state
    Script,

    // The PLAINTEXT state
    // Consume the next input character:
    // U+0000 NULL
    //     This is an unexpected-null-character parse erroe. Emit a U+FFFD REPLACEMENT CHARACTER character token.
    // EOF
    //     Emit an end-of-file token.
    // Anything else
    //     Emit the current input character as a character token.
    // https://html.spec.whatwg.org/#plaintext-state
    PLAINTEXT,

    // The Tag open state
    // Consume the next input character:
    // U+0021 EXCLAMATION MARK (!)
    //     Switch to the markup declaration open state.
    // U+002F SOLIDUS (/)
    //     Switch to the end tag open state.
    // ASCII alpha
    //     Create a new start tag token, set its tag name to the empty string. Reconsume in the tag name state.
    // U+003F QUESTION MARK (?)
    //     This is an unexpected-question-mark-instead-of-tag-name parse error. Create a comment token whose data is the empty string. Reconsume in the bogus comment state.
    // EOF
    //     This is an eof-before-tag-name parse error. Emit a U+003C LESS-THAN SIGN character token and an end-of-file token.
    // Anything else
    //     This is an unexpected-solidus-in-tag parse error. Emit a U+003C LESS-THAN SIGN character token. Reconsume in the data state.
    // https://html.spec.whatwg.org/#tag-open-state
    TagOpen,

    // The End tag open state
    // Consume the next input character:
    // ASCII alpha
    //     Create a new end tag token, set its tag name to the empty string. Reconsume in the tag name state.
    // U+003E GREATER-THAN SIGN (>)
    //     This is a missing-end-tag-name parse error. Switch to the data state.
    // EOF
    //     This is an eof-before-tag-name parse error. Emit a U+003C LESS-THAN SIGN character token, a U+002F SOLIDUS character token, and an end-of-file token.
    // Anything else
    //     This is an invalid-first-character-of-tag-name parse error. Create a comment token whose data is the empty string. Reconsume in the bogus comment state.
    // https://html.spec.whatwg.org/#end-tag-open-state
    EndTagOpen,

    // The Tag name state
    // Consume the next input character:
    // U+0009 CHARACTER TABULATION (tab)
    // U+000A LINE FEED (LF)
    // U+000C FORM FEED (FF)
    // U+0020 SPACE
    //     Switch to the before attribute name state.
    // U+002F SOLIDUS (/)
    //     Switch to the self-closing start tag state.
    // U+003E GREATER-THAN SIGN (>)
    //     Switch to the data state. Emit the current tag token.
    // ASCII upper alpha
    //     Append the lowercase version of the current input character (add 0x0020 to the character's code point) to the current tag token's tag name. Append the current input character to the temporary buffer.
    // U+0000 NULL
    //     This is an unexpected-null-character parse error. Append a U+FFFD REPLACEMENT CHARACTER character to the current tag token's tag name. Append a U+FFFD REPLACEMENT CHARACTER character to the temporary buffer.
    // EOF
    //     This is an eof-in-tag parse error. Emit an end-of-file token.
    // Anything else
    //     Append the current input character to the current tag token's tag name. Append the current input character to the temporary buffer.
    // https://html.spec.whatwg.org/#tag-name-state
    TagName,

    // The RCDATA less-than sign state
    // Consume the next input character:
    // U+002F SOLIDUS (/)
    //     Set the temporary buffer to the empty string. Switch to the RCDATA end tag open state.
    // Anything else
    //     Emit a U+003C LESS-THAN SIGN character token. Reconsume in the RCDATA state.
    // https://html.spec.whatwg.org/#rcdata-less-than-sign-state
    RCDATALessThanSign,

    // The RCDATA end tag open state
    // Consume the next input character:
    // ASCII alpha
    //     Create a new end tag token, set its tag name to the empty string. Reconsume in the RCDATA end tag name state.
    // Anything else
    //     Emit a U+003C LESS-THAN SIGN character token and a U+002F SOLIDUS character token. Reconsume in the RCDATA state.
    // https://html.spec.whatwg.org/#rcdata-end-tag-open-state
    RCDATAEndTagOpen,

    // The RCDATA end tag name state
    // Consume the next input character:
    // U+0009 CHARACTER TABULATION (tab)
    // U+000A LINE FEED (LF)
    // U+000C FORM FEED (FF)
    // U+0020 SPACE
    //     If the current end tag token is an appropriate end tag token, then switch to the before attribute name state. Otherwise, treat it as per the "anything else" entry below.
    // U+002F SOLIDUS (/)
    //     If the current end tag token is an appropriate end tag token, then switch to the self-closing start tag state. Otherwise, treat it as per the "anything else" entry below.
    // U+003E GREATER-THAN SIGN (>)
    //     If the current end tag token is an appropriate end tag token, then switch to the data state and emit the current tag token. Otherwise, treat it as per the "anything else" entry below.
    // ASCII upper alpha
    //     Append the lowercase version of the current input character (add 0x0020 to the character's code point) to the current tag token's tag name. Append the current input character to the temporary buffer.
    // ASCII lower alpha
    //     Append the current input character to the current tag token's tag name. Append the current input character to the temporary buffer.
    // Anything else
    //     Emit a U+003C LESS-THAN SIGN character token, a U+002F SOLIDUS character token, and a character token for each of the characters in the temporary buffer (in the order they were added to the buffer). Reconsume in the RCDATA state.
    // https://html.spec.whatwg.org/#rcdata-end-tag-name-state
    RCDATAEndTagName,

    // The RAWTEXT less-than sign state
    // Consume the next input character:
    // U+002F SOLIDUS (/)
    //     Set the temporary buffer to the empty string. Switch to the RAWTEXT end tag open state.
    // Anything else
    //     Emit a U+003C LESS-THAN SIGN character token. Reconsume in the RAWTEXT state.
    // https://html.spec.whatwg.org/#rawtext-less-than-sign-state
    RAWTEXTLessThanSign,

    // The RAWTEXT end tag open state
    // Consume the next input character:
    // ASCII alpha
    //     Create a new end tag token, set its tag name to the empty string. Reconsume in the RAWTEXT end tag name state.
    // Anything else
    //     Emit a U+003C LESS-THAN SIGN character token and a U+002F SOLIDUS character token. Reconsume in the RAWTEXT state.
    // https://html.spec.whatwg.org/#rawtext-end-tag-open-state
    RAWTEXTEndTagOpen,

    // The RAWTEXT end tag name state
    // Consume the next input character:
    // U+0009 CHARACTER TABULATION (tab)
    // U+000A LINE FEED (LF)
    // U+000C FORM FEED (FF)
    // U+0020 SPACE
    //     If the current end tag token is an appropriate end tag token, then switch to the before attribute name state. Otherwise, treat it as per the "anything else" entry below.
    // U+002F SOLIDUS (/)
    //     If the current end tag token is an appropriate end tag token, then switch to the self-closing start tag state. Otherwise, treat it as per the "anything else" entry below.
    // U+003E GREATER-THAN SIGN (>)
    //     If the current end tag token is an appropriate end tag token, then switch to the data state and emit the current tag token. Otherwise, treat it as per the "anything else" entry below.
    // ASCII upper alpha
    //     Append the lowercase version of the current input character (add 0x0020 to the character's code point) to the current tag token's tag name. Append the current input character to the temporary buffer.
    // ASCII lower alpha
    //     Append the current input character to the current tag token's tag name. Append the current input character to the temporary buffer.
    // Anything else
    //     Emit a U+003C LESS-THAN SIGN character token, a U+002F SOLIDUS character token, and a character token for each of the characters in the temporary buffer (in the order they were added to the buffer). Reconsume in the RAWTEXT state.
    // https://html.spec.whatwg.org/#rawtext-end-tag-name-state
    RAWTEXTEndTagName,

    // The Script data less-than sign state
    // Consume the next input character:
    // U+002F SOLIDUS (/)
    //     Set the temporary buffer to the empty string. Switch to the script data end tag open state.
    // U+0021 EXCLAMATION MARK (!)
    //     Switch to the script data escape start state. Emit a U+003C LESS-THAN SIGN character token and a U+0021 EXCLAMATION MARK character token.
    // Anything else
    //     Emit a U+003C LESS-THAN SIGN character token. Reconsume in the script data state.
    // https://html.spec.whatwg.org/#script-data-less-than-sign-state
    SriptDataLessThanSign,

    // The Script data end tag open state
    // Consume the next input character:
    // ASCII alpha
    //     Create a new end tag token, set its tag name to the empty string. Reconsume in the script data end tag name state.
    // Anything else
    //     Emit a U+003C LESS-THAN SIGN character token and a U+002F SOLIDUS character token. Reconsume in the script data state.
    // https://html.spec.whatwg.org/#script-data-end-tag-open-state
    ScriptDataEndTagOpen,

    // The Script data end tag name state
    // Consume the next input character:
    // U+0009 CHARACTER TABULATION (tab)
    // U+000A LINE FEED (LF)
    // U+000C FORM FEED (FF)
    // U+0020 SPACE
    //     If the current end tag token is an appropriate end tag token, then switch to the before attribute name state. Otherwise, treat it as per the "anything else" entry below.
    // U+002F SOLIDUS (/)
    //     If the current end tag token is an appropriate end tag token, then switch to the self-closing start tag state. Otherwise, treat it as per the "anything else" entry below.
    // U+003E GREATER-THAN SIGN (>)
    //     If the current end tag token is an appropriate end tag token, then switch to the data state and emit the current tag token. Otherwise, treat it as per the "anything else" entry below.
    // ASCII upper alpha
    //     Append the lowercase version of the current input character (add 0x0020 to the character's code point) to the current tag token's tag name. Append the current input character to the temporary buffer.
    // ASCII lower alpha
    //     Append the current input character to the current tag token's tag name. Append the current input character to the temporary buffer.
    // Anything else
    //     Emit a U+003C LESS-THAN SIGN character token, a U+002F SOLIDUS character token, and a character token for each of the characters in the temporary buffer (in the order they were added to the buffer). Reconsume in the script data state.
    // https://html.spec.whatwg.org/#script-data-end-tag-name-state
    ScriptDataEndTagName,

    // The Script data escape start state
    // Consume the next input character:
    // U+002D HYPHEN-MINUS (-)
    //     Switch to the script data escape start dash state. Emit a U+002D HYPHEN-MINUS character token.
    // Anything else
    //     Reconsume in the script data state.
    // https://html.spec.whatwg.org/#script-data-escape-start-state
    ScriptDataEscapeStart,

    // The Script data escape start dash state
    // Consume the next input character:
    // U+002D HYPHEN-MINUS (-)
    //     Switch to the script data escaped dash dash state. Emit a U+002D HYPHEN-MINUS character token.
    // Anything else
    //     Reconsume in the script data state.
    // https://html.spec.whatwg.org/#script-data-escape-start-dash-state
    ScriptDataEscapeStartDash,

    // The Script data escaped state
    // Consume the next input character:
    // U+002D HYPHEN-MINUS (-)
    //     Switch to the script data escaped dash state. Emit a U+002D HYPHEN-MINUS character token.
    // U+003C LESS-THAN SIGN (<)
    //     Switch to the script data escaped less-than sign state.
    // U+0000 NULL
    //     This is an unexpected-null-character parse error. Emit a U+FFFD REPLACEMENT CHARACTER character token.
    // EOF
    //     This is an eof-in-script-html-comment-like-text parse error. Emit an end-of-file token.
    // Anything else
    //     Emit the current input character as a character token.
    // https://html.spec.whatwg.org/#script-data-escaped-state
    ScriptDataEscaped,

    // The Script data escaped dash state
    // Consume the next input character:
    // U+002D HYPHEN-MINUS (-)
    //     Switch to the script data escaped dash dash state. Emit a U+002D HYPHEN-MINUS character token.
    // U+003C LESS-THAN SIGN (<)
    //     Switch to the script data escaped less-than sign state.
    // U+0000 NULL
    //     This is an unexpected-null-character parse error. Emit a U+FFFD REPLACEMENT CHARACTER character token.
    // EOF
    //     This is an eof-in-script-html-comment-like-text parse error. Emit an end-of-file token.
    // Anything else
    //     Switch to the script data escaped state. Reconsume in that state.
    // https://html.spec.whatwg.org/#script-data-escaped-dash-state
    ScriptDataEscapedDash,

    // The Script data escaped dash dash state
    // Consume the next input character:
    // U+002D HYPHEN-MINUS (-)
    //     Emit a U+002D HYPHEN-MINUS character token.
    // U+003C LESS-THAN SIGN (<)
    //     Switch to the script data escaped less-than sign state.
    // U+003E GREATER-THAN SIGN (>)
    //     Switch to the script data state. Emit a U+003E GREATER-THAN SIGN character token.
    // U+0000 NULL
    //     This is an unexpected-null-character parse error. Emit a U+FFFD REPLACEMENT CHARACTER character token.
    // EOF
    //     This is an eof-in-script-html-comment-like-text parse error. Emit an end-of-file token.
    // Anything else
    //     Switch to the script data escaped state. Reconsume in that state.
    // https://html.spec.whatwg.org/#script-data-escaped-dash-dash-state
    ScriptDataEscapedDashDash,

    // The Script data escaped less-than sign state
    // Consume the next input character:
    // U+002F SOLIDUS (/)
    //     Set the temporary buffer to the empty string. Switch to the script data escaped end tag open state.
    // ASCII alpha
    //     Set the temporary
    // Anything else
    // Emit a U+003C LESS-THAN SIGN character token. Reconsume in the script data escaped state.
    // https://html.spec.whatwg.org/#script-data-escaped-less-than-sign-state
    ScriptDataEscapedLessThanSign,

    // The Script data escaped end tag open state
    // Consume the next input character:
    // ASCII alpha
    //     Create a new end tag token, set its tag name to the empty string. Reconsume in the script data escaped end tag name state.
    // Anything else
    //     Emit a U+003C LESS-THAN SIGN character token and a U+002F SOLIDUS character token. Reconsume in the script data escaped state.
    //  https://html.spec.whatwg.org/#script-data-escaped-end-tag-open-state
    ScriptDataEscapedEndTagOpen,

    // The Script data escaped end tag name state
    // Consume the next input character:
    // U+0009 CHARACTER TABULATION (tab)
    // U+000A LINE FEED (LF)
    // U+000C FORM FEED (FF)
    // U+0020 SPACE
    //     If the current end tag token is an appropriate end tag token, then switch to the before attribute name state. Otherwise, treat it as per the "anything else" entry below.
    // U+002F SOLIDUS (/)
    //     If the current end tag token is an appropriate end tag token, then switch to the self-closing start tag state. Otherwise, treat it as per the "anything else" entry below.
    // U+003E GREATER-THAN SIGN (>)
    //     If the current end tag token is an appropriate end tag token, then switch to the data state and emit the current tag token. Otherwise, treat it as per the "anything else" entry below.
    // ASCII upper alpha
    //     Append the lowercase version of the current input character (add 0x0020 to the character's code point) to the current tag token's tag name. Append the current input character to the temporary buffer.
    // ASCII lower alpha
    //     Append the current input character to the current tag token's tag name. Append the current input character to the temporary buffer.
    // Anything else
    //     Emit a U+003C LESS-THAN SIGN character token, a U+002F SOLIDUS character token, and a character token for each of the characters in the temporary buffer (in the order they were added to the buffer). Reconsume in the script data escaped state.
    // https://html.spec.whatwg.org/#script-data-escaped-end-tag-name-state
    ScriptDataEscapedEndTagName,

    // The Script data double escape start state
    // Consume the next input character:
    // U+0009 CHARACTER TABULATION (tab)
    // U+000A LINE FEED (LF)
    // U+000C FORM FEED (FF)
    // U+0020 SPACE
    // U+002F SOLIDUS (/)
    // U+003E GREATER-THAN SIGN (>)
    //     If the temporary buffer is the string "script", then switch to the script data escaped state. Otherwise, switch to the script data double escaped state. Emit the current input character as a character token.
    // ASCII upper alpha
    //     Append the lowercase version of the current input character (add 0x0020 to the character's code point) to the temporary buffer. Emit the current input character as a character token.
    // ASCII lower alpha
    //     Append the current input character to the temporary buffer. Emit the current input character as a character token.
    // Anything else
    //     Reconsume in the script data escaped state.
    // https://html.spec.whatwg.org/#script-data-double-escape-start-state
    ScriptDataDoubleEscapeStart,

    // The Script data double escaped state
    // Consume the next input character:
    // U+002D HYPHEN-MINUS (-)
    //     Switch to the script data double escaped dash state. Emit a U+002D HYPHEN-MINUS character token.
    // U+003C LESS-THAN SIGN (<)
    //     Switch to the script data double escaped less-than sign state.
    // U+0000 NULL
    //     This is an unexpected-null-character parse error. Emit a U+FFFD REPLACEMENT CHARACTER character token.
    // EOF
    //     This is an eof-in-script-html-comment-like-text parse error. Emit an end-of-file token.
    // Anything else
    //     Emit the current input character as a character token.
    // https://html.spec.whatwg.org/#script-data-double-escaped-state
    ScriptDataDoubleEscaped,

    // The Script data double escaped dash state
    // Consume the next input character:
    // U+002D HYPHEN-MINUS (-)
    //     Switch to the script data double escaped dash dash state. Emit a U+002D HYPHEN-MINUS character token.
    // U+003C LESS-THAN SIGN (<)
    //     Switch to the script data double escaped less-than sign state. Emit a U+003C LESS-THAN SIGN character token.
    // U+0000 NULL
    //     This is an unexpected-null-character parse error. Emit a U+FFFD REPLACEMENT CHARACTER character token.
    // EOF
    //     This is an eof-in-script-html-comment-like-text parse error. Emit an end-of-file token.
    // Anything else
    //     Switch to the script data double escaped state. Emit the current input character as a character token.
    // https://html.spec.whatwg.org/#script-data-double-escaped-dash-state
    ScriptDataDoubleEscapedDash,

    // The Script data double escaped dash dash state
    // Consume the next input character:
    // U+002D HYPHEN-MINUS (-)
    //     Emit a U+002D HYPHEN-MINUS character token.
    // U+003C LESS-THAN SIGN (<)
    //     Switch to the script data double escaped less-than sign state. Emit a U+003C LESS-THAN SIGN character token.
    // U+003E GREATER-THAN SIGN (>)
    //     Switch to the script data state. Emit a U+003E GREATER-THAN SIGN character token.
    // U+0000 NULL
    //     This is an unexpected-null-character parse error. Emit a U+FFFD REPLACEMENT CHARACTER character token.
    // EOF
    //     This is an eof-in-script-html-comment-like-text parse error. Emit an end-of-file token.
    // Anything else
    //     Switch to the script data double escaped state. Emit the current input character as a character token.
    // https://html.spec.whatwg.org/#script-data-double-escaped-dash-dash-state
    ScriptDataDoubleEscapedDashDash,

    // The Script data double escaped less-than sign state
    // Consume the next input character:
    // U+002F SOLIDUS (/)
    //     Set the temporary buffer to the empty string. Switch to the script data double escaped end state. Emit a U+002F SOLIDUS character token.
    // Anything else
    //     Reconsume in the script data double escaped state.
    // https://html.spec.whatwg.org/#script-data-double-escaped-less-than-sign-state
    ScriptDataDoubleEscapedLessThanSign,

    // The Script data double escaped end state
    // Consume the next input character:
    // U+0009 CHARACTER TABULATION (tab)
    // U+000A LINE FEED (LF)
    // U+000C FORM FEED (FF)
    // U+0020 SPACE
    // U+002F SOLIDUS (/)
    // U+003E GREATER-THAN SIGN (>)
    //     If the temporary buffer is the string "script", then switch to the script data escaped state. Otherwise, switch to the script data double escaped state. Emit the current input character as a character token.
    // ASCII upper alpha
    //     Append the lowercase version of the current input character (add 0x0020 to the character's code point) to the temporary buffer. Emit the current input character as a character token.
    // ASCII lower alpha
    //     Append the current input character to the temporary buffer. Emit the current input character as a character token.
    // Anything else
    //     Reconsume in the script data double escaped state.
    // https://html.spec.whatwg.org/#script-data-double-escaped-end-state
    ScriptDataDoubleEscapeEnd,

    // The Before attribute name state
    // Consume the next input character:
    // U+0009 CHARACTER TABULATION (tab)
    // U+000A LINE FEED (LF)
    // U+000C FORM FEED (FF)
    // U+0020 SPACE
    //     Ignore the character.
    // U+002F SOLIDUS (/)
    // U+003E GREATER-THAN SIGN (>)
    // EOF
    //     Reconsume in the after attribute name state.
    // U+003D EQUALS SIGN (=)
    //     This is an unexpected-equals-sign-before-attribute-name parse error. Start a new attribute in the current tag token. Set that attribute's name to the current input character, and its value to the empty string. Switch to the attribute name state.
    // Anything else
    //     Start a new attribute in the current tag token. Set that attribute's name and value to the empty string. Reconsume in the attribute name state.
    // https://html.spec.whatwg.org/#before-attribute-name-state
    BeforeAttributeName,

    // The Attribute name state
    // Consume the next input character:
    // U+0009 CHARACTER TABULATION (tab)
    // U+000A LINE FEED (LF)
    // U+000C FORM FEED (FF)
    // U+0020 SPACE
    // U+002F SOLIDUS (/)
    // U+003E GREATER-THAN SIGN (>)
    // EOF
    //     Reconsume in the after attribute name state.
    // U+003D EQUALS SIGN (=)
    //     Switch to the before attribute value state.
    // ASCII upper alpha
    //     Append the lowercase version of the current input character (add 0x0020 to the character's code point) to the current attribute's name.
    // U+0000 NULL
    //     This is an unexpected-null-character parse error. Append a U+FFFD REPLACEMENT CHARACTER character to the current attribute's name.
    // U+0022 QUOTATION MARK (")
    // U+0027 APOSTROPHE (')
    // U+003C LESS-THAN SIGN (<)
    //     This is an unexpected-character-in-attribute-name parse error. Treat it as per the "anything else" entry below.
    // Anything else
    //     Append the current input character to the current attribute's name.
    // https://html.spec.whatwg.org/#attribute-name-state
    AttributeName,

    // The After attribute name state
    // Consume the next input character:
    // U+0009 CHARACTER TABULATION (tab)
    // U+000A LINE FEED (LF)
    // U+000C FORM FEED (FF)
    // U+0020 SPACE
    //     Ignore the character.
    // U+002F SOLIDUS (/)
    //     Switch to the self-closing start tag state.
    // U+003D EQUALS SIGN (=)
    //     Switch to the before attribute value state.
    // U+003E GREATER-THAN SIGN (>)
    //     Switch to the data state and emit the current tag token.
    // EOF
    //     Emit an end-of-file token.
    // Anything else
    //     Start a new attribute in the current tag token. Set that attribute's name and value to the empty string. Reconsume in the attribute name state.
    // https://html.spec.whatwg.org/#after-attribute-name-state
    AfterAttributeName,

    // The Before attribute value state
    // Consume the next input character:
    // U+0009 CHARACTER TABULATION (tab)
    // U+000A LINE FEED (LF)
    // U+000C FORM FEED (FF)
    // U+0020 SPACE
    //     Ignore the character.
    // U+0022 QUOTATION MARK (")
    //     Switch to the attribute value (double-quoted) state.
    // U+0027 APOSTROPHE (')
    //     Switch to the attribute value (single-quoted) state.
    // U+003E GREATER-THAN SIGN (>)
    //     This is a missing-attribute-value parse error. Switch to the data state and emit the current tag token.
    // Anything else
    //     Reconsume in the attribute value (unquoted) state.
    // https://html.spec.whatwg.org/#before-attribute-value-state
    BeforeAttributeValue,

    // The Attribute value (double-quoted) state
    // Consume the next input character:
    // U+0022 QUOTATION MARK (")
    //     Switch to the after attribute value (quoted) state.
    // U+0026 AMPERSAND (&)
    //     Set the return state to the attribute value (double-quoted) state. Switch to the character reference state.
    // U+0000 NULL
    //     This is an unexpected-null-character parse error. Append a U+FFFD REPLACEMENT CHARACTER character to the current attribute's value.
    // EOF
    //     This is an eof-in-tag parse error. Emit an end-of-file token.
    // Anything else
    //     Append the current input character to the current attribute's value.
    // https://html.spec.whatwg.org/#attribute-value-double-quoted-state
    AttributeValueDoubleQuoted,

    // The Attribute value (single-quoted) state
    // Consume the next input character:
    // U+0027 APOSTROPHE (')
    //     Switch to the after attribute value (quoted) state.
    // U+0026 AMPERSAND (&)
    //     Set the return state to the attribute value (single-quoted) state. Switch to the character reference state.
    // U+0000 NULL
    //     This is an unexpected-null-character parse error. Append a U+FFFD REPLACEMENT CHARACTER character to the current attribute's value.
    // EOF
    //     This is an eof-in-tag parse error. Emit an end-of-file token.
    // Anything else
    //     Append the current input character to the current attribute's value.
    // https://html.spec.whatwg.org/#attribute-value-single-quoted-state
    AttributeValueSingleQuoted,

    // The Attribute value (unquoted) state
    // Consume the next input character:
    // U+0009 CHARACTER TABULATION (tab)
    // U+000A LINE FEED (LF)
    // U+000C FORM FEED (FF)
    // U+0020 SPACE
    //     Switch to the before attribute name state.
    // U+0026 AMPERSAND (&)
    //     Set the return state to the attribute value (unquoted) state. Switch to the character reference state.
    // U+003E GREATER-THAN SIGN (>)
    //     Switch to the data state and emit the current tag token.
    // U+0000 NULL
    //     This is an unexpected-null-character parse error. Append a U+FFFD REPLACEMENT CHARACTER character to the current attribute's value.
    // U+0022 QUOTATION MARK (")
    // U+0027 APOSTROPHE (')
    // U+003C LESS-THAN SIGN (<)
    // U+003D EQUALS SIGN (=)
    // U+0060 GRAVE ACCENT (`)
    //     This is an unexpected-character-in-unquoted-attribute-value parse error. Treat it as per the "anything else" entry below.
    // EOF
    //     This is an eof-in-tag parse error. Emit an end-of-file token.
    // Anything else
    //     Append the current input character to the current attribute's value.
    // https://html.spec.whatwg.org/#attribute-value-unquoted-state
    AttributeValueUnquoted,

    // The After attribute value (quoted) state
    // Consume the next input character:
    // U+0009 CHARACTER TABULATION (tab)
    // U+000A LINE FEED (LF)
    // U+000C FORM FEED (FF)
    // U+0020 SPACE
    //     Switch to the before attribute name state.
    // U+002F SOLIDUS (/)
    //     Switch to the self-closing start tag state.
    // U+003E GREATER-THAN SIGN (>)
    //     Switch to the data state and emit the current tag token.
    // EOF
    //     This is an eof-in-tag parse error. Emit an end-of-file token.
    // Anything else
    //     This is a missing-whitespace-between-attributes parse error. Reconsume in the before attribute name state.
    // https://html.spec.whatwg.org/#after-attribute-value-quoted-state
    AfterAttributeValueQuoted,

    // The Self-closing start tag state
    // Consume the next input character:
    // U+003E GREATER-THAN SIGN (>)
    //     Set the self-closing flag of the current tag token. Switch to the data state and emit the current tag token.
    // EOF
    //     This is an eof-in-tag parse error. Emit an end-of-file token.
    // Anything else
    //     This is an unexpected-solidus-in-tag parse error. Reconsume in the before attribute name state.
    // https://html.spec.whatwg.org/#self-closing-start-tag-state
    SelfClosingStartTag,

    // The Bogus comment state
    // Consume the next input character:
    // U+003E GREATER-THAN SIGN (>)
    //     Switch to the data state. Emit the comment token.
    // EOF
    //     Emit the comment token. Emit an end-of-file token.
    // U+0000 NULL
    //     This is an unexpected-null-character parse error. Append a U+FFFD REPLACEMENT CHARACTER character to the comment token's data.
    // Anything else
    //     Append the current input character to the comment token's data.
    // https://html.spec.whatwg.org/#bogus-comment-state
    BogusComment,

    // The Markup declaration open state
    // Consume the next input character:
    // Two U+002D HYPHEN-MINUS characters (-)
    //     Consume those two characters, create a comment token whose data is the empty string, and switch to the comment start state.
    // ASCII case-insensitive match for "DOCTYPE"
    //     Consume those characters and switch to the DOCTYPE state.
    // This string "[CDATA[" (the five uppercase letters "CDATA" with a U+005B LEFT SQUARE BRACKET character before and after)
    //     Consume those characters, If there is an adjusted current node and it it not an element in the HTML namespace, then switch to the CDATA section state. Otherwise, this is a cdata-in-html-content parse error. Create a comment token whose data is the "[CDATA[" string and switch to the bogus comment state.
    // Anything else
    //     This is an incorrectly-opened-comment parse error. Create a comment token whose data is the empty string and switch to the bogus comment state.
    // https://html.spec.whatwg.org/#markup-declaration-open-state
    MarkupDeclarationOpen,

    // The Comment start state
    // Consume the next input character:
    // U+002D HYPHEN-MINUS (-)
    //     Switch to the comment start dash state.
    // U+003E GREATER-THAN SIGN (>)
    //     This is an abrubt-closing-of-empty-comment parse error. Emit the comment token.
    // Anything else
    //     Reconsume in the comment state.
    // https://html.spec.whatwg.org/#comment-start-state
    CommentStart,

    // The Comment start dash state
    // Consume the next input character:
    // U+002D HYPHEN-MINUS (-)
    //     Switch to the comment end state.
    // U+003E GREATER-THAN SIGN (>)
    //     This is an abrubt-closing-of-empty-comment parse error. Emit the comment token.
    // EOF
    //     This is an eof-in-comment parse error. Emit the comment token. Emit an end-of-file token.
    // Anything else
    //     Append a U+002D HYPHEN-MINUS character (-) and the current input character to the comment token's data. Switch to the comment state.
    // https://html.spec.whatwg.org/#comment-start-dash-state
    CommentStartDash,

    // The Comment state
    // Consume the next input character:
    // U+003C LESS-THAN SIGN (<)
    //     Append teh current input character to the comment token's data. Switch to the comment less-than sign state.
    // U+002D HYPHEN-MINUS (-)
    //     Switch to the comment end dash state.
    // U+0000 NULL
    //     This is an unexpected-null-character parse error. Append a U+FFFD REPLACEMENT CHARACTER character to the comment token's data.
    // EOF
    //     This is an eof-in-comment parse error. Emit the comment token. Emit an end-of-file token.
    // Anything else
    //     Append the current input character to the comment token's data.
    // https://html.spec.whatwg.org/#comment-state
    Comment,

    // The Comment end dash state
    // Consume the next input character:
    // U+0021 EXCLAMATION MARK (!)
    //     Append the current input character to the comment token's data. Switch to the comment end bang state.
    // U+003C LESS-THAN SIGN (<)
    //     Append the current input character to the comment token's data.
    // Anything else
    //     Reconsume in the comment state.
    // https://html.spec.whatwg.org/#comment-end-dash-state
    CommentLessThanSign,

    // The Comment less-than sign bang state
    // Consume the next input character:
    // U+002D HYPHEN-MINUS (-)
    //     Switch to the comment less-than sign bang dash state.
    // Anything else
    //     Reconsume in the comment state.
    // https://html.spec.whatwg.org/#comment-less-than-sign-bang-dash-state
    CommentLessThanSignBang,

    // The Comment less-than sign bang dash state
    // Consume the next input character:
    // U+002D HYPHEN-MINUS (-)
    //     Switch to the comment less-than sign bang dash dash state.
    // Anything else
    //     Reconsume in the comment state.
    // https://html.spec.whatwg.org/#comment-less-than-sign-bang-dash-dash-state
    CommentLessThanSignBangDash,

    // The Comment less-than sign bang dash dash state
    // Consume the next input character:
    // U+003E GREATER-THAN SIGN (>)
    // EOF
    //     Reconsume in the comment end state.
    // Anything else
    //     This is a nested-comment parse error. Reconsume in the comment end state.
    // https://html.spec.whatwg.org/#comment-less-than-sign-bang-dash-dash-state
    CommentLessThanSignBangDashDash,

    // The Comment end dash state
    // Consume the next input character:
    // U+002D HYPHEN-MINUS (-)
    //     Switch to the comment end state.
    // EOF
    //     This is an eof-in-comment parse error. Emit the comment token. Emit an end-of-file token.
    // Anything else
    //     Append a U+002D HYPHEN-MINUS character (-) and the current input character to the comment token's data. Reconsume in the comment state.
    // https://html.spec.whatwg.org/#comment-end-dash-state
    CommentEndDash,

    // The Comment end state
    // Consume the next input character:
    // U+003E GREATER-THAN SIGN (>)
    //     Switch to the data state and emit the comment token.
    // U+0021 EXCLAMATION MARK (!)
    //     Switch to the comment end bang state.
    // U+002D HYPHEN-MINUS (-)
    //     Append a U+002D HYPHEN-MINUS character (-) to the comment token's data.
    // EOF
    //     This is an eof-in-comment parse error. Emit the comment token. Emit an end-of-file token.
    // Anything else
    //     Append two U+002D HYPHEN-MINUS characters (-) to the comment token's data. Reconsume in the comment state.
    // https://html.spec.whatwg.org/#comment-end-state
    CommentEnd,

    // The Comment end bang state
    // Consume the next input character:
    // U+002D HYPHEN-MINUS (-)
    //     Append a U+0021 EXCLAMATION MARK character (!) and a U+002D HYPHEN-MINUS character (-) to the comment token's data. Switch to the comment end dash state.
    // U+003E GREATER-THAN SIGN (>)
    //     This is an incorrectly-closed-comment parse error. Switch to the data state and emit the comment token.
    // EOF
    //     This is an eof-in-comment parse error. Emit the comment token. Emit an end-of-file token.
    // Anything else
    //     Append two U+002D HYPHEN-MINUS characters (-) and a U+0021 EXCLAMATION MARK character (!) to the comment token's data. Reconsume in the comment state.
    // https://html.spec.whatwg.org/#comment-end-bang-state
    CommentEndBang,

    // The DOCTYPE state
    // Consume the next input character:
    // U+0009 CHARACTER TABULATION (tab)
    // U+000A LINE FEED (LF)
    // U+000C FORM FEED (FF)
    // U+0020 SPACE
    //     Switch to the before DOCTYPE name state.
    // U+003E GREATER-THAN SIGN (>)
    //     Reconsume in the before DOCTYPE name state.
    // EOF
    //     This is an eof-in-doctype parse error. Create a new DOCTYPE token. Set its force-quirks flag to on. Emit the current token. Emit an end-of-file token.
    // Anything else
    //     This is a missing-whitespace-before-doctype-name parse error. Reconsume in the before DOCTYPE name state.
    // https://html.spec.whatwg.org/#doctype-state
    DOCTYPE,

    // The Before DOCTYPE name state
    // Consume the next input character:
    // U+0009 CHARACTER TABULATION (tab)
    // U+000A LINE FEED (LF)
    // U+000C FORM FEED (FF)
    // U+0020 SPACE
    //     Ignore the character.
    // ASCII upper alpha
    //     Create a new DOCTYPE token. Set its name to the lowercase version of the current input character (add 0x0020 to the character's code point). Switch to the DOCTYPE name state.
    // U+0000 NULL
    //     This is an unexpected-null-character parse error. Create a new DOCTYPE token. Set its name to the U+FFFD REPLACEMENT CHARACTER character. Switch to the DOCTYPE name state.
    // U+003E GREATER-THAN SIGN (>)
    //     This is a missing-doctype-name parse error. Create a new DOCTYPE token. Set its force-quirks flag to on. Switch to the data state and emit the current token.
    // EOF
    //     This is an eof-in-doctype parse error. Create a new DOCTYPE token. Set its force-quirks flag to on. Emit the current token. Emit an end-of-file token.
    // Anything else
    //     Create a new DOCTYPE token. Set its name to the current input character. Switch to the DOCTYPE name state.
    // https://html.spec.whatwg.org/#before-doctype-name-state
    BeforeDOCTYPEName,

    // The DOCTYPE name state
    // Consume the next input character:
    // U+0009 CHARACTER TABULATION (tab)
    // U+000A LINE FEED (LF)
    // U+000C FORM FEED (FF)
    // U+0020 SPACE
    //     Switch to the after DOCTYPE name state.
    // U+003E GREATER-THAN SIGN (>)
    //     Switch to the data state and emit the current token.
    // Uppercase ASCII letter
    //     Append the lowercase version of the current input character (add 0x0020 to the character's code point) to the current DOCTYPE token's name.
    // U+0000 NULL
    //     This is an unexpected-null-character parse error. Append a U+FFFD REPLACEMENT CHARACTER character to the current DOCTYPE token's name.
    // EOF
    //     This is an eof-in-doctype parse error. Set the current DOCTYPE token's force-quirks flag to on. Emit the current token. Emit an end-of-file token.
    // Anything else
    //     Append the current input character to the current DOCTYPE token's name.
    // https://html.spec.whatwg.org/#doctype-name-state
    DOCTYPEName,

    // The After DOCTYPE name state
    // Consume the next input character:
    // U+0009 CHARACTER TABULATION (tab)
    // U+000A LINE FEED (LF)
    // U+000C FORM FEED (FF)
    // U+0020 SPACE
    //     Ignore the character.
    // U+003E GREATER-THAN SIGN (>)
    //     Switch to the data state and emit the current token.
    // EOF
    //     This is an eof-in-doctype parse error. Set the current DOCTYPE token's force-quirks flag to on. Emit teh current DOCTYPE token. Emit an end-of-file token.
    // Anything else
    //     If the six characters starting from the current input character are an ASCII case-insensitive match for the word "PUBLIC", then consume those characters and switch to the after DOCTYPE public keyword state.
    //     Otherwise, if the six characters starting from the current input character are an ASCII case-insensitive match for the word "SYSTEM", then consume those characters and switch to the after DOCTYPE system keyword state.
    //     Otherwise, this is a missing-whitespace-after-doctype-name parse error. Set the DOCTYPE token's force-quirks flag to on. Reconsume in the bogus DOCTYPE state.
    // https://html.spec.whatwg.org/#after-doctype-name-state
    AfterDOCTYPEName,

    // The After DOCTYPE public keyword state
    // Consume the next input character:
    // U+0009 CHARACTER TABULATION (tab)
    // U+000A LINE FEED (LF)
    // U+000C FORM FEED (FF)
    // U+0020 SPACE
    //     Switch to the before DOCTYPE public identifier state.
    // U+0022 QUOTATION MARK (")
    //     This is a missing-whitespace-after-doctype-public-keyword parse error. Set the DOCTYPE token's public identifier to the empty string(not missing). Switch to the DOCTYPE public identifier (double-quoted) state.
    // U+0027 APOSTROPHE (')
    //     This is a missing-whitespace-after-doctype-public-keyword parse error. Set the DOCTYPE token's public identifier to the empty string(not missing). Switch to the DOCTYPE public identifier (single-quoted) state.
    // U+003E GREATER-THAN SIGN (>)
    //     This is a missing-doctype-public-identifier parse error. Set the DOCTYPE token's force-quirks flag to on. Switch to the data state and emit the current token.
    // EOF
    //     This is an eof-in-doctype parse error. Set the DOCTYPE token's force-quirks flag to on. Emit the current token. Emit an end-of-file token.
    // Anything else
    //     This is a missing-whitespace-after-doctype-public-keyword parse error. Set the DOCTYPE token's force-quirks flag to on. Reconsume in the bogus DOCTYPE state.
    // https://html.spec.whatwg.org/#after-doctype-public-keyword-state
    AfterDOCTYPEPublicKeyword,

    // The Before DOCTYPE public identifier state
    // Consume the next input character:
    // U+0009 CHARACTER TABULATION (tab)
    // U+000A LINE FEED (LF)
    // U+000C FORM FEED (FF)
    // U+0020 SPACE
    //     Ignore the character.
    // U+0022 QUOTATION MARK (")
    //     Set the DOCTYPE token's public identifier to the empty string (not missing). Switch to the DOCTYPE public identifier (double-quoted) state.
    // U+0027 APOSTROPHE (')
    //     Set the DOCTYPE token's public identifier to the empty string (not missing). Switch to the DOCTYPE public identifier (single-quoted) state.
    // U+003E GREATER-THAN SIGN (>)
    //     This is a missing-doctype-public-identifier parse error. Set the DOCTYPE token's force-quirks flag to on. Switch to the data state and emit the current token.
    // EOF
    //     This is an eof-in-doctype parse error. Set the DOCTYPE token's force-quirks flag to on. Emit the current token. Emit an end-of-file token.
    // Anything else
    //     This is a missing-whitespace-before-doctype-public-identifier parse error. Set the DOCTYPE token's force-quirks flag to on. Reconsume in the bogus DOCTYPE state.
    // https://html.spec.whatwg.org/#before-doctype-public-identifier-state
    BeforeDOCTYPEPublicIdentifier,

    // The DOCTYPE public identifier (double-quoted) state
    // Consume the next input character:
    // U+0022 QUOTATION MARK (")
    //     Switch to the after DOCTYPE public identifier state.
    // U+0000 NULL
    //     This is an unexpected-null-character parse error. Append a U+FFFD REPLACEMENT CHARACTER character to the current DOCTYPE token's public identifier.
    // U+003E GREATER-THAN SIGN (>)
    //     This is an abrupt-doctype-public-identifier parse error. Set the DOCTYPE token's force-quirks flag to on. Switch to the data state and emit the current token.
    // EOF
    //     This is an eof-in-doctype parse error. Set the DOCTYPE token's force-quirks flag to on. Emit the current token. Emit an end-of-file token.
    // Anything else
    //     Append the current input character to the current DOCTYPE token's public identifier.
    // https://html.spec.whatwg.org/#doctype-public-identifier-(double-quoted)-state
    DOCTYPEPublicIdentifierDoubleQuoted,

    // The DOCTYPE public identifier (single-quoted) state
    // Consume the next input character:
    // U+0027 APOSTROPHE (')
    //     Switch to the after DOCTYPE public identifier state.
    // U+0000 NULL
    //     This is an unexpected-null-character parse error. Append a U+FFFD REPLACEMENT CHARACTER character to the current DOCTYPE token's public identifier.
    // U+003E GREATER-THAN SIGN (>)
    //     This is an abrupt-doctype-public-identifier parse error. Set the DOCTYPE token's force-quirks flag to on. Switch to the data state and emit the current token.
    // EOF
    //     This is an eof-in-doctype parse error. Set the DOCTYPE token's force-quirks flag to on. Emit the current token. Emit an end-of-file token.
    // Anything else
    //     Append the current input character to the current DOCTYPE token's public identifier.
    // https://html.spec.whatwg.org/#doctype-public-identifier-(single-quoted)-state
    DOCTYPEPublicIdentifierSingleQuoted,

    // The After DOCTYPE public identifier state
    // Consume the next input character:
    // U+0009 CHARACTER TABULATION (tab)
    // U+000A LINE FEED (LF)
    // U+000C FORM FEED (FF)
    // U+0020 SPACE
    //     Switch to the between DOCTYPE public and system identifiers state.
    // U+0022 QUOTATION MARK (")
    //     This is a missing-whitespace-between-doctype-public-and-system-identifiers parse error. Set the DOCTYPE token's system identifier to the empty string (not missing). Switch to the DOCTYPE system identifier (double-quoted) state.
    // U+0027 APOSTROPHE (')
    //     This is a missing-whitespace-between-doctype-public-and-system-identifiers parse error. Set the DOCTYPE token's system identifier to the empty string (not missing). Switch to the DOCTYPE system identifier (single-quoted) state.
    // U+003E GREATER-THAN SIGN (>)
    //     Switch to the data state and emit the current token.
    // EOF
    //     This is an eof-in-doctype parse error. Set the DOCTYPE token's force-quirks flag to on. Emit the current token. Emit an end-of-file token.
    // Anything else
    //     This is a missing-whitespace-after-doctype-public-identifier parse error. Set the DOCTYPE token's force-quirks flag to on. Reconsume in the bogus DOCTYPE state.
    // https://html.spec.whatwg.org/#after-doctype-public-identifier-state
    AfterDOCTYPEPublicIdentifier,

    // The Between DOCTYPE public and system identifiers state
    // Consume the next input character:
    // U+0009 CHARACTER TABULATION (tab)
    // U+000A LINE FEED (LF)
    // U+000C FORM FEED (FF)
    // U+0020 SPACE
    //     Ignore the character.
    // U+003E GREATER-THAN SIGN (>)
    //     Switch to the data state and emit the current token.
    // U+0022 QUOTATION MARK (")
    //     Set the DOCTYPE token's system identifier to the empty string (not missing). Switch to the DOCTYPE system identifier (double-quoted) state.
    // U+0027 APOSTROPHE (')
    //     Set the DOCTYPE token's system identifier to the empty string (not missing). Switch to the DOCTYPE system identifier (single-quoted) state.
    // EOF
    //     This is an eof-in-doctype parse error. Set the DOCTYPE token's force-quirks flag to on. Emit the current token. Emit an end-of-file token.
    // Anything else
    //     This is a missing-whitespace-between-doctype-public-and-system-identifiers parse error. Set the DOCTYPE token's force-quirks flag to on. Reconsume in the bogus DOCTYPE state.
    // https://html.spec.whatwg.org/#between-doctype-public-and-system-identifiers-state
    BetweenDOCTYPEPublicAndSystemIdentifiers,

    // The After DOCTYPE system keyword state
    // Consume the next input character:
    // U+0009 CHARACTER TABULATION (tab)
    // U+000A LINE FEED (LF)
    // U+000C FORM FEED (FF)
    // U+0020 SPACE
    //     Ignore the character.
    // U+0022 QUOTATION MARK (")
    //     This is a missing-whitespace-after-doctype-system-keyword parse error. Set the DOCTYPE token's system identifier to the empty string (not missing). Switch to the DOCTYPE system identifier (double-quoted) state.
    // U+0027 APOSTROPHE (')
    //     This is a missing-whitespace-after-doctype-system-keyword parse error. Set the DOCTYPE token's system identifier to the empty string (not missing). Switch to the DOCTYPE system identifier (single-quoted) state.
    // U+003E GREATER-THAN SIGN (>)
    //     This is a missing-doctype-system-identifier parse error. Set the DOCTYPE token's force-quirks flag to on. Switch to the data state and emit the current token.
    // EOF
    //     This is an eof-in-doctype parse error. Set the DOCTYPE token's force-quirks flag to on. Emit the current token. Emit an end-of-file token.
    // Anything else
    //    This is a missing-quote-before-doctype-system-identifier parse error. Set the DOCTYPE token's force-quirks flag to on. Reconsume in the bogus DOCTYPE state.
    // https://html.spec.whatwg.org/#after-doctype-system-keyword-state
    AfterDOCTYPESystemKeyword,

    // The Before DOCTYPE system identifier state
    // Consume the next input character:
    // U+0009 CHARACTER TABULATION (tab)
    // U+000A LINE FEED (LF)
    // U+000C FORM FEED (FF)
    // U+0020 SPACE
    //     Ignore the character.
    // U+0022 QUOTATION MARK (")
    //     Switch to the DOCTYPE system identifier (double-quoted) state.
    // U+0027 APOSTROPHE (')
    //     Switch to the DOCTYPE system identifier (single-quoted) state.
    // U+003E GREATER-THAN SIGN (>)
    //     This is a missing-doctype-system-identifier parse error. Set the DOCTYPE token's force-quirks flag to on. Switch to the data state and emit the current token.
    // EOF
    //     This is an eof-in-doctype parse error. Set the DOCTYPE token's force-quirks flag to on. Emit the current token. Emit an end-of-file token.
    // Anything else
    //     This is a missing-whitespace-before-doctype-system-identifier parse error. Set the DOCTYPE token's force-quirks flag to on. Reconsume in the bogus DOCTYPE state.
    // https://html.spec.whatwg.org/#before-doctype-system-identifier-state
    BeforeDOCTYPESystemIdentifier,

    // The DOCTYPE system identifier (double-quoted) state
    // Consume the next input character:
    // U+0022 QUOTATION MARK (")
    //     Switch to the after DOCTYPE system identifier state.
    // U+0000 NULL
    //     This is an unexpected-null-character parse error. Append a U+FFFD REPLACEMENT CHARACTER character to the current DOCTYPE token's system identifier.
    // U+003E GREATER-THAN SIGN (>)
    //     This is an abrupt-doctype-system-identifier parse error. Set the DOCTYPE token's force-quirks flag to on. Switch to the data state and emit the current token.
    // EOF
    //     This is an eof-in-doctype parse error. Set the DOCTYPE token's force-quirks flag to on. Emit the current token. Emit an end-of-file token.
    // Anything else
    //     Append the current input character to the current DOCTYPE token's system identifier.
    // https://html.spec.whatwg.org/#doctype-system-identifier-(double-quoted)-state
    DOCTYPESystemIdentifierDoubleQuoted,

    // The DOCTYPE system identifier (single-quoted) state
    // Consume the next input character:
    // U+0027 APOSTROPHE (')
    //     Switch to the after DOCTYPE system identifier state.
    // U+0000 NULL
    //     This is an unexpected-null-character parse error. Append a U+FFFD REPLACEMENT CHARACTER character to the current DOCTYPE token's system identifier.
    // U+003E GREATER-THAN SIGN (>)
    //     This is an abrupt-doctype-system-identifier parse error. Set the DOCTYPE token's force-quirks flag to on. Switch to the data state and emit the current token.
    // EOF
    //     This is an eof-in-doctype parse error. Set the DOCTYPE token's force-quirks flag to on. Emit the current token. Emit an end-of-file token.
    // Anything else
    //     Append the current input character to the current DOCTYPE token's system identifier.
    // https://html.spec.whatwg.org/#doctype-system-identifier-(single-quoted)-state
    DOCTYPESystemIdentifierSingleQuoted,

    // The After DOCTYPE system identifier state
    // Consume the next input character:
    // U+0009 CHARACTER TABULATION (tab)
    // U+000A LINE FEED (LF)
    // U+000C FORM FEED (FF)
    // U+0020 SPACE
    //     Ignore the character.
    // U+003E GREATER-THAN SIGN (>)
    //     Switch to the data state and emit the current token.
    // EOF
    //     This is an eof-in-doctype parse error. Emit the current token. Emit an end-of-file token.
    // Anything else
    //     This is an unexpected-character-after-doctype-system-identifier parse error. Reconsume in the bogus DOCTYPE state. (This does not set the current DOCTYPE token's force-quirks flag to on)
    // https://html.spec.whatwg.org/#after-doctype-system-identifier-state
    AfterDOCTYPESystemIdentifier,

    // The Bogus DOCTYPE state
    // Consume the next input character:
    // U+003E GREATER-THAN SIGN (>)
    //     Switch to the data state and emit the current token.
    // U+0000 NULL
    //     This is an unexpected-null-character parse error. Ignore the character.
    // EOF
    //     Emit the current token. Emit an end-of-file token.
    // Anything else
    //     Ignore the character.
    // https://html.spec.whatwg.org/#bogus-doctype-state
    BogusDOCTYPE,

    // The CDATA section state
    // Consume the next input character:
    // U+005D RIGHT SQUARE BRACKET (])
    //     Switch to the CDATA section bracket state.
    // EOF
    //     This is an eof-in-cdata parse error. Emit an end-of-file token.
    // Anything else
    //     Emit the current input character as a character token.
    // https://html.spec.whatwg.org/#cdata-section-state
    CDATASection,

    // The CDATA section bracket state
    // Consume the next input character:
    // U+005D RIGHT SQUARE BRACKET (])
    //     Switch to the CDATA section end state.
    // Anything else
    //     Emit a U+005D RIGHT SQUARE BRACKET character (]) as a character token. Reconsume in the CDATA section state.
    // https://html.spec.whatwg.org/#cdata-section-bracket-state
    CDATASectionBracket,

    // The CDATA section end state
    // Consume the next input character:
    // U+005D RIGHT SQUARE BRACKET (])
    //     Emit a U+005D RIGHT SQUARE BRACKET character (]) as a character token.
    // U+003E GREATER-THAN SIGN (>)
    //    Switch to the data state.
    // Anything else
    //     Emit two U+005D RIGHT SQUARE BRACKET characters (]]) as character tokens. Reconsume in the CDATA section state.
    // https://html.spec.whatwg.org/#cdata-section-end-state
    CDATASectionEnd,

    // The Character reference state
    // Set the temporary buffer to the empty string. Append a U+0026 AMPERSAND (&) character to the temporary buffer. Consume the next input character:
    // ASCII alphanumeric
    //     Reconsume in the named character reference state.
    // U+0023 NUMBER SIGN (#)
    //     Append the current input character to the temporary buffer. Switch to the numeric character reference state.
    // Anything else
    //     Flush code points consumed as a character reference. Reconsume in the return state.
    // https://html.spec.whatwg.org/#character-reference-state
    CharacterReference,

    // The Named character reference state
    // Consume the maximum number of characters possible, where the consumed characters are on eof the identifiers in the first column of the named character references table.
    // Append each character to the temporary buffer when it is consumed.
    // If there is a match
    //     If the character reference was consumed as part of an attribute, and the last character matched is not a U+003B SEMICOLON character (;),
    //     and the next character is either a U+003D EQUALS SIGN character (=) or an ASCII alphanumeric, then, for each character in the temporary buffer,
    //     emit the character as a character token. Reconsume in the return state.
    //
    //     Otherwise

    //       1. If the last character matched is not a U+003B SEMICOLON character (;), this is a missing-semicolon-after-character-reference parse error.
    //       2. Set the temporary buffer to the empty string. Append one or two characters corresponding to the character reference name
    //       (as given by the second column of the named character references table) to the temporary buffer.
    //       3. Flush code points consumed as a character reference. Reconsume in the return state.
    // https://html.spec.whatwg.org/#named-character-reference-state
    NamedCharacterReference,

    // The Ambiguous ampersand state
    // Consume the next input character:
    // ASCII alphanumeric
    //     If the character reference was consumed as part of an attribute,then append the current input character to the current attribute's value. Otherwise, emit the current input character as a character token.
    // U+003B SEMICOLON (;)
    //     This is an unknown-named-character-reference parse error. Reconsume in the return state.
    // Anything else
    //     Reconsume in the return state.
    // https://html.spec.whatwg.org/#ambiguous-ampersand-state
    AmbiguousAmpersand,

    // The Numeric character reference state
    // Set the character reference code to zero (0)
    // Consume the next input character:
    // U+0078 LATIN SMALL LETTER X (x)
    //     Appened the current input character to the temporary buffer. Switch to the hexadecimal character reference start state.
    // Anything else
    //     Reconsume in the decimal character reference start state.
    // https://html.spec.whatwg.org/#numeric-character-reference-state
    NumericCharacterReference,

    // The Hexadecimal character reference start state
    // Consume the next input character:
    // ASCII hex digit
    //     Reconsume in the hexadecimal character reference state.
    // Anything else
    //     This is an absence-of-digits-in-numeric-character-reference parse error. Flush code points consumed as a character reference. Reconsume in the return state.
    // https://html.spec.whatwg.org/#hexadecimal-character-reference-start-state
    HexadecimalCharacterReferenceStart,

    // The Decimal character reference start state
    // Consume the next input character:
    // ASCII digit
    //     Reconsume in the decimal character reference state.
    // Anything else
    //     This is an absence-of-digits-in-numeric-character-reference parse error. Flush code points consumed as a character reference. Reconsume in the return state.
    // https://html.spec.whatwg.org/#decimal-character-reference-start-state
    DecimalCharacterReferenceStart,

    // The Hexadecimal character reference state
    // Consume the next input character:
    // ASCII digit
    //     Multiply the character reference code by 0x10 (16). Add a numeric version of the current input character (subtract 0x0030 from the character's code point) to the character reference code.
    // ASCII uppercase ASCII letter
    //     Multiply the character reference code by 0x10 (16). Add a numeric version of the current input character (subtract 0x0037 from the character's code point) to the character reference code.
    // ASCII lowercase ASCII letter
    //     Multiply the character reference code by 0x10 (16). Add a numeric version of the current input character (subtract 0x0057 from the character's code point) to the character reference code.
    // U+003B SEMICOLON (;)
    //     Switch to the numeric character reference end state.
    // Anything else
    //     This is an unexpected-character-in-numeric-character-reference parse error. Reconsume in the numeric character reference end state.
    // https://html.spec.whatwg.org/#hexadecimal-character-reference-state
    HexadecimalCharacterReference,

    // The Decimal character reference state
    // Consume the next input character:
    // ASCII digit
    //     Multiply the character reference code by 10. Add a numeric version of the current input character (subtract 0x0030 from the character's code point) to the character reference code.
    // U+003B SEMICOLON (;)
    //     Switch to the numeric character reference end state.
    // Anything else
    //     This is an unexpected-character-in-numeric-character-reference parse error. Reconsume in the numeric character reference end state.
    // https://html.spec.whatwg.org/#decimal-character-reference-state
    DecimalCharacterReference,

    // The Numeric character reference end state
    // Check the character reference code:
    // If the number is 0x00, then this is a null-character-reference parse error. Emit a U+FFFD REPLACEMENT CHARACTER character token.
    // If the number is greater than 0x10FFFF, then this is a character-reference-outside-unicode-range parse error. Set the character reference code to 0xFFFD, the REPLACEMENT CHARACTER.
    // If the number is a surrogate, then this is a surrogate-character-reference parse error. Set the character reference code to 0xFFFD, the REPLACEMENT CHARACTER.
    // If the number is a noncharacter, then this is a noncharacter-character-reference parse error.
    // If the number is 0x0D, or a control that's not ASCII whitespace, then this is a control character reference parse error. If the number is one of the numbers in the first column of the following table, then find the row with that number in the first column, and set the character reference code to the number in the second column of that row.
    // https://html.spec.whatwg.org/#numeric-character-reference-end-state
    NumericCharacterReferenceEnd,
}
