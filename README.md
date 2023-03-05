# chatgpt-github-app


Deploy this function on flows.network and you will get a ChatGPT GitHub App.

See a live demo [here](https://github.com/second-state/chat-with-chatgpt/)

Powered by `gpt-3.5-turbo`, Rust and [WasmEdge](https://github.com/WasmEdge/WasmEdge)

## How to deploy ChatGPT GitHub App on your GitHub repo
To install the ChatGPT GitHub App, we will be using [flows.network](https://flows.network/) - a serverless platform that makes deploying your own app quick and easy in just three steps.

### Customize your code

Fork [this repo](https://github.com/flows-network/chatgpt-github-app/) and customize the code as the instructions given below.

<img width="876" alt="image" src="https://user-images.githubusercontent.com/45785633/222972386-0f66634c-eae6-41df-bc20-635face26822.png">


### deploy the code on flow.network
1. Sign up an account for deploying flows on flows.network. It's free.
2. Click the Get Started button and go to a new page named my flows.
3. Click the Create a Flow button to start deploying the ChatGPT GitHub APP
4. Authciate the flow.network to access chatgpt-github-app repo you just forked and customized. Once done, click on the "Next" button.
<img width="794" alt="image" src="https://user-images.githubusercontent.com/45785633/222972678-f3df000e-dcbe-4987-bc6c-fc9bbb86f599.png">
5. Name your flow, and click Turn on and Save button to deploy the flow.
<img width="619" alt="image" src="https://user-images.githubusercontent.com/45785633/222972788-68331a34-210d-4184-84b9-aff9bd5d88b2.png">

### Configure the SaaS integrations

After that, go to my flows page again and click the flow we just created. In the Flow details tab, we can see we need to connect the SaaS integrations like the following picture.

<img width="1091" alt="image" src="https://user-images.githubusercontent.com/45785633/222972947-e1db6140-4a14-4b35-b75c-105431435ac0.png">

1. Click the Connect button to authenciate your OpenAI account. You'll be redirected to a new page where you should name your OpenAI API key and copy-paste the key. Note that the name you enter here should be the same as the code. This is because, to protect your API key, flows.network only allows the name in public.

<img width="758" alt="image" src="https://user-images.githubusercontent.com/45785633/222973214-ecd052dc-72c2-4711-90ec-db1ec9d5f24e.png">
2. Click the Connect button to authenciate your GitHub accountout. You'll be redirected to a new page where you must grant flows.network access to install flows-network-integration on the repo you just customized in the first step.

That's all. As soon as the function's status is ready, the ChatGPT GitHub App will go live. Go ahead and chat with ChatGPT.

Do note that flows.network is still in its early stages, and there may be some features missing. We would love to hear your feedback!
