use std::collections::HashMap;
use std::error::Error;

use crate::resolve_modified_ref::resolve_modified_ref;
use graphqxl_parser::{BlockDef, BlockEntry, GenericBlockDef};

pub(crate) fn transpile_generic_block_def(
    generic_block_def: &GenericBlockDef,
    store: &HashMap<String, BlockDef>,
) -> Result<BlockDef, Box<dyn Error>> {
    let resolved = resolve_modified_ref(&generic_block_def.modified_ref, store)?;

    let mut directives = resolved.directives.clone();
    directives.extend(generic_block_def.directives.clone());

    let description = if !generic_block_def.description.is_empty() {
        generic_block_def.description.clone()
    } else {
        resolved.description
    };

    Ok(BlockDef {
        span: generic_block_def.span.clone(),
        name: generic_block_def.name.clone(),
        implements: resolved.implements.clone(),
        generic: None,
        description,
        kind: generic_block_def.kind.clone(),
        directives,
        entries: resolved
            .fields
            .iter()
            .map(|el| BlockEntry::Field(el.clone()))
            .collect(),
    })
}
