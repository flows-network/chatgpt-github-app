# A ChatGPT bot to respond to your GitHub Issues

[Deploy this function on flows.network](#deploy-chatgpt-github-app-on-your-github-repo), and you will get a GitHub bot that uses ChatGPT to respond to every comment in your GitHub issues automatically. It enables developers to use GitHub Issues as the conversational UI for ChatGPT!

See a live demo [here](https://github.com/second-state/chat-with-chatgpt/)

Powered by `gpt-3.5-turbo`, Rust and [WasmEdge](https://github.com/WasmEdge/WasmEdge)

## Prerequisite 

You will need an [OpenAI API key](https://openai.com/blog/openai-api). If you do not already have one, [sign up here](https://platform.openai.com/signup).

## Deploy ChatGPT GitHub App on your GitHub repo

To install the ChatGPT GitHub App, we will use [flows.network](https://flows.network/), a serverless platform that makes deploying your own app quick and easy in just three steps.

### Fork this repo

Fork [this repo](https://github.com/flows-network/chatgpt-github-app/) and go to flows.network to deploy your function. 

### Deploy the code on flow.network

1. Sign up for an account for deploying flows on [flows.network](https://flows.network/). It's free.
2. Click on the "Get Started" button and go to a new page, "My flows".
3. Click on the "Create a Flow" button to start deploying the ChatGPT GitHub APP
4. Authenticate the [flows.network](https://flows.network/) to access the `chatgpt-github-app` repo you just forked. Don't forget to choose "With Environment Variables", which we will configure the required parameters. Once done, click on the "Next" button.
<img width="681" alt="image" src="https://user-images.githubusercontent.com/45785633/224470404-7ce36f25-efd7-475f-a800-ea3ca3b9412e.png">
5. Fill in the required Environment Variables. In this example, we have three variables. One is `owner`. Fill in the GitHub org Name you want to connect here. The second one is `repo`. Fill in the GitHub repo Name under the GitHub org you just entered. The last one is `open_ai_key`. Fill in the name you want to name your OpenAI Key.
<img width="628" alt="image" src="https://user-images.githubusercontent.com/45785633/224470663-787c669d-dcbd-46f5-be01-cdcac1c261f7.png">

6. Name your flow, and click on "Turn on and Save" button to deploy the flow function.
<img width="619" alt="image" src="https://user-images.githubusercontent.com/45785633/222972788-68331a34-210d-4184-84b9-aff9bd5d88b2.png">

### Configure SaaS integrations

After that, the flows.network will redirect you to the flow details page automatically. In the Flow details tab, we can set up SaaS integrations required by the flow.

<img width="1091" alt="image" src="https://user-images.githubusercontent.com/45785633/222972947-e1db6140-4a14-4b35-b75c-105431435ac0.png">

1. Click on the "Connect" button to authenticate your OpenAI account. You'll be redirected to a new page where you could copy and paste your OpenAI API key and then name the key. Note that the name you enter here should be the same as the name in the environment variables.

<img width="758" alt="image" src="https://user-images.githubusercontent.com/45785633/222973214-ecd052dc-72c2-4711-90ec-db1ec9d5f24e.png">

2. Click the "Connect" button to authenticate your GitHub account. You'll be redirected to a new page where you must grant [flows.network](https://flows.network/) permission to install the `flows-network-integration` bot on a repo. This repo is the one you entered into the environment variables above.

That's all. As soon as the flow function's status becomes `ready`, the ChatGPT GitHub App goes live. Go ahead and chat with ChatGPT by creating an issue!

> [flows.network](https://flows.network/) is still in its early stages. We would love to hear your feedback!
