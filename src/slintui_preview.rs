use zed_extension_api::{
    self as zed, KeyValueStore, SlashCommand, SlashCommandArgumentCompletion, SlashCommandOutput,
    SlashCommandOutputSection, Worktree,
};

struct SlintUIPreviewExtension;

fn create_list_of_slint_files() -> Optional<Vec<KeyValueStore>> {
    // must find all files terminated with .slint inside project dir
    // if it finds adds a new item to a vector of valid file names pointing
    // to its correponding path

    let vec = Vec<KeyValueStore>::new();
    let worktree = worktree.ok_or("no worktree available")?;
    let root = worktree.root_path();
    println!(root);
    None
}

impl Extension for SlintUIPreviewExtension {
    fn new() -> Self {
        SlintUIPreviewExtension
    }

    fn complete_slash_command_argument(
        &self,
        command: SlashCommand,
        _args: Vec<String>,
    ) -> Result<Vec<zed_extension_api::SlashCommandArgumentCompletion>, String> {
        match command.name.as_str() {
            "preview-current" => Ok(vec![]),
            "pick-one" => Ok(vec![
                SlashCommandArgumentCompletion {
                    label: "Option One".to_string(),
                    new_text: "option-1".to_string(),
                    run_command: true,
                },
                SlashCommandArgumentCompletion {
                    label: "Option Two".to_string(),
                    new_text: "option-2".to_string(),
                    run_command: true,
                },
                SlashCommandArgumentCompletion {
                    label: "Option Three".to_string(),
                    new_text: "option-3".to_string(),
                    run_command: true,
                },
            ]),
            command => Err(format!("unknown slash command: \"{command}\"")),
        }
    }

    fn run_slash_command(
        &self,
        command: SlashCommand,
        args: Vec<String>,
        _worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        match command.name.as_str() {
            "preview-current" => {
                if args.is_empty() {
                    return Err("nothing to echo".to_string());
                }

                let text = args.join(" ");

                Ok(SlashCommandOutput {
                    sections: vec![SlashCommandOutputSection {
                        range: (0..text.len()).into(),
                        label: "Echo".to_string(),
                    }],
                    text,
                })
            }
            "pick-one" => {
                let Some(selection) = args.first() else {
                    return Err("no option selected. You should select a slint file".to_string());
                };

                match selection.as_str() {
                    "option-1" | "option-2" | "option-3" => {}
                    invalid_option => {
                        return Err(format!("{invalid_option} is not a valid option"));
                    }
                }

                let text = format!("You chose {selection}.");

                Ok(SlashCommandOutput {
                    sections: vec![SlashCommandOutputSection {
                        range: (0..text.len()).into(),
                        label: format!("Pick One: {selection}"),
                    }],
                    text,
                })
            }
            command => Err(format!("unknown slash command: \"{command}\"")),
        }
    }
}

zed::register_extension!(SlintUIPreviewExtension);
