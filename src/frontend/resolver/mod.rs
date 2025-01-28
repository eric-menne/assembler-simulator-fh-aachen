use std::collections::HashMap;

use super::parser::command_builder::CommandBuilder;
use super::parser::operant::OperantKind;
use super::LineTable;
use super::ParseErrorReportBuilder;
use crate::commands::{Command, Instruction};
use crate::error::{ParseErrorBuilder, ParseErrorType};

type LabelTable<'a> = HashMap<&'a str, usize>;

pub(super) fn resolve<'a>(
    text: &'a str,
    command_builder: &mut Vec<CommandBuilder>,
    error_report: &mut ParseErrorReportBuilder,
    line_table: &mut LineTable,
) -> Vec<Command> {
    let label_table = get_label_table(text, command_builder, error_report);
    convert_to_commands(
        text,
        command_builder,
        &label_table,
        error_report,
        line_table,
    )
}

fn get_label_table<'a>(
    text: &'a str,
    command_builder: &Vec<CommandBuilder>,
    error_report: &mut ParseErrorReportBuilder,
) -> LabelTable<'a> {
    let mut label_table = HashMap::new();

    for (index, command) in command_builder.iter().enumerate() {
        if let Some(label) = command.label {
            let label_identifier = label.resolve(text);
            if let Some(_) = label_table.get(&label_identifier) {
                error_report.add(ParseErrorBuilder::new(
                    ParseErrorType::LabelReassign,
                    label.start,
                    label.end,
                ));
            } else {
                label_table.insert(label_identifier, index);
            }
        }
    }
    label_table
}

fn convert_to_commands<'a>(
    text: &'a str,
    command_builder: &mut Vec<CommandBuilder>,
    label_table: &LabelTable,
    error_report: &mut ParseErrorReportBuilder,
    line_table: &mut LineTable,
) -> Vec<Command> {
    let mut commands: Vec<Command> = Vec::with_capacity(command_builder.len());

    for command in command_builder {
        match translate_command(text, command, &label_table, line_table) {
            Ok(command) => commands.push(command),
            Err(error) => error_report.add(error),
        }
    }
    commands
}

fn translate_command<'a>(
    text: &'a str,
    command: &mut CommandBuilder,
    label_table: &LabelTable,
    line_table: &mut LineTable,
) -> Result<Command, ParseErrorBuilder> {
    let instruction = command.instruction.resolve(text).to_uppercase();

    let operant: usize = resolve_operant(text, command, label_table)?;

    match instruction.as_str() {
        "NOP" => Ok(Command::new(
            Instruction::NOP,
            0,
            line_table.get_line_index_of(command.instruction.start),
        )),
        "LDA" => translate_load_command(command, operant, line_table),
        "STA" => translate_store_command(command, operant, line_table),
        "ADD" => translate_add_command(command, operant, line_table),
        "SUB" => translate_sub_command(command, operant, line_table),
        "JMP" => translate_jmp_command(command, operant, line_table),
        "BRZ" => translate_brz_command(command, operant, line_table),
        "BRC" => translate_brc_command(command, operant, line_table),
        "BRN" => translate_brn_command(command, operant, line_table),
        _ => Err(ParseErrorBuilder::new(
            ParseErrorType::InvalidInstruction,
            command.instruction.start,
            command.instruction.end,
        )),
    }
}

fn resolve_operant<'a>(
    text: &'a str,
    command: &CommandBuilder,
    label_table: &LabelTable,
) -> Result<usize, ParseErrorBuilder> {
    let operant_value = command.operant.value.resolve(text);

    if command.operant.kind == OperantKind::Label {
        if let Some(address) = label_table.get(operant_value) {
            return Ok(*address);
        } else {
            return Err(ParseErrorBuilder::new(
                ParseErrorType::MissingLabel,
                command.operant.value.start,
                command.operant.value.end,
            ));
        }
    } else {
        operant_value.parse::<usize>().map_err(|_| {
            ParseErrorBuilder::new(
                ParseErrorType::InvalidOperant,
                command.operant.value.start,
                command.operant.value.end,
            )
        })
    }
}

/// Handles the translation to an executable command for 'LDA' instructions
/// Returns an error if a label is used as operant
fn translate_load_command<'a>(
    command: &mut CommandBuilder,
    operant: usize,
    line_table: &mut LineTable,
) -> Result<Command, ParseErrorBuilder> {
    Ok(match command.operant.kind {
        OperantKind::Fixed => Ok(Command::new(
            Instruction::LoadFix,
            operant,
            line_table.get_line_index_of(command.instruction.start),
        )),
        OperantKind::Address => Ok(Command::new(
            Instruction::LoadFromRegister,
            operant,
            line_table.get_line_index_of(command.instruction.start),
        )),
        OperantKind::Label => Err(ParseErrorBuilder::new(
            ParseErrorType::NotAllowedLabel,
            command.operant.value.start,
            command.operant.value.end,
        )),
    }?)
}

/// Handles the translation to an executable command for 'STA' instructions
/// Returns an error if a fixed number or a label is used as operant
fn translate_store_command<'a>(
    command: &mut CommandBuilder,
    operant: usize,
    line_table: &mut LineTable,
) -> Result<Command, ParseErrorBuilder> {
    Ok(match command.operant.kind {
        OperantKind::Fixed => Err(ParseErrorBuilder::new(
            ParseErrorType::NotAllowedFixNumber,
            command.operant.value.start,
            command.operant.value.end,
        ))?,
        OperantKind::Address => Command::new(
            Instruction::SaveToRegister,
            operant,
            line_table.get_line_index_of(command.instruction.start),
        ),
        OperantKind::Label => Err(ParseErrorBuilder::new(
            ParseErrorType::NotAllowedLabel,
            command.operant.value.start,
            command.operant.value.end,
        ))?,
    })
}

// Handles the translation of an executable command for 'ADD' instruction
// Returns an error if a label is used as operant
fn translate_add_command<'a>(
    command: &mut CommandBuilder,
    operant: usize,
    line_table: &mut LineTable,
) -> Result<Command, ParseErrorBuilder> {
    Ok(match command.operant.kind {
        OperantKind::Fixed => Command::new(
            Instruction::AddFix,
            operant,
            line_table.get_line_index_of(command.instruction.start),
        ),
        OperantKind::Address => Command::new(
            Instruction::AddFromRegister,
            operant,
            line_table.get_line_index_of(command.instruction.start),
        ),
        OperantKind::Label => Err(ParseErrorBuilder::new(
            ParseErrorType::NotAllowedLabel,
            command.operant.value.start,
            command.operant.value.end,
        ))?,
    })
}

// Handles the translation of an executable command for 'SUB' instruction
// Returns an error if a label is used as operant
fn translate_sub_command<'a>(
    command: &mut CommandBuilder,
    operant: usize,
    line_table: &mut LineTable,
) -> Result<Command, ParseErrorBuilder> {
    Ok(match command.operant.kind {
        OperantKind::Fixed => Command::new(
            Instruction::SubFix,
            operant,
            line_table.get_line_index_of(command.instruction.start),
        ),
        OperantKind::Address => Command::new(
            Instruction::SubFromRegister,
            operant,
            line_table.get_line_index_of(command.instruction.start),
        ),
        OperantKind::Label => Err(ParseErrorBuilder::new(
            ParseErrorType::NotAllowedLabel,
            command.operant.value.start,
            command.operant.value.end,
        ))?,
    })
}

// Handles the translation of an executable command for 'JMP' instruction
// Returns an error if a address is used as operant
fn translate_jmp_command<'a>(
    command: &mut CommandBuilder,
    operant: usize,
    line_table: &mut LineTable,
) -> Result<Command, ParseErrorBuilder> {
    Ok(match command.operant.kind {
        OperantKind::Fixed => Command::new(
            Instruction::JMP,
            operant,
            line_table.get_line_index_of(command.instruction.start),
        ),
        OperantKind::Address => Err(ParseErrorBuilder::new(
            ParseErrorType::NotAllowedAddress,
            command.operant.value.start,
            command.operant.value.end,
        ))?,
        OperantKind::Label => Command::new(
            Instruction::JMP,
            operant,
            line_table.get_line_index_of(command.instruction.start),
        ),
    })
}
// Handles the translation of an executable command for 'BRZ' instruction
// Returns an error if a label or address is used as operant
fn translate_brz_command<'a>(
    command: &mut CommandBuilder,
    operant: usize,
    line_table: &mut LineTable,
) -> Result<Command, ParseErrorBuilder> {
    Ok(match command.operant.kind {
        OperantKind::Fixed => Command::new(
            Instruction::BRZ,
            operant,
            line_table.get_line_index_of(command.instruction.start),
        ),
        OperantKind::Address => Err(ParseErrorBuilder::new(
            ParseErrorType::NotAllowedAddress,
            command.operant.value.start,
            command.operant.value.end,
        ))?,
        OperantKind::Label => Err(ParseErrorBuilder::new(
            ParseErrorType::NotAllowedLabel,
            command.operant.value.start,
            command.operant.value.end,
        ))?,
    })
}

// Handles the translation of an executable command for 'BRC' instruction
// Returns an error if a label or address is used as operant
fn translate_brc_command<'a>(
    command: &mut CommandBuilder,
    operant: usize,
    line_table: &mut LineTable,
) -> Result<Command, ParseErrorBuilder> {
    Ok(match command.operant.kind {
        OperantKind::Fixed => Command::new(
            Instruction::BRC,
            operant,
            line_table.get_line_index_of(command.instruction.start),
        ),
        OperantKind::Address => Err(ParseErrorBuilder::new(
            ParseErrorType::NotAllowedAddress,
            command.operant.value.start,
            command.operant.value.end,
        ))?,
        OperantKind::Label => Err(ParseErrorBuilder::new(
            ParseErrorType::NotAllowedLabel,
            command.operant.value.start,
            command.operant.value.end,
        ))?,
    })
}

// Handles the translation of an executable command for 'BRN' instruction
// Returns an error if a label or address is used as operant
fn translate_brn_command<'a>(
    command: &mut CommandBuilder,
    operant: usize,
    line_table: &mut LineTable,
) -> Result<Command, ParseErrorBuilder> {
    Ok(match command.operant.kind {
        OperantKind::Fixed => Command::new(
            Instruction::BRN,
            operant,
            line_table.get_line_index_of(command.instruction.start),
        ),
        OperantKind::Address => Err(ParseErrorBuilder::new(
            ParseErrorType::NotAllowedAddress,
            command.operant.value.start,
            command.operant.value.end,
        ))?,
        OperantKind::Label => Err(ParseErrorBuilder::new(
            ParseErrorType::NotAllowedLabel,
            command.operant.value.start,
            command.operant.value.end,
        ))?,
    })
}
