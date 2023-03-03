use flowsnet_platform_sdk::write_error_log;
use github_flows::{get_octo, listen_to_event, octocrab::models::events::payload::EventPayload};
use openai_flows::chat_completion;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    listen_to_event(
        "DarumaDockerDev",
        "github-func-test",
        vec!["issue_comment"],
        handler,
    )
    .await;
}

async fn handler(payload: EventPayload) {
    let octo = get_octo(Some(String::from("DarumaDockerDev")));
    let issues = octo.issues("DarumaDockerDev", "github-func-test");

    match payload {
        EventPayload::IssueCommentEvent(e) => {
            if e.comment.user.r#type != "Bot" {
                if let Some(b) = e.comment.body {
                    if let Some(r) =
                        chat_completion("DarumaDocker", &format!("issue#{}", e.issue.number), &b)
                    {
                        if let Err(e) = issues.create_comment(e.issue.number, r.choice).await {
                            write_error_log!(e.to_string());
                        }
                    }
                }
            }
        }
        _ => (),
    };
}
