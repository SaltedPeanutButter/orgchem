# Instructions for 'reactions.json'

The file `reactions.json` contains all the reactions (and their accompanying information) that OrgChem can interpret.
This README acts as an instructions on how to add/remove reactions **without** editing the code.

OrgChem embeds the content of `reactions.json` on building. Hence, changing the content of this file does not affect
existing executables.

## What can I do?

### Adding reaction types and compound types

Unfortunately, adding reaction types and compound types are **NOT** supported by editing `reactions.json`. These are
custom types, specifically `enum`, written as part of the code. Hence, code modification is required and more stringent
requirements are imposed (for testing purposes).

The `CompoundType` and `ReactionType` enums are defined as follows:

```rust
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum CompoundType {
    Inorganic,
    Alkane,
    Alkene,
    Arene,
    Alkylbenzene,
    Carboxylic,
    Alcohol,
    Halogenoalkane,
    Aldehyde,
    Ketone,
    Nitro,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum ReactionType {
    FreeRadicalSubstitution,
    ElectrophilicAddition,
    Reduction,
    Oxidation,
    ElectrophilicSubstitution,
    NucleophilicSubstitution,
}
```

Adding new variants to the enums involved:

1. Adding the actual variant

2. Edit methods in `impl Display` and `impl`

3. Editing existing help messages, if applicable

### Adding new reactions for existing reaction types and compound types

The `Reaction` struct is defined as follows:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Reaction {
    pub from: CompoundType,
    pub to: CompoundType,
    pub reaction_type: ReactionType,
    pub reagents_conditions: String,
    pub example: String,
    pub observation: Option<String>,
    pub additional_information: Option<String>,
}
```

Note that, the fields `observation` and `additional_information` are optional.

On embedding, the file will be read as a JSON string and parsed as a `Vec<Reaction>`. Hence, adding another
reaction is trivial. Simply add another object in the array, like this:

```json
{
    "from": "CompoundType",
    "to": "CompoundType",
    "reaction_type": "ReactionType",
    "reagents_conditions": "....",
    "example": "....",
    "observation": null,
    "additional_information": null
}
```