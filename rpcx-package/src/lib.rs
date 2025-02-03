use atlas_rpcx_bindings::program_parser::*;
use serde_json::json;

/// The rpcX package
struct Package;

/// The implementation of the rpcX package.
/// This example implements the ProgramParser interface, which is an
/// interface for parsing atomic elements of a program such as individual account or instructions.
impl ProgramParserGuest for Package {
    fn get_program_metadata(_idl: Option<AtlasAccount>) -> Option<ProgramMetadata> {
        Some(ProgramMetadata {
            name: Some("rpcX hello world example".to_string()),
            program_id: None,
            project_url: None,
            contacts: None,
            policy: None,
            source_code: None,
            encryption: None,
            auditors: None,
            acknowledgements: None,
            custom_metadata: None,
        })
    }

    fn parse_accounts(
        _idl: Option<AtlasAccount>,
        accounts: Vec<AtlasAccount>,
        _params: String,
    ) -> Result<Vec<Result<AccountResponse, String>>, String> {
        let parsed_accounts: Vec<Result<AccountResponse, String>> = accounts
            .into_iter()
            .map(|a| {
                if a.data.is_empty() {
                    return Err(format!("Account data is empty: {}", a.key));
                }

                let message = String::from_utf8(a.data).expect("Could not parse message");
                let value = serde_json::to_string(&json!({
                    "message": message
                }))
                .expect("Could not serialize message string");

                Ok(AccountResponse {
                    name: "MessageAccount".to_string(),
                    discriminator: None,
                    value,
                })
            })
            .collect();

        Ok(parsed_accounts)
    }

    fn parse_instructions(
        _idl: Option<AtlasAccount>,
        instructions: Vec<AtlasInstruction>,
        _params: String,
    ) -> Result<Vec<Result<InstructionResponse, String>>, String> {
        let parsed_instructions: Vec<Result<InstructionResponse, String>> = instructions
            .into_iter()
            .map(|ix| {
                let accounts = vec![
                    AccountMetaTags {
                        key: ix.accounts[0].clone(),
                        name: "Signer".to_string(),
                    },
                    AccountMetaTags {
                        key: ix.accounts[1].clone(),
                        name: "Message Account".to_string(),
                    },
                    AccountMetaTags {
                        key: ix.accounts[2].clone(),
                        name: "System Program".to_string(),
                    },
                ];

                Ok(InstructionResponse {
                    name: "rpcX-example".to_string(),
                    discriminator: Some(0),
                    accounts,
                    value: serde_json::to_string(&json!({}))
                        .expect("Could not serialize empty data"),
                })
            })
            .collect();

        Ok(parsed_instructions)
    }

    fn parse_error(_idl: Option<AtlasAccount>, error: u64, _params: String) -> Option<String> {
        Some(format!("error: {:?}", error))
    }

    fn parse_logs(
        _idl: Option<AtlasAccount>,
        _logs: Vec<String>,
        _params: String,
    ) -> Result<Vec<String>, String> {
        Ok(vec![])
    }
}

// Export the impl of the package that fulfills the ProgramParser interface.
// Don't forget this line or your rpcX pacakge won't work properly!
export_program_parser!(Package with_types_in bindings);
