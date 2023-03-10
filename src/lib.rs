use dotenv::dotenv;
use flowsnet_platform_sdk::write_error_log;
use github_flows::{
    get_octo, listen_to_event,
    octocrab::{models::events::payload::EventPayload, models::events::payload::IssuesEventAction},
};
use openai_flows::chat_completion;
use std::env;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    dotenv().ok();

    let owner: String = match env::var("owner") {
        Err(_) => "second-state".to_string(),
        Ok(name) => name,
    };

    let repo: String = match env::var("repo") {
        Err(_) => "chat-with-chatgpt".to_string(),
        Ok(name) => name,
    };

    let openai_key_name: String = match env::var("openai_key_name") {
        Err(_) => "chatmichael".to_string(),
        Ok(name) => name,
    };

    listen_to_event(&owner, &repo, vec!["issue_comment", "issues"], |payload| {
        handler(&owner, &repo, &openai_key_name, payload)
    })
    .await;
}

async fn handler(owner: &str, repo: &str, openai_key_name: &str, payload: EventPayload) {
    let octo = get_octo(Some(String::from(owner)));
    let issues = octo.issues(owner, repo);

    match payload {
        EventPayload::IssueCommentEvent(e) => {
            if e.comment.user.r#type != "Bot" {
                if let Some(b) = e.comment.body {
                    if let Some(r) =
                        chat_completion(openai_key_name, &format!("issue#{}", e.issue.number), &b)
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
                chat_completion(openai_key_name, &format!("issue#{}", e.issue.number), &q)
            {
                if let Err(e) = issues.create_comment(e.issue.number, r.choice).await {
                    write_error_log!(e.to_string());
                }
            }
        }

        _ => (),
    };
}
