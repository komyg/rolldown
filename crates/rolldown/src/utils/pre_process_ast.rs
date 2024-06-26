use std::path::Path;

use oxc::span::SourceType;
use oxc::transformer::{TransformOptions, Transformer};
use rolldown_oxc_utils::OxcAst;

use crate::types::oxc_parse_type::OxcParseType;

use super::tweak_ast_for_scanning::tweak_ast_for_scanning;

// #[allow(clippy::match_same_arms)]: `OxcParseType::Tsx` will have special logic to deal with ts compared to `OxcParseType::Jsx`
#[allow(clippy::match_same_arms)]
pub fn pre_process_ast(
  mut ast: OxcAst,
  parse_type: &OxcParseType,
  path: &Path,
  source_type: SourceType,
) -> anyhow::Result<OxcAst> {
  if let Err(errors) = ast.with_mut(|fields| {
    let mut transformer_options = TransformOptions::default();
    match parse_type {
      OxcParseType::Js => {
        // Bailout because there are no enabled features that need to pre process the js ast.
        return Ok(());
      }
      OxcParseType::Jsx => {
        transformer_options.react.jsx_plugin = true;
      }
      OxcParseType::Ts => {}
      OxcParseType::Tsx => {
        transformer_options.react.jsx_plugin = true;
      }
    }

    Transformer::new(
      fields.allocator,
      path,
      source_type,
      fields.source,
      fields.trivias,
      transformer_options,
    )
    .build(fields.program)
  }) {
    return Err(anyhow::anyhow!("Transform failed, got {:#?}", errors));
  }

  tweak_ast_for_scanning(&mut ast);

  Ok(ast)
}
