# A ChatGPT bot to respond to your GitHub Issues

[Deploy this function on flows.network](#deploy-chatgpt-github-app-on-your-github-repo), and you will get a GitHub bot that uses ChatGPT to respond to every comment in your GitHub issues automatically. It enables developers to use GitHub Issues as the conversational UI for ChatGPT!

See a live demo [here](https://github.com/second-state/chat-with-chatgpt/) | Powered by `gpt4`, Rust and [WasmEdge](https://github.com/WasmEdge/WasmEdge).

IMPORTANT NOTICE: If you have deployed this function on flows.network before 7 AM 24/03/2023(UTC), please refer to [this issue](https://github.com/flows-network/chatgpt-github-app/issues/4) to fix your GitHub App.

## Prerequisite 

You will need an [OpenAI API key](https://openai.com/blog/openai-api). If you do not already have one, [sign up here](https://platform.openai.com/signup).

## Deploy ChatGPT GitHub App on your GitHub repo

To install the ChatGPT GitHub App, we will use [flows.network](https://flows.network/), a serverless platform that makes deploying your own app quick and easy in just three steps.

### Fork this repo

Fork [this repo](https://github.com/flows-network/chatgpt-github-app/) and go to flows.network to deploy your function. 

### Deploy the code on flow.network

1. Sign up for an account for deploying flows on [flows.network](https://flows.network/). It's free.
2. Click on the "Create a Flow" button to start deploying the ChatGPT GitHub APP
3. Authenticate the [flows.network](https://flows.network/) to access the `chatgpt-github-app` repo you just forked. 
![image](https://user-images.githubusercontent.com/45785633/226546523-93071359-b957-4653-a429-ab983ee9a078.png)

4. Click on the Advanced text and you will see more settings. we can fill in the required Environment Variables here. In this example, we have four variables. 
* One is `login`: Fill in your personel github id here. The github app will act as you when respond to questions. 
* The second one is `owner`: Fill in the GitHub org Name you want to connect here. 
* The thrid one is `repo` : Fill in the GitHub repo Name under the GitHub org you just entered. 
* The last one is `openai_key_name` : **Fill in the name you want to name your OpenAI Key**.

![image](https://user-images.githubusercontent.com/45785633/227463828-9ea913a5-f0a0-46bd-8d4c-da439ee72a94.png)

5. At last, click the Deploy button to deploy your function.

### Configure SaaS integrations

After that, the flows.network will direct you to configure the SaaS integration required by your flow.

![image](https://user-images.githubusercontent.com/45785633/226547995-54927771-7782-484a-8c9c-908e91f99444.png)

Here we can see, we need to configue two SaaS integrations.

1. Click on the "Connect/+ Add new authentication" button to authenticate your OpenAI account. You'll be redirected to a new page where you could copy and paste your OpenAI API key and then name the key. **Note that the name you enter here should be the same as the name in the environment variables.**

<img width="758" alt="image" src="https://user-images.githubusercontent.com/45785633/222973214-ecd052dc-72c2-4711-90ec-db1ec9d5f24e.png">

2. Click the "Connect/+ Add new authentication" button to authenticate your GitHub account. You'll be redirected to a new page where you must grant [flows.network](https://flows.network/) permission to install the `flows-network-integration` bot on a repo. This repo is the one you entered into the environment variables above.

After that, click the Check button to see your flow details. As soon as the flow function's status becomes `ready` and the flow's status became `running`, the ChatGPT GitHub App goes live. Go ahead and chat with ChatGPT by creating an issue!

![image](https://user-images.githubusercontent.com/45785633/226550405-67d0741c-6c78-42ef-87d1-b30bbd45a5a9.png)

> [flows.network](https://flows.network/) is still in its early stages. We would love to hear your feedback!
