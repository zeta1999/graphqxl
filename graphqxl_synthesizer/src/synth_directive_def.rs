use crate::synth_arguments::ArgumentsSynth;
use crate::synth_description::DescriptionSynth;
use crate::synths::{ChainSynth, ListSynth, PairSynth, StringSynth, Synth, SynthContext};
use graphqxl_parser::{DirectiveDef, DirectiveLocation};

pub(crate) struct DirectiveDefSynth(pub(crate) DirectiveDef);

fn print_directive_location(directive_location: &DirectiveLocation) -> String {
    match directive_location {
        DirectiveLocation::Query => "QUERY".to_string(),
        DirectiveLocation::Mutation => "MUTATION".to_string(),
        DirectiveLocation::Subscription => "SUBSCRIPTION".to_string(),
        DirectiveLocation::FieldDefinition => "FIELD_DEFINITION".to_string(),
        DirectiveLocation::Field => "FIELD".to_string(),
        DirectiveLocation::FragmentDefinition => "FRAGMENT_DEFINITION".to_string(),
        DirectiveLocation::FragmentSpread => "FRAGMENT_SPREAD".to_string(),
        DirectiveLocation::InlineFragment => "INLINE_FRAGMENT".to_string(),
        DirectiveLocation::Schema => "SCHEMA".to_string(),
        DirectiveLocation::Scalar => "SCALAR".to_string(),
        DirectiveLocation::Object => "OBJECT".to_string(),
        DirectiveLocation::ArgumentDefinition => "ARGUMENT_DEFINITION".to_string(),
        DirectiveLocation::Interface => "INTERFACE".to_string(),
        DirectiveLocation::Union => "UNION".to_string(),
        DirectiveLocation::EnumValue => "ENUM_VALUE".to_string(),
        DirectiveLocation::Enum => "ENUM".to_string(),
        DirectiveLocation::InputObject => "INPUT_OBJECT".to_string(),
        DirectiveLocation::InputFieldDefinition => "INPUT_FIELD_DEFINITION".to_string(),
        DirectiveLocation::VariableDefinition => "VARIABLE_DEFINITION".to_string(),
    }
}

struct DirectiveLocationSynth(Vec<DirectiveLocation>);

impl Synth for DirectiveLocationSynth {
    fn synth(&self, context: &SynthContext) -> String {
        let list_synth = ListSynth::inline_or_multiline_suffixed((
            "",
            self.0
                .iter()
                .map(|t| StringSynth(print_directive_location(t)))
                .collect(),
            (" | ", " |"),
            "",
        ));
        if self.0.len() > context.max_one_line_ors {
            list_synth.synth(&context.multiline())
        } else {
            list_synth.synth(&context.no_multiline())
        }
    }
}

impl Synth for DirectiveDefSynth {
    fn synth(&self, context: &SynthContext) -> String {
        let synth = PairSynth::top_level(
            DescriptionSynth::from(self.0.description.as_str()),
            ChainSynth({
                let mut v: Vec<Box<dyn Synth>> = vec![
                    Box::new(StringSynth(format!("directive @{}", self.0.name))),
                    Box::new(StringSynth::from(" on ")),
                    Box::new(DirectiveLocationSynth(self.0.locations.clone())),
                ];
                if self.0.is_repeatable {
                    v.insert(1, Box::new(StringSynth::from(" repeatable")));
                }
                if !self.0.arguments.is_empty() {
                    v.insert(1, Box::new(ArgumentsSynth(self.0.arguments.clone())));
                }
                v
            }),
        );
        synth.synth(&context.no_multiline())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use graphqxl_parser::Argument;

    #[test]
    fn test_most_simple_directive_def() {
        let synth = DirectiveDefSynth(DirectiveDef::build("dir").location(DirectiveLocation::Enum));
        assert_eq!(synth.synth_zero(), "directive @dir on ENUM");
    }

    #[test]
    fn test_with_description() {
        let synth = DirectiveDefSynth(
            DirectiveDef::build("dir")
                .description("my description")
                .location(DirectiveLocation::Enum),
        );
        assert_eq!(
            synth.synth_zero(),
            "\
\"my description\"
directive @dir on ENUM"
        );
    }

    #[test]
    fn test_with_description_with_args() {
        let synth = DirectiveDefSynth(
            DirectiveDef::build("dir")
                .description("my description")
                .arg(Argument::string("arg"))
                .location(DirectiveLocation::Enum),
        );
        assert_eq!(
            synth.synth_zero(),
            "\
\"my description\"
directive @dir(arg: String) on ENUM"
        );
    }

    #[test]
    fn test_with_description_with_args_repeatable() {
        let synth = DirectiveDefSynth(
            DirectiveDef::build("dir")
                .description("my description")
                .arg(Argument::string("arg"))
                .repeatable()
                .location(DirectiveLocation::Enum),
        );
        assert_eq!(
            synth.synth_zero(),
            "\
\"my description\"
directive @dir(arg: String) repeatable on ENUM"
        );
    }

    #[test]
    fn test_with_description_with_args_repeatable_multiple_locations() {
        let synth = DirectiveDefSynth(
            DirectiveDef::build("dir")
                .description("my description")
                .arg(Argument::string("arg"))
                .repeatable()
                .location(DirectiveLocation::Enum)
                .location(DirectiveLocation::ArgumentDefinition)
                .location(DirectiveLocation::Interface),
        );
        assert_eq!(
            synth.synth(&SynthContext::default().max_one_line_ors(3)),
            "\
\"my description\"
directive @dir(arg: String) repeatable on ENUM | ARGUMENT_DEFINITION | INTERFACE"
        );
    }

    #[test]
    fn test_with_description_with_args_repeatable_multiple_locations_multiline() {
        let synth = DirectiveDefSynth(
            DirectiveDef::build("dir")
                .description("my description")
                .arg(Argument::string("arg"))
                .repeatable()
                .location(DirectiveLocation::Enum)
                .location(DirectiveLocation::ArgumentDefinition)
                .location(DirectiveLocation::Interface),
        );
        assert_eq!(
            synth.synth(&SynthContext {
                indent_spaces: 2,
                multiline: true,
                ..Default::default()
            }),
            "\
\"my description\"
directive @dir(arg: String) repeatable on 
  ENUM |
  ARGUMENT_DEFINITION |
  INTERFACE"
        );
    }
}
