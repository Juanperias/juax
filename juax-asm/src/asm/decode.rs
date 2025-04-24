use crate::parser::ast::AstNode;
use juax_lib::{jli::JliFile, opcode::Opcode};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("Cannot decode node {0:?}")]
    InvalidNode(AstNode),
}

pub fn decode_node(node: AstNode) -> Result<Opcode, DecodeError> {
    Ok(match node {
        AstNode::Mov { to, from } => Opcode {
            arg1: to.into(),
            arg2: from.into(),
            imm: 0,
            ins: 0x04,
        },
        AstNode::Load { dist, val } => Opcode {
            arg1: dist.into(),
            arg2: 0x0,
            imm: val,
            ins: 0x02,
        },
        _ => {
            return Err(DecodeError::InvalidNode(node));
        }
    })
}

pub fn decode_tree(tree: Vec<AstNode>) -> Result<JliFile, DecodeError> {
    let mut jli = JliFile::new();

    for node in tree {
        jli.write(decode_node(node)?);
    }

    Ok(jli)
}
