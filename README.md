Helius Webhooks Tutorial

Overview of what you'll do in this tutorial

 - Create an ngrok account (https://ngrok.com/)
 - Create an account on Helius (https://www.helius.dev)  in order to push data to the webhook you'll create
 - Create a simple application using TypeScript with an HTTP endpoint as the webhook.
 - (Optional) Create a simple application using Rust with an HTTP endpoint as the webhook.

Step 1 - Setup an ngrok account

Because we'll be running our webhook locally (localhost:3000/webhook), we need a way to tell Helius how to send a request to our localhost. There are several ways to do this, but the easiest is to use ngrok (https://ngrok.com/). Sign up is free and straightforward. Once you do that, you can download ngrok to your machine (Windows, MacOS, Linux). Once that is installed you can check to see if it's working:

     ngrok -v

If that successfully prints the version you installed then you can run:

    ngrok http 3000

After executing that command you should see something like this:

    Forwarding      https://3214-199-223-251-92.ngrok-free.app -> http://localhost:3000

This means, you can now use "https://3214-199-223-251-92.ngrok-free.app" and a public facing URL that will forward to your localhost. This URL is what we'll use to configure Helius.

Step 2 - Setup

If you don't already have a Helius (https://www.helius.dev) account, you can create one for free. It will generate a project name for you and ask you to generate a new API key. Click the button and it will generate a new key and forward you to your new dashboard. You'll see two a devnet and mainnet URL that you can use for Solana RPC endpoints, but what we are interested in is the webhook functionality. Let's click on the webhook link in the navigation menu.

Click the "New Webhook" button. Now we'll configure the webhook using the URL that ngrok provided, but with the addition of "/webhook": [https://3214-199-223-251-92.ngrok-free.app/webhook](https://3214-199-223-251-92.ngrok-free.app/webhook)

The applications we'll make in next section will use "webhook" as the path for the HTTP endpoint.



curl --header "Content-Type: application/json" \
--request POST \
--data '[{"blockTime":1673445241,"indexWithinBlock":2557,"meta":{"err":null,"fee":10000,"innerInstructions":[null],"loadedAddresses":[null],"logMessages":[null],"postBalances":[null],"postTokenBalances":[null],"preBalances":[null],"preTokenBalances":[null],"rewards":[]},"slot":171942732,"transaction":{"message":[null],"signatures":[null]}}]' \
http://localhost:3000/webhook

