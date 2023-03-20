use flowsnet_platform_sdk::write_error_log;
use github_flows::{get_octo, listen_to_event, octocrab::models::events::payload::EventPayload};
use openai_flows::{chat_completion, ChatOptions, ChatModel};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    listen_to_event(
        "second-state",
        "chat-with-chatgpt",
        vec!["issue_comment", "issues"],
        handler,
    )
    .await;
}

async fn handler(payload: EventPayload) {
    let octo = get_octo(Some(String::from("second-state")));
    let issues = octo.issues("second-state", "chat-with-chatgpt");


    match payload {
        EventPayload::IssueCommentEvent(e) => {
            if e.comment.user.r#type != "Bot" {
                if let Some(b) = e.comment.body {
                    if let Some(r) =
                        chat_completion("chatmichael", &format!("issue#{}", e.issue.number), &b, &ChatOptions::default()) {
                        if let Err(e) = issues.create_comment(e.issue.number, r.choice).await {
                            write_error_log!(e.to_string());
                        }
                    }
                }
            }
        }

        EventPayload::IssuesEvent(e) => {
            let title = e.issue.title;
            let body = e.issue.body.unwrap_or("".to_string());
            let q = title + "\n" + &body;

            let prompt = "You are a helpful assistant answering questions on GitHub. In your response, you can use simple markdown text to format your answers.\n\n If someone greets you without asking a question, you should simply respond \"Hello, I am your assistant on GitHub, built by the Second State team. I am ready for your question now!\" \n\n".to_owned() + &q + "\n```";
            let co = ChatOptions {
                model: ChatModel::GPT4,
                restart: true,
                restarted_sentence: Some(&prompt)
            };

            if let Some(r) = chat_completion("chatmichael", &format!("issue#{}", e.issue.number), &prompt, &co) {
                if let Err(e) = issues.create_comment(e.issue.number, r.choice).await {
                    write_error_log!(e.to_string());
                }
            }
        }

        _ => (),
    };
}
