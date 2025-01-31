use super::parser::command_builder::CommandBuilder;
use super::parser::operant::{Operant, OperantKind};
use super::ParseContext;
use crate::commands::{get_instruction_attribute, Command, Instruction, InstructionAttribute};
use crate::error::{ParseErrorBuilder, ParseErrorType};

pub(super) fn resolve<'a>(
    command_builder: &mut Vec<CommandBuilder>,
    context: &mut ParseContext<'a>,
) -> Vec<Command> {
    get_label_table(command_builder, context);
    convert_to_commands(command_builder, context)
}

fn get_label_table<'a>(command_builder: &Vec<CommandBuilder>, context: &'a mut ParseContext) {
    for (index, command) in command_builder.iter().enumerate() {
        if let Some(label) = command.label {
            let label_identifier = label.resolve(context.text);
            if let Some(_) = context.labels.get(&label_identifier) {
                context.errors.add(ParseErrorBuilder::new(
                    ParseErrorType::LabelReassign,
                    label.start,
                    label.end,
                ));
            } else {
                context.labels.insert(label_identifier, index);
            }
        }
    }
}

fn convert_to_commands<'a>(
    command_builder: &mut Vec<CommandBuilder>,
    context: &mut ParseContext,
) -> Vec<Command> {
    let mut commands: Vec<Command> = Vec::with_capacity(command_builder.len());

    for command in command_builder {
        match resolve_command(context, command) {
            Ok(command) => commands.push(command),
            Err(error) => context.errors.add(error),
        }
    }
    commands
}

fn resolve_command<'a>(
    context: &'a mut ParseContext,
    command: &CommandBuilder,
) -> Result<Command, ParseErrorBuilder> {
    let instruction_name = command.instruction.resolve(context.text).to_uppercase();
    let attributes =
        get_instruction_attribute(&instruction_name).expect("No instruction attributes found"); // Ensured
                                                                                                // to previous check in parser

    let instruction = match &command.operant {
        Some(operant) => resolve_instruction_with_operant(&instruction_name, operant)?,
        None => resolve_instruction_without_operant(command, attributes, &instruction_name)?,
    };

    let operant: usize = match &command.operant {
        Some(operant) => resolve_operant(context, operant)?,
        None => 0,
    };

    let line = context
        .line_table
        .get_line_index_of(command.instruction.start);

    Ok(Command::new(instruction, operant, line))
}

fn resolve_instruction_without_operant<'a>(
    command: &CommandBuilder,
    attributes: InstructionAttribute,
    instruction: &str,
) -> Result<Instruction, ParseErrorBuilder> {
    if attributes.allow_no_operant() {
        return match instruction {
            "NOP" => Ok(Instruction::NOP),
            _ => Err(ParseErrorBuilder::new(
                ParseErrorType::MissingOperant,
                command.instruction.start,
                command.instruction.end,
            )),
        };
    } else {
        Err(ParseErrorBuilder::new(
            ParseErrorType::MissingOperant,
            command.instruction.start,
            command.instruction.end,
        ))
    }
}

fn resolve_instruction_with_operant<'a>(
    instruction: &str,
    operant: &Operant,
) -> Result<Instruction, ParseErrorBuilder> {
    match match_instruction(instruction, operant) {
        Some(inst) => Ok(inst),
        None => Err(ParseErrorBuilder::new(
            ParseErrorType::InvalidOperant,
            operant.value.start,
            operant.value.end,
        )),
    }
}

fn match_instruction(instruction: &str, operant: &Operant) -> Option<Instruction> {
    match instruction {
        "LDA" => match operant.kind {
            OperantKind::Fixed => Some(Instruction::LoadFix),
            OperantKind::Address => Some(Instruction::LoadFromRegister),
            OperantKind::Label => None,
        },
        "STA" => match operant.kind {
            OperantKind::Fixed => None,
            OperantKind::Address => Some(Instruction::SaveToRegister),
            OperantKind::Label => None,
        },
        "ADD" => match operant.kind {
            OperantKind::Fixed => Some(Instruction::AddFix),
            OperantKind::Address => Some(Instruction::AddFromRegister),
            OperantKind::Label => None,
        },
        "SUB" => match operant.kind {
            OperantKind::Fixed => Some(Instruction::SubFix),
            OperantKind::Address => Some(Instruction::SubFromRegister),
            OperantKind::Label => None,
        },
        "JMP" => match operant.kind {
            OperantKind::Fixed => Some(Instruction::JMP),
            OperantKind::Address => None,
            OperantKind::Label => Some(Instruction::JMP),
        },
        "BRZ" => match operant.kind {
            OperantKind::Fixed => Some(Instruction::BRZ),
            OperantKind::Address => None,
            OperantKind::Label => None,
        },
        "BRC" => match operant.kind {
            OperantKind::Fixed => Some(Instruction::BRC),
            OperantKind::Address => None,
            OperantKind::Label => None,
        },
        "BRN" => match operant.kind {
            OperantKind::Fixed => Some(Instruction::BRN),
            OperantKind::Address => None,
            OperantKind::Label => None,
        },
        _ => None,
    }
}

fn resolve_operant(
    context: &mut ParseContext,
    operant: &Operant,
) -> Result<usize, ParseErrorBuilder> {
    Ok(match operant.kind {
        OperantKind::Fixed | OperantKind::Address => operant
            .value
            .resolve(context.text)
            .parse::<usize>()
            .map_err(|_| {
                ParseErrorBuilder::new(
                    ParseErrorType::InvalidOperant,
                    operant.value.start,
                    operant.value.end,
                )
            })?,
        OperantKind::Label => *context
            .labels
            .get(operant.value.resolve(context.text))
            .ok_or(ParseErrorBuilder::new(
                ParseErrorType::MissingLabel,
                operant.value.start,
                operant.value.end,
            ))?,
    })
}
