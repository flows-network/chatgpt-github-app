# <p align="center">A ChatGPT bot to respond to your GitHub Issues</p>

<p align="center">
  <a href="https://discord.gg/ccZn9ZMfFf">
    <img src="https://img.shields.io/badge/chat-Discord-7289DA?logo=discord" alt="flows.network Discord">
  </a>
  <a href="https://twitter.com/flows_network">
    <img src="https://img.shields.io/badge/Twitter-1DA1F2?logo=twitter&amp;logoColor=white" alt="flows.network Twitter">
  </a>
   <a href="https://flows.network/flow/new">
    <img src="https://img.shields.io/website?up_message=deploy&url=https%3A%2F%2Fflows.network%2Fflow%2Fnew" alt="Create a flow">
  </a>
</p>

[Deploy this function on flows.network](#deploy-chatgpt-github-app-on-your-github-repo), and you will get a GitHub bot that uses ChatGPT to respond to every comment in your GitHub issues automatically. It enables developers to use GitHub Issues as the conversational UI for ChatGPT!

See a live demo [here](https://github.com/second-state/chat-with-chatgpt/) | Powered by `gpt4`, Rust and [WasmEdge](https://github.com/WasmEdge/WasmEdge).

IMPORTANT NOTICE: If you have deployed this function on flows.network before 7 AM 24/03/2023(UTC), please refer to [this issue](https://github.com/flows-network/chatgpt-github-app/issues/4) to fix your GitHub App.

## Prerequisites 

You will need to bring your own [OpenAI API key](https://openai.com/blog/openai-api). If you do not already have one, [sign up here](https://platform.openai.com/signup).

## Deploy ChatGPT GitHub App on your GitHub repo

To install the ChatGPT GitHub App, we will use [flows.network](https://flows.network/), a serverless platform that makes deploying your own app quick and easy in just three steps.

### Fork this repo

Fork [this repo](https://github.com/flows-network/chatgpt-github-app/) and go to flows.network to deploy your function. 

### Deploy the code on flow.network

1. Log into [flows.network](https://flows.network/) from your GitHub account. It's free.
2. Click on the "Create a Flow" button to start deploying the ChatGPT bot
3. Authenticate [flows.network](https://flows.network/) to access the `chatgpt-github-app` repo you just forked. 
![image](https://user-images.githubusercontent.com/45785633/226546523-93071359-b957-4653-a429-ab983ee9a078.png)

4. Click on the Advanced text and you will see more settings. we can fill in the required Environment Variables here. In this example, we have four variables. 
* `login`: Fill in your personal GitHub id here. The GitHub app will act as you when responding to questions. 
* `owner`: Fill in the GitHub org you want to deploy the bot on. 
* `repo` : Fill in the GitHub repo you want to deploy the bot on. 
* `openai_key_name` : Fill in the name you want for your OpenAI key. You can put any name here, and we will connect it to the actual key later.

![image](https://user-images.githubusercontent.com/45785633/227463828-9ea913a5-f0a0-46bd-8d4c-da439ee72a94.png)

5. Click on the Deploy button to deploy your function.

### Configure SaaS integrations

After that, [flows.network](https://flows.network/) will direct you to configure the SaaS integration required by your flow.

![image](https://user-images.githubusercontent.com/45785633/226547995-54927771-7782-484a-8c9c-908e91f99444.png)

Here we can see, we need to configure two SaaS integrations.

1. Click on the "Connect" or "+ Add new authentication" button to authenticate your OpenAI account. On the next page, copy and paste your OpenAI API key and then name the key. **Note** the name you enter here should be the same as the name in the environment variables.

<img width="758" alt="image" src="https://user-images.githubusercontent.com/45785633/222973214-ecd052dc-72c2-4711-90ec-db1ec9d5f24e.png">

2. Click on the "Connect" or "+ Add new authentication" button to authenticate your GitHub account to deploy the bot. You'll be redirected to a GitHub page where you must grant [flows.network](https://flows.network/) permission to install the `flows-network-integration` bot on an `owner/repo`. This repo is the one you entered into the environment variables above.

Click on the Check button to see your flow details. As soon as the flow function's status becomes `ready` and the flow's status became `running`, the ChatGPT GitHub App goes live. Go ahead and chat with ChatGPT by creating an issue!

![image](https://user-images.githubusercontent.com/45785633/226550405-67d0741c-6c78-42ef-87d1-b30bbd45a5a9.png)

