use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    cli::{ListArgs, ShowArgs},
    fmt,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum ReactionType {
    FreeRadicalSubstitution,
    ElectrophilicAddition,
    Reduction,
    Oxidation,
    ElectrophilicSubstitution,
    NucleophilicSubstitution,
}

impl Display for ReactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FreeRadicalSubstitution => write!(f, "Free Radical Substitution"),
            Self::ElectrophilicAddition => write!(f, "Electrophilic Addition"),
            Self::ElectrophilicSubstitution => write!(f, "Electrophilic Substitution"),
            Self::Oxidation => write!(f, "Oxidation"),
            Self::Reduction => write!(f, "Reduction"),
            Self::NucleophilicSubstitution => write!(f, "Nucleophilic Substitution"),
        }
    }
}

impl ReactionType {
    pub fn short(&self) -> &'static str {
        match self {
            Self::FreeRadicalSubstitution => "frs",
            Self::ElectrophilicAddition => "eadd",
            Self::ElectrophilicSubstitution => "esub",
            Self::Oxidation => "oxi",
            Self::Reduction => "red",
            Self::NucleophilicSubstitution => "nsub",
        }
    }

    pub fn get_variants() -> Vec<ReactionType> {
        vec![
            ReactionType::FreeRadicalSubstitution,
            ReactionType::ElectrophilicAddition,
            ReactionType::ElectrophilicSubstitution,
            ReactionType::Oxidation,
            ReactionType::Reduction,
            ReactionType::NucleophilicSubstitution,
        ]
    }

    #[allow(dead_code)]
    pub fn get_names() -> Vec<String> {
        ReactionType::get_variants()
            .into_iter()
            .map(|x| x.to_string())
            .collect()
    }

    pub fn from_short(name: &str) -> Option<ReactionType> {
        match name {
            "frs" => Some(ReactionType::FreeRadicalSubstitution),
            "eadd" => Some(ReactionType::ElectrophilicAddition),
            "esub" => Some(ReactionType::ElectrophilicSubstitution),
            "oxi" => Some(ReactionType::Oxidation),
            "red" => Some(ReactionType::Reduction),
            "nsub" => Some(ReactionType::NucleophilicSubstitution),
            _ => None,
        }
    }
}

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
    Nitrile,
}

impl Display for CompoundType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompoundType::Inorganic => write!(f, "inorganic"),
            CompoundType::Alkane => write!(f, "alkane"),
            CompoundType::Alkene => write!(f, "alkene"),
            CompoundType::Arene => write!(f, "arene"),
            CompoundType::Alkylbenzene => write!(f, "alkylbenzene"),
            CompoundType::Carboxylic => write!(f, "carboxylic"),
            CompoundType::Alcohol => write!(f, "alcohol"),
            CompoundType::Halogenoalkane => write!(f, "halogenoalkane"),
            CompoundType::Aldehyde => write!(f, "aldehyde"),
            CompoundType::Ketone => write!(f, "ketone"),
            CompoundType::Nitro => write!(f, "nitro"),
            CompoundType::Nitrile => write!(f, "nitrile"),
        }
    }
}

impl CompoundType {
    pub fn get_variants() -> Vec<CompoundType> {
        vec![
            // Inorganic compound variant is ignored
            CompoundType::Alkane,
            CompoundType::Alkene,
            CompoundType::Arene,
            CompoundType::Alkylbenzene,
            CompoundType::Carboxylic,
            CompoundType::Alcohol,
            CompoundType::Halogenoalkane,
            CompoundType::Aldehyde,
            CompoundType::Ketone,
            CompoundType::Nitro,
            CompoundType::Nitrile,
        ]
    }

    pub fn get_names() -> Vec<String> {
        CompoundType::get_variants()
            .into_iter()
            .map(|x| x.to_string())
            .collect()
    }

    pub fn from(name: &str) -> Option<CompoundType> {
        match name {
            "inorganic" => Some(CompoundType::Inorganic),
            "alkane" => Some(CompoundType::Alkane),
            "alkene" => Some(CompoundType::Alkene),
            "arene" => Some(CompoundType::Arene),
            "alkylbenzene" => Some(CompoundType::Alkene),
            "carboxylic" => Some(CompoundType::Carboxylic),
            "alcohol" => Some(CompoundType::Alcohol),
            "halogenoalkane" => Some(CompoundType::Halogenoalkane),
            "aldehyde" => Some(CompoundType::Aldehyde),
            "ketone" => Some(CompoundType::Ketone),
            "nitro" => Some(CompoundType::Nitro),
            "nitrile" => Some(CompoundType::Nitrile),
            _ => None,
        }
    }

    pub fn get_reactions(&self) -> Vec<ReactionType> {
        match self {
            CompoundType::Alkane => vec![ReactionType::FreeRadicalSubstitution],
            CompoundType::Alkene => vec![
                ReactionType::Reduction,
                ReactionType::ElectrophilicAddition,
                ReactionType::Oxidation,
            ],
            CompoundType::Arene => vec![
                ReactionType::Reduction,
                ReactionType::ElectrophilicSubstitution,
            ],
            _ => vec![],
        }
    }
}

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

impl Display for Reaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}\n{} {}\n{} {}\n{} {}\n{} {}\n{} {}",
            fmt::italic("Mechanism:"),
            self.reaction_type,
            fmt::italic("Product:"),
            fmt::capitalise(&self.to.to_string()),
            fmt::italic("Reagents/Conditions:"),
            self.reagents_conditions,
            fmt::italic("Example:"),
            self.example,
            fmt::italic("Observation:"),
            self.observation.clone().unwrap_or("None".to_string()),
            fmt::italic("Additional information:"),
            self.additional_information
                .clone()
                .unwrap_or("None".to_string()),
        )
    }
}

#[derive(Debug)]
pub struct ShowCommand {}
impl ShowCommand {
    pub fn execute(args: &ShowArgs) -> u8 {
        // Match compound
        let compound = match CompoundType::from(&args.compound) {
            Some(c) => c,
            None => {
                // If compound doesn't match, ask user to run 'orgchem list'
                eprintln!("Unknown compound: {}", &args.compound);
                println!("Run 'orgchem list' to see all available compounds and reactions.");
                return 1;
            }
        };

        // Now that compound is matched, reaction type(s) will be matched next.
        let valid_reactions = match &args.reactions {
            // Check if user passed in any reaction types
            Some(v) => {
                // If they specified at least a reaction type, check if 'all' is one of them
                // Check for 'all'
                if v.into_iter().any(|x| x == "all") {
                    // If 'all' was specified, get all the applicable reactions
                    compound.get_reactions()
                } else {
                    // Otherwise, match all the reaction types that were passed in and emit
                    // error when it doesn't match.
                    //
                    // For all matched reaction types, check if the reaction type is supported,
                    // i.e. the specified compound has the characteristic reaction type.
                    v.into_iter()
                        // Convert all the short reaction types to enum variants
                        .map(|x| {
                            let reaction = ReactionType::from_short(x);

                            // Emit error if doesn't match
                            if reaction.is_none() {
                                eprintln!("Unknown reaction type '{}'", x);
                            }
                            reaction
                        })

                        // Filter all the matched reaction types, i.e. filter out all the None value
                        .filter(|x| x.is_some())

                        // Unwrap Option into ReactionType
                        .map(|x| x.unwrap())

                        // Filter again, by checking against the list of applicable reactions
                        .filter(|x| {
                            if compound.get_reactions().contains(x) {
                                true
                            } else {
                                eprintln!("Inapplicable reaction type '{}'", x);
                                false
                            }
                        })
                        .collect()
                }
            }
            // Show all applicable reactions if no reaction types is supplied
            None => compound.get_reactions(),
        };

        // Check if there is any applicable reaction left after filtering
        if valid_reactions.len() == 0 {
            return 1;
        }

        // Loop through each valid reaction type and look up all the relevant reactions
        let mut all_reactions: Vec<Reaction> = vec![];
        for reaction in valid_reactions {
            // Get all reactions
            let mut reactions: Vec<Reaction> = get_reaction_pairs()
                .into_iter()

                // Filter by compound type and reaction type
                .filter(|x| x.from == compound && x.reaction_type == reaction)
                .collect();

            // Add to the list of reactions
            all_reactions.append(&mut reactions);
        }

        // Print all reactions
        for (idx, reaction) in all_reactions.into_iter().enumerate() {
            let msg = fmt::underline(&fmt::bold(&format!("Reaction {}:", idx + 1)));
            println!("{}\n{}\n", msg, reaction);
        }
        0
    }
}

#[derive(Debug)]
pub struct ListCommand {}
impl ListCommand {
    pub fn execute(args: &ListArgs) -> u8 {
        ListCommand::print_intro();

        if args.reaction == args.compound {
            ListCommand::print_reaction();
            ListCommand::print_compound();
        } else if args.reaction {
            ListCommand::print_reaction();
        } else if args.compound {
            ListCommand::print_compound();
        }

        0
    }

    fn print_intro() {
        println!("Stuff that OrgChem can help you with:");
        println!("(Phrases inside the round brackets are what OrgChem understands. Don't worry, it is case-insensitive.)");
    }

    fn print_compound() {
        println!("{}", fmt::bold(&fmt::underline("\nOrganic Compounds:\n")));
        for (idx, compound) in CompoundType::get_names().into_iter().enumerate() {
            println!("{}. {} ({})", idx + 1, fmt::capitalise(&compound), compound,)
        }
    }

    fn print_reaction() {
        println!("{}", fmt::bold(&fmt::underline("\nReactions:\n")));
        for (idx, reaction) in ReactionType::get_variants().into_iter().enumerate() {
            println!("{}. {} ({})", idx + 1, reaction, reaction.short(),)
        }
    }
}

pub fn get_reaction_pairs() -> Vec<Reaction> {
    let data = include_str!("../data/reactions.json");
    serde_json::from_str(data).unwrap()
}

pub fn no_command() -> u8 {
    eprintln!("No command was supplied!");
    let msg = concat!(
        "Here's something you can ask me:\n",
        "1. Show the list of things that I know: orgchem list\n",
        "2. Characteristic reactions of alkanes: orgchem show alkane\n",
        "3. Nucleophilic substitution of halogenoalkanes: orgchem show halogenoalkane -r nsub\n",
    );
    println!("{}", msg);
    1
}