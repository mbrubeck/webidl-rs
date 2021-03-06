extern crate webidl;
extern crate zip;

use std::fs;
use std::io::Read;

use webidl::*;

#[test]
fn parse_servo_webidls() {
    let parser = Parser::new();
    let file = fs::File::open("tests/mozilla_webidls.zip").unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let mut webidl = String::new();
        file.read_to_string(&mut webidl).unwrap();

        // With the new update to the specification, the "implements" definition has been replaced
        // with "includes", but the Mozilla WebIDLs have not been updated.

        if let Err(err) = parser.parse_string(&*webidl) {
            match err {
                ParseError::UnrecognizedToken {
                    token: Some((_, ref token, _)),
                    ..
                } if *token == Token::Identifier("implements".to_string()) =>
                {
                    break
                }
                _ => panic!("parse error: {:?}", err),
            }
        }
    }
}

// A test case using the "includes" definition does not appear in the Mozilla WebIDLs, so it needs
// to be tested separately.
#[test]
fn parse_includes() {
    use ast::*;

    let parser = Parser::new();
    let ast = parser.parse_string("[test] A includes B;").unwrap();

    assert_eq!(
        ast,
        vec![
            Definition::Includes(Includes {
                extended_attributes: vec![
                    Box::new(ExtendedAttribute::NoArguments(Other::Identifier(
                        "test".to_string(),
                    ))),
                ],
                includee: "B".to_string(),
                includer: "A".to_string(),
            }),
        ]
    );
}

// A test case using the "mixin" definition does not appear in the Mozilla WebIDLs, so it needs to
// be tested separately.
#[test]
fn parse_mixin() {
    use ast::*;

    let parser = Parser::new();
    let ast = parser
        .parse_string(
            "[test]
            partial interface mixin Name {
                readonly attribute unsigned short entry;
            };",
        )
        .unwrap();

    assert_eq!(
        ast,
        vec![
            Definition::Mixin(Mixin::Partial(PartialMixin {
                extended_attributes: vec![
                    Box::new(ExtendedAttribute::NoArguments(Other::Identifier(
                        "test".to_string(),
                    ))),
                ],
                members: vec![
                    MixinMember::Attribute(Attribute::Regular(RegularAttribute {
                        extended_attributes: vec![],
                        inherits: false,
                        name: "entry".to_string(),
                        read_only: true,
                        type_: Box::new(Type {
                            extended_attributes: vec![],
                            kind: TypeKind::UnsignedShort,
                            nullable: false,
                        }),
                    })),
                ],
                name: "Name".to_string(),
            })),
        ]
    );
}
