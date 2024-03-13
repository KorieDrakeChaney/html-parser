#[derive(Debug, PartialEq, Clone, Copy)]
pub enum State {
    // The Data State
    // https://html.spec.whatwg.org/#data-state
    Data,

    // The Replaceable Character Data (RCDATA) state
    // https://html.spec.whatwg.org/#rcdata-state
    RCDATA,

    // The RAWTEXT state
    // https://html.spec.whatwg.org/#rawtext-state
    RAWTEXT,

    // The Script data state
    // https://html.spec.whatwg.org/#script-data-state
    ScriptData,

    // The PLAINTEXT state
    // https://html.spec.whatwg.org/#plaintext-state
    PLAINTEXT,

    // The Tag open state
    // https://html.spec.whatwg.org/#tag-open-state
    TagOpen,

    // The End tag open state
    // https://html.spec.whatwg.org/#end-tag-open-state
    EndTagOpen,

    // The Tag name state
    // https://html.spec.whatwg.org/#tag-name-state
    TagName,

    // The RCDATA less-than sign state
    // https://html.spec.whatwg.org/#rcdata-less-than-sign-state
    RCDATALessThanSign,

    // The RCDATA end tag open state
    // https://html.spec.whatwg.org/#rcdata-end-tag-open-state
    RCDATAEndTagOpen,

    // The RCDATA end tag name state
    // https://html.spec.whatwg.org/#rcdata-end-tag-name-state
    RCDATAEndTagName,

    // The RAWTEXT less-than sign state
    // https://html.spec.whatwg.org/#rawtext-less-than-sign-state
    RAWTEXTLessThanSign,

    // The RAWTEXT end tag open state
    // https://html.spec.whatwg.org/#rawtext-end-tag-open-state
    RAWTEXTEndTagOpen,

    // The RAWTEXT end tag name state
    // https://html.spec.whatwg.org/#rawtext-end-tag-name-state
    RAWTEXTEndTagName,

    // The Script data less-than sign state
    // https://html.spec.whatwg.org/#script-data-less-than-sign-state
    ScriptDataLessThanSign,

    // The Script data end tag open state
    // https://html.spec.whatwg.org/#script-data-end-tag-open-state
    ScriptDataEndTagOpen,

    // The Script data end tag name state
    // https://html.spec.whatwg.org/#script-data-end-tag-name-state
    ScriptDataEndTagName,

    // The Script data escape start state
    // https://html.spec.whatwg.org/#script-data-escape-start-state
    ScriptDataEscapeStart,

    // The Script data escape start dash state
    // https://html.spec.whatwg.org/#script-data-escape-start-dash-state
    ScriptDataEscapeStartDash,

    // The Script data escaped state
    // https://html.spec.whatwg.org/#script-data-escaped-state
    ScriptDataEscaped,

    // The Script data escaped dash state
    // https://html.spec.whatwg.org/#script-data-escaped-dash-state
    ScriptDataEscapedDash,

    // The Script data escaped dash dash state
    // https://html.spec.whatwg.org/#script-data-escaped-dash-dash-state
    ScriptDataEscapedDashDash,

    // The Script data escaped less-than sign state
    // https://html.spec.whatwg.org/#script-data-escaped-less-than-sign-state
    ScriptDataEscapedLessThanSign,

    // The Script data escaped end tag open state
    //  https://html.spec.whatwg.org/#script-data-escaped-end-tag-open-state
    ScriptDataEscapedEndTagOpen,

    // The Script data escaped end tag name state
    // https://html.spec.whatwg.org/#script-data-escaped-end-tag-name-state
    ScriptDataEscapedEndTagName,

    // The Script data double escape start state
    // https://html.spec.whatwg.org/#script-data-double-escape-start-state
    ScriptDataDoubleEscapeStart,

    // The Script data double escaped state
    // https://html.spec.whatwg.org/#script-data-double-escaped-state
    ScriptDataDoubleEscaped,

    // The Script data double escaped dash state
    // https://html.spec.whatwg.org/#script-data-double-escaped-dash-state
    ScriptDataDoubleEscapedDash,

    // The Script data double escaped dash dash state
    // https://html.spec.whatwg.org/#script-data-double-escaped-dash-dash-state
    ScriptDataDoubleEscapedDashDash,

    // The Script data double escaped less-than sign state
    // https://html.spec.whatwg.org/#script-data-double-escaped-less-than-sign-state
    ScriptDataDoubleEscapedLessThanSign,

    // The Script data double escaped end state
    // https://html.spec.whatwg.org/#script-data-double-escaped-end-state
    ScriptDataDoubleEscapeEnd,

    // The Before attribute name state
    // https://html.spec.whatwg.org/#before-attribute-name-state
    BeforeAttributeName,

    // The Attribute name state
    // https://html.spec.whatwg.org/#attribute-name-state
    AttributeName,

    // The After attribute name state
    // https://html.spec.whatwg.org/#after-attribute-name-state
    AfterAttributeName,

    // The Before attribute value state
    // https://html.spec.whatwg.org/#before-attribute-value-state
    BeforeAttributeValue,

    // The Attribute value (double-quoted) state
    // https://html.spec.whatwg.org/#attribute-value-double-quoted-state
    AttributeValueDoubleQuoted,

    // The Attribute value (single-quoted) state
    // https://html.spec.whatwg.org/#attribute-value-single-quoted-state
    AttributeValueSingleQuoted,

    // The Attribute value (unquoted) state
    // https://html.spec.whatwg.org/#attribute-value-unquoted-state
    AttributeValueUnquoted,

    // The After attribute value (quoted) state
    // https://html.spec.whatwg.org/#after-attribute-value-quoted-state
    AfterAttributeValueQuoted,

    // The Self-closing start tag state

    // https://html.spec.whatwg.org/#self-closing-start-tag-state
    SelfClosingStartTag,

    // The Bogus comment state
    // https://html.spec.whatwg.org/#bogus-comment-state
    BogusComment,

    // The Markup declaration open state
    // https://html.spec.whatwg.org/#markup-declaration-open-state
    MarkupDeclarationOpen,

    // The Comment start state
    // https://html.spec.whatwg.org/#comment-start-state
    CommentStart,

    // The Comment start dash state
    // https://html.spec.whatwg.org/#comment-start-dash-state
    CommentStartDash,

    // The Comment state
    // https://html.spec.whatwg.org/#comment-state
    Comment,

    // The Comment end dash state
    // https://html.spec.whatwg.org/#comment-end-dash-state
    CommentLessThanSign,

    // The Comment less-than sign bang state
    // https://html.spec.whatwg.org/#comment-less-than-sign-bang-dash-state
    CommentLessThanSignBang,

    // The Comment less-than sign bang dash state
    // https://html.spec.whatwg.org/#comment-less-than-sign-bang-dash-dash-state
    CommentLessThanSignBangDash,

    // The Comment less-than sign bang dash dash state
    // https://html.spec.whatwg.org/#comment-less-than-sign-bang-dash-dash-state
    CommentLessThanSignBangDashDash,

    // The Comment end dash state
    // https://html.spec.whatwg.org/#comment-end-dash-state
    CommentEndDash,

    // The Comment end state
    // https://html.spec.whatwg.org/#comment-end-state
    CommentEnd,

    // The Comment end bang state
    // https://html.spec.whatwg.org/#comment-end-bang-state
    CommentEndBang,

    // The DOCTYPE state
    // https://html.spec.whatwg.org/#doctype-state
    DOCTYPE,

    // The Before DOCTYPE name state
    // https://html.spec.whatwg.org/#before-doctype-name-state
    BeforeDOCTYPEName,

    // The DOCTYPE name state
    // https://html.spec.whatwg.org/#doctype-name-state
    DOCTYPEName,

    // The After DOCTYPE name state
    // https://html.spec.whatwg.org/#after-doctype-name-state
    AfterDOCTYPEName,

    // The After DOCTYPE public keyword state
    // https://html.spec.whatwg.org/#after-doctype-public-keyword-state
    AfterDOCTYPEPublicKeyword,

    // The Before DOCTYPE public identifier state
    // https://html.spec.whatwg.org/#before-doctype-public-identifier-state
    BeforeDOCTYPEPublicIdentifier,

    // The DOCTYPE public identifier (double-quoted) state
    // https://html.spec.whatwg.org/#doctype-public-identifier-(double-quoted)-state
    DOCTYPEPublicIdentifierDoubleQuoted,

    // The DOCTYPE public identifier (single-quoted) state
    // https://html.spec.whatwg.org/#doctype-public-identifier-(single-quoted)-state
    DOCTYPEPublicIdentifierSingleQuoted,

    // The After DOCTYPE public identifier state
    // https://html.spec.whatwg.org/#after-doctype-public-identifier-state
    AfterDOCTYPEPublicIdentifier,

    // The Between DOCTYPE public and system identifiers state
    // https://html.spec.whatwg.org/#between-doctype-public-and-system-identifiers-state
    BetweenDOCTYPEPublicAndSystemIdentifiers,

    // The After DOCTYPE system keyword state
    // https://html.spec.whatwg.org/#after-doctype-system-keyword-state
    AfterDOCTYPESystemKeyword,

    // The Before DOCTYPE system identifier state
    // https://html.spec.whatwg.org/#before-doctype-system-identifier-state
    BeforeDOCTYPESystemIdentifier,

    // The DOCTYPE system identifier (double-quoted) state
    // https://html.spec.whatwg.org/#doctype-system-identifier-(double-quoted)-state
    DOCTYPESystemIdentifierDoubleQuoted,

    // The DOCTYPE system identifier (single-quoted) state
    // https://html.spec.whatwg.org/#doctype-system-identifier-(single-quoted)-state
    DOCTYPESystemIdentifierSingleQuoted,

    // The After DOCTYPE system identifier state
    // https://html.spec.whatwg.org/#after-doctype-system-identifier-state
    AfterDOCTYPESystemIdentifier,

    // The Bogus DOCTYPE state
    // https://html.spec.whatwg.org/#bogus-doctype-state
    BogusDOCTYPE,

    // The CDATA section state
    // https://html.spec.whatwg.org/#cdata-section-state
    CDATASection,

    // The CDATA section bracket state
    // https://html.spec.whatwg.org/#cdata-section-bracket-state
    CDATASectionBracket,

    // The CDATA section end state
    // https://html.spec.whatwg.org/#cdata-section-end-state
    CDATASectionEnd,

    // The Character reference state
    // https://html.spec.whatwg.org/#character-reference-state
    CharacterReference,

    // The Named character reference state
    // https://html.spec.whatwg.org/#named-character-reference-state
    NamedCharacterReference,

    // The Ambiguous ampersand state
    // https://html.spec.whatwg.org/#ambiguous-ampersand-state
    AmbiguousAmpersand,

    // The Numeric character reference state
    // https://html.spec.whatwg.org/#numeric-character-reference-state
    NumericCharacterReference,

    // The Hexadecimal character reference start state
    // https://html.spec.whatwg.org/#hexadecimal-character-reference-start-state
    HexadecimalCharacterReferenceStart,

    // The Decimal character reference start state
    // https://html.spec.whatwg.org/#decimal-character-reference-start-state
    DecimalCharacterReferenceStart,

    // The Hexadecimal character reference state
    // https://html.spec.whatwg.org/#hexadecimal-character-reference-state
    HexadecimalCharacterReference,

    // The Decimal character reference state
    // https://html.spec.whatwg.org/#decimal-character-reference-state
    DecimalCharacterReference,

    // The Numeric character reference end state
    // https://html.spec.whatwg.org/#numeric-character-reference-end-state
    NumericCharacterReferenceEnd,
}
