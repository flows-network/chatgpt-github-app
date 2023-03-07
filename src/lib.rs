use flowsnet_platform_sdk::write_error_log;
use github_flows::{
    get_octo, listen_to_event,
    octocrab::{
        actions, models::events::payload::EventPayload, models::events::payload::IssuesEventAction,
    },
};
use openai_flows::chat_completion;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    let owner = "second-state";
    let repo = "chat-with-chatgpt";

    listen_to_event(owner, repo, vec!["issue_comment"], |payload| {
        handler(owner, repo, payload)
    })
    .await;
}

async fn handler(owner: &str, repo: &str, payload: EventPayload) {
    let octo = get_octo(Some(String::from(owner)));
    let issues = octo.issues(owner, repo);

    let openpi_key_name = "chatmichael";

    match payload {
        EventPayload::IssueCommentEvent(e) => {
            if e.comment.user.r#type != "Bot" {
                if let Some(b) = e.comment.body {
                    if let Some(r) =
                        chat_completion(openpi_key_name, &format!("issue#{}", e.issue.number), &b)
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
                chat_completion(openpi_key_name, &format!("issue#{}", e.issue.number), &q)
            {
                if let Err(e) = issues.create_comment(e.issue.number, r.choice).await {
                    write_error_log!(e.to_string());
                }
            }
        }

        _ => (),
    };
}
