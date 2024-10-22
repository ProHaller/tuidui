// TODO: Create assistant
// TODO: Define functions based on assest json files
// ISSUE: How to keep the different AI logics separate?
// TODO: Create the task post-processing logic
// TODO: Create the user creation logic

use std::path::PathBuf;

use anyhow::{anyhow, Error};
use async_openai::*;
use colored::Colorize;
use config::OpenAIConfig;
use include_dir::{include_dir, Dir, DirEntry};
use serde_json::{from_str, Value};
use types::{
    AssistantObject, AssistantTools, AssistantsApiResponseFormatOption, CreateAssistantRequestArgs,
    CreateThreadRequestArgs, FunctionObject, ListAssistantsResponse, ListModelResponse,
    MessageContent, MessageObject, ResponseFormat, ResponseFormatJsonSchema, RunObject, RunStatus,
    ThreadObject, ToolsOutputs,
};

use crate::{
    save::save_tasks,
    task::{Task, Tasks},
};

static ASSETS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/assets");

pub async fn initialize() -> Result<Client<OpenAIConfig>, Error> {
    let client = Client::new();

    let test_client = client.clone();
    match check_api_key(&test_client).await {
        Ok(_) => println!("API Key is valid"),
        Err(e) => eprintln!("Error: {}", e),
    };
    Ok(client)
}

async fn check_api_key(client: &Client<OpenAIConfig>) -> Result<ListModelResponse, Error> {
    let models_list = client.models().list().await?;
    Ok(models_list)
}

fn load_function_objects() -> Result<Vec<FunctionObject>, Error> {
    let folder_dir = ASSETS_DIR
        .get_dir("assistants/task_creator/functions")
        .expect("Failed to get assistant_functions directory");

    let mut function_objects = Vec::new();

    // Read the folder
    for entry in folder_dir.entries() {
        match entry {
            DirEntry::File(file) => {
                let path = file.path();

                // Ensure the entry is a JSON file
                if path.extension().and_then(|ext| ext.to_str()) == Some("json") {
                    // Read the file contents
                    let content = file
                        .contents_utf8()
                        .ok_or_else(|| anyhow!("File content is not valid UTF-8"))?;

                    // Parse the content as a JSON value
                    let function_data: Value = serde_json::from_str(content)?;

                    // Extract relevant fields from the JSON object
                    let name = function_data["name"].as_str().unwrap_or_default();
                    let description = function_data["description"].as_str().unwrap_or_default();
                    let parameters = function_data["parameters"].clone(); // This extracts the parameters part
                    let strict = function_data["strict"].as_bool().unwrap_or(false); // Defaults to true if not found

                    // Create a FunctionObject and push it to the vector
                    let function_object = FunctionObject {
                        name: name.to_string(),
                        description: Some(description.to_string()),
                        parameters: Some(parameters), // Use the extracted parameters
                        strict: Some(strict),
                    };
                    function_objects.push(function_object);
                }
            }
            DirEntry::Dir(_) => {
                // Optionally handle subdirectories if needed
            }
        }
    }
    Ok(function_objects)
}

fn load_assistant_def(path: PathBuf) -> Result<Value, Error> {
    let assistant_value: Value = serde_json::from_str(
        ASSETS_DIR
            .get_file(path)
            .ok_or_else(|| anyhow!("Failed to get assistant file"))?
            .contents_utf8()
            .ok_or_else(|| anyhow!("Failed to read assistant file"))?,
    )?;
    Ok(assistant_value)
}

fn get_assistant_value(name: &str) -> Result<Value, Error> {
    let assistant_path = PathBuf::from(format!("assistants/{name}.json"));
    let assistant_value = load_assistant_def(assistant_path)?;
    Ok(assistant_value)
}

fn get_json_schema(value: Value) -> Result<ResponseFormatJsonSchema, Error> {
    let json_schema_value = value["json_schema"].clone();
    let json_schema = ResponseFormatJsonSchema {
        name: json_schema_value["name"].to_string(),
        description: Some(json_schema_value["description"].to_string()),
        strict: json_schema_value["strict"].as_bool(),
        schema: Some(json_schema_value["schema"].clone()),
    };
    Ok(json_schema)
}

pub async fn create_assistant(
    assistant_name: &str,
    client: &Client<OpenAIConfig>,
) -> Result<AssistantObject, Error> {
    let assistant_value = get_assistant_value(assistant_name)?;
    let name = assistant_value["name"].as_str().unwrap_or_default();
    let description = assistant_value["description"].as_str().unwrap_or_default();
    let instructions = assistant_value["instructions"].as_str().unwrap_or_default();
    let model = assistant_value["model"].as_str().unwrap_or_default();
    let temperature = assistant_value["temperature"].as_f64().unwrap_or_default();
    let top_p = assistant_value["top_p"].as_f64().unwrap_or_default();
    let function_objects = load_function_objects()?;
    let assistant_tools = function_objects
        .into_iter()
        .map(Into::into) // Use the Into trait for conversion
        .collect::<Vec<AssistantTools>>();
    let response_format = match assistant_value["response_format"].as_str() {
        Some("JsonObject") => ResponseFormat::JsonObject,
        Some("Text") => ResponseFormat::Text,
        Some("JsonSchema") => ResponseFormat::JsonSchema {
            json_schema: get_json_schema(assistant_value.clone()).unwrap(),
        },
        _ => ResponseFormat::Text,
    };
    let create_assistant_request = CreateAssistantRequestArgs::default()
        .name(name)
        .description(description)
        .temperature(temperature as f32)
        .top_p(top_p as f32)
        .instructions(instructions)
        .model(model)
        .response_format(AssistantsApiResponseFormatOption::Format(response_format))
        .tools(assistant_tools) // Pass the vector of AssistantTools
        .build()?;

    if let Ok(assistant) = client.assistants().create(create_assistant_request).await {
        Ok(assistant)
    } else {
        Err(anyhow::anyhow!("Failed to create assistant"))
    }
}

pub async fn list_assistants(
    client: &Client<OpenAIConfig>,
) -> Result<ListAssistantsResponse, Error> {
    let response = client
        .assistants()
        .list(&())
        .await
        .map_err(|e| anyhow::anyhow!(e))?;

    Ok(response)
}

pub async fn delete_assistant(assistant: AssistantObject, client: &Client<OpenAIConfig>) {
    let _ = client.assistants().delete(&assistant.id).await;
    println!(
        "{}",
        format!(
            "Assistant deleted: {}",
            assistant.name.unwrap_or_else(|| assistant.id.to_string())
        )
        .dimmed()
    );
}

pub async fn create_thread(client: &Client<OpenAIConfig>) -> Result<ThreadObject, Error> {
    let thread = client
        .threads()
        .create(CreateThreadRequestArgs::default().build()?)
        .await?;
    Ok(thread)
}

pub async fn cancel_run(
    client: &Client<OpenAIConfig>,
    run: &RunObject,
) -> Result<RunObject, Error> {
    let response = client
        .threads()
        .runs(&run.thread_id)
        .cancel(&run.id)
        .await?;
    Ok(response)
}

pub async fn poll_run(
    client: &Client<OpenAIConfig>,
    run: &RunObject,
) -> Result<Option<MessageObject>, Error> {
    let mut awaiting_response = true;
    while awaiting_response {
        //retrieve the run
        let run = client
            .threads()
            .runs(&run.thread_id)
            .retrieve(&run.id)
            .await?;
        let query = [("limit", "1")];
        //check the status of the run
        if let Some(message) = match run.status {
            RunStatus::Completed => {
                awaiting_response = false;
                // once the run is completed we
                // get the response from the run
                // which will be the first message
                // in the thread

                //retrieve the response from the run
                let response = client
                    .threads()
                    .messages(&run.thread_id)
                    .list(&query)
                    .await?;
                //get the message id from the response
                let message_id = response.data.first().unwrap().id.clone();
                //get the message from the response
                let message = client
                    .threads()
                    .messages(&run.thread_id)
                    .retrieve(&message_id)
                    .await?;
                //get the content from the message
                let content = message.content.first().unwrap();
                //get the text from the content
                let text = match content {
                    MessageContent::Text(text) => text.text.value.clone(),
                    MessageContent::ImageFile(_) | MessageContent::ImageUrl(_) => {
                        panic!("imaged are not expected in this example");
                    }
                    MessageContent::Refusal(refusal) => refusal.refusal.clone(),
                };
                //print the text
                println!("{}", format!("--- Response: {}\n", text).dimmed());
                Some(message)
            }
            RunStatus::Failed => {
                awaiting_response = false;
                println!("--- Run Failed: {:#?}", run);
                None
            }
            RunStatus::Queued => {
                println!("--- Run Queued");
                None
            }
            RunStatus::Cancelling => {
                println!("--- Run Cancelling");
                None
            }
            RunStatus::Cancelled => {
                println!("--- Run Cancelled");
                None
            }
            RunStatus::Expired => {
                println!("--- Run Expired");
                None
            }
            RunStatus::RequiresAction => {
                println!(
                    "{}",
                    format!("--- Run Requires Action: {:#?}", run.required_action).dimmed()
                );
                handle_requires_action(client, &run).await?;
                None
            }
            RunStatus::InProgress => {
                println!("--- In Progress ...");
                None
            }
            RunStatus::Incomplete => {
                println!("--- Run Incomplete");
                None
            }
        } {
            return Ok(Some(message));
        }
        //wait for 1 second before checking the status again
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
    Ok(None)
}

async fn handle_requires_action(
    client: &Client<OpenAIConfig>,
    run: &RunObject,
) -> Result<(), anyhow::Error> {
    let mut tool_outputs: Vec<ToolsOutputs> = vec![];
    if let Some(ref required_actions) = run.required_action {
        let mut save: Tasks = Tasks::new();
        for tool in &required_actions.submit_tool_outputs.tool_calls {
            let task: Task = from_str(&tool.function.arguments).expect("Invalid JSON response");
            save.tasks.push(task.clone());
            tool_outputs.push(ToolsOutputs {
                tool_call_id: Some(tool.id.clone()),
                output: Some(task.title),
            })
        }
        save_tasks(&save)?;
        submit_tool_outputs(client, run, tool_outputs).await?;
    }
    Ok(())
}

async fn submit_tool_outputs(
    client: &Client<OpenAIConfig>,
    run: &RunObject,
    tool_outputs: Vec<ToolsOutputs>,
) -> Result<(), Error> {
    let _ = client
        .threads()
        .runs(&run.thread_id)
        .submit_tool_outputs(
            &run.id,
            types::SubmitToolOutputsRunRequest {
                tool_outputs,
                stream: Some(false),
            },
        )
        .await?;
    Ok(())
}
