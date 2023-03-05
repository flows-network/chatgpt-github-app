# chatgpt-github-app


Deploy this function on flows.network and you will get a ChatGPT GitHub App.

See a live demo [here](https://github.com/second-state/chat-with-chatgpt/)

Powered by `gpt-3.5-turbo`, Rust and [WasmEdge](https://github.com/WasmEdge/WasmEdge)

## How to deploy ChatGPT GitHub App on your GitHub repo

We will use flows.network to install ChatGPT GitHub App. It is a serverless platform, so it's easy to deploy your own app in three steps.

### Customize your code

1. For [this repo](https://github.com/flows-network/chatgpt-github-app/).
2. Follow thefollowing instruction to customize the code.

<img width="876" alt="image" src="https://user-images.githubusercontent.com/45785633/222972386-0f66634c-eae6-41df-bc20-635face26822.png">


### deploy the code on flow.network
1. Sign up an account for deploying flows on flows.network. It's free.
2. Click the Get Started button and go to a new page named my flows.
3. Click the Create a Flow button to start deploying the ChatGPT GitHub APP
4. Authciate the flow.network to access chatgpt-github-app repo you just forked and customized. Then click the Next button.
<img width="794" alt="image" src="https://user-images.githubusercontent.com/45785633/222972678-f3df000e-dcbe-4987-bc6c-fc9bbb86f599.png">
5. Name your flow, and click Turn on and Save button to deploy the flow.
<img width="619" alt="image" src="https://user-images.githubusercontent.com/45785633/222972788-68331a34-210d-4184-84b9-aff9bd5d88b2.png">

### Configure the SaaS integrations

After that, go to my flows page again and click the flow we just created.

1. In the Flow details tab, we can see we need to connect the SaaS integrations like the following picture.

<img width="1091" alt="image" src="https://user-images.githubusercontent.com/45785633/222972947-e1db6140-4a14-4b35-b75c-105431435ac0.png">

2. Click the Connect button to authenciate your OpenAI account and you will be redirected to a new page. In this page, you should name your OpenAI API key and copy and paste your OpenAI API key. Please noted, the name you input here should be the same as the code, becucause, to protect your API key, the flows.network only allows the name in public.

<img width="758" alt="image" src="https://user-images.githubusercontent.com/45785633/222973214-ecd052dc-72c2-4711-90ec-db1ec9d5f24e.png">
3. Click the Connect button to authenciate your GitHub account and you will be redirected to a new page. Please noted, you must grant flows.network the access to install flows-network-integration on the repo you just customized in the first step.

That's all. As soon as the function's status is ready, the ChatGPT GitHub App is live. Go to chat with ChatGPT.

The flows.network is its early stage and lots of things are missing. We look forward to hearing your feedback.
