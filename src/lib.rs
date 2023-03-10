use dotenv::dotenv;
use flowsnet_platform_sdk::write_error_log;
use github_flows::{
    get_octo, listen_to_event,
    octocrab::{
        actions, models::events::payload::EventPayload, models::events::payload::IssuesEventAction,
    },
};
use openai_flows::chat_completion;
use std::env;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    dotenv().ok();

    let OWNER: String = match env::var("OWNER") {
        Err(_) => "second-state".to_string(),
        Ok(name) => name,
    };

    let REPO: String = match env::var("REPO") {
        Err(_) => "chat-with-chatgpt".to_string(),
        Ok(name) => name,
    };

    let OPENPI_KEY_NAME: String = match env::var("OPENPI_KEY_NAME") {
        Err(_) => "chatmichael".to_string(),
        Ok(name) => name,
    };

    listen_to_event(&OWNER, &REPO, vec!["issue_comment", "issues"], |payload| {
        handler(&OWNER, &REPO, &OPENPI_KEY_NAME, payload)
    })
    .await;
}

async fn handler(OWNER: &str, REPO: &str, OPENPI_KEY_NAME: &str, payload: EventPayload) {
    let octo = get_octo(Some(String::from(OWNER)));
    let issues = octo.issues(OWNER, REPO);

    match payload {
        EventPayload::IssueCommentEvent(e) => {
            if e.comment.user.r#type != "Bot" {
                if let Some(b) = e.comment.body {
                    if let Some(r) =
                        chat_completion(OPENPI_KEY_NAME, &format!("issue#{}", e.issue.number), &b)
                    {
                        if let Err(e) = issues.create_comment(e.issue.number, r.choice).await {
                            write_error_log!(e.to_string());
                        }
                    }
                }
            }
        }

        EventPayload::IssuesEvent(e) => {
            if e.action == IssuesEventAction::Closed {
                return;
            }

            let title = e.issue.title;
            let body = e.issue.body.unwrap_or("".to_string());
            let q = title + "\n" + &body;
            if let Some(r) =
                chat_completion(OPENPI_KEY_NAME, &format!("issue#{}", e.issue.number), &q)
            {
                if let Err(e) = issues.create_comment(e.issue.number, r.choice).await {
                    write_error_log!(e.to_string());
                }
            }
        }

        _ => (),
    };
}
