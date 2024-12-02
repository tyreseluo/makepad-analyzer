use std::error::Error;

use lsp_types::{
    request::GotoDefinition, CompletionOptions, GotoDefinitionResponse, HoverOptions, HoverProviderCapability, InitializeParams, OneOf, ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind
};

use lsp_server::{Connection, ExtractError, Message, Request, RequestId, Response};
use makepad_lsp_server::utils::scan_workspace;
// use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Sync + Send>> {

    eprintln!("-----------------------------------> starting generic LSP server");
    // Create the transport. Includes the stdio (stdin and stdout) versions but this could
    // also be implemented to use sockets or HTTP.
    let (connection, io_threads) = Connection::stdio();
    // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).
    let server_capabilities = serde_json::to_value(&ServerCapabilities {
        completion_provider: Some(CompletionOptions::default()),
        text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
        hover_provider: Some(HoverProviderCapability::Options(HoverOptions::default())),
        definition_provider: Some(OneOf::Left(true)),
        ..Default::default()
    })
    .unwrap();

    eprintln!("wating for initialize.....");

    let initialization_params = match connection.initialize(server_capabilities) {
        Ok(it) => it,
        Err(e) => {
            if e.channel_is_disconnected() {
                io_threads.join()?;
            }
            return Err(e.into());
        }
    };

    let init_params: InitializeParams = serde_json::from_value(initialization_params)?;

    if let Some(workspace_folders) = init_params.workspace_folders {
        for folder in workspace_folders {
          //eprintln!("workspace folder: {:#?}", folder.uri);
          scan_workspace(folder.uri).await?;
        }
    }

    main_loop(connection)?;
    io_threads.join()?;

    Ok(())
}

fn main_loop(
    connection: Connection
) -> Result<(), Box<dyn Error + Sync + Send>> {

    for msg in &connection.receiver {
        eprintln!("got msg: {msg:?}");
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }
                eprintln!("got request: {req:?}");
                match cast::<GotoDefinition>(req) {
                    Ok((id, params)) => {
                        eprintln!("goto definition request: {:#?}", params);
                        let result = Some(GotoDefinitionResponse::Array(Vec::new()));
                        let result = serde_json::to_value(&result).unwrap();
                        let resp = Response { id, result: Some(result), error: None };
                        connection.sender.send(Message::Response(resp))?;
                        continue;
                    }
                    Err(err @ ExtractError::JsonError { .. }) => panic!("{err:?}"),
                    Err(ExtractError::MethodMismatch(req)) => req,
                };
                // ...
            }
            Message::Response(resp) => {
                eprintln!("got response: {resp:#?}");
            }
            Message::Notification(not) => {
                eprintln!("got notification: {not:#?}");
            }
        }
    }
    Ok(())
}

fn cast<R>(req: Request) -> Result<(RequestId, R::Params), ExtractError<Request>>
where
    R: lsp_types::request::Request,
    R::Params: serde::de::DeserializeOwned,
{
    req.extract(R::METHOD)
}
