# Helius Webhooks Tutorial

All code for this tutorial can be found here:
https://github.com/wkennedy/helius-webhooks-tutorial

## Overview of what you'll do in this tutorial

 - Create an ngrok account (https://ngrok.com/)
 - Create an account on Helius (https://www.helius.dev)  in order to push data to the webhook you'll create
 - Create a simple application using TypeScript with an HTTP endpoint as the webhook.
 - (Optional) Create a simple application using Rust with an HTTP endpoint as the webhook.

Assumptions

 - You already have npm installed
 - You have cURL installed
 - (optional) You already have Rust installed.

## Step 1 - Setup an ngrok account

Because we'll be running our webhook locally (localhost:3000/webhook), we need a way to tell Helius how to send a request to our localhost. There are several ways to do this, but the easiest is to use ngrok (https://ngrok.com/). Sign up is free and straightforward. Once you do that, you can download ngrok to your machine (Windows, MacOS, Linux). Once that is installed you can check to see if it's working:

     ngrok -v

If that successfully prints the version you installed then you can run:

    ngrok http 3000

After executing that command you should see something like this:

    Forwarding      https://3214-199-223-251-92.ngrok-free.app -> http://localhost:3000

This means, you can now use "https://3214-199-223-251-92.ngrok-free.app" and a public facing URL that will forward to your localhost. This URL is what we'll use to configure Helius.

## Step 2 - Setup

If you don't already have a Helius (https://www.helius.dev) account, you can create one for free. It will generate a project name for you and ask you to generate a new API key. Click the button and it will generate a new key and forward you to your new dashboard. You'll see two a devnet and mainnet URL that you can use for Solana RPC endpoints, but what we are interested in is the webhook functionality. Let's click on the webhook link in the navigation menu.

Click the "New Webhook" button. Now we'll configure the webhook using the URL that ngrok provided, but with the addition of "/webhook": [https://3214-199-223-251-92.ngrok-free.app/webhook](https://3214-199-223-251-92.ngrok-free.app/webhook)

The applications we'll make in next section will use "webhook" as the path for the HTTP endpoint.

## Step 3 - Webhook

For this application, we'll create a simple web application using Express. The webapp will have a single POST endpoint, "/webhook", that will be called by Helius.  Let's start by taking a look at the ts-app/package.json file. The application will need the following dependencies:

    "dependencies": {  
      "@types/express": "^4.17.17",  
      "@types/node": "^20.4.8",  
      "body-parser": "^1.20.2",  
      "express": "^4.18.2",  
      "ts-node": "^10.9.1",  
      "typescript": "^5.1.6"  
    }

These dependencies will provide us with TypeScript support, the ability to deploy a web application using Express, and parse the request body as JSON. 

If you haven't already, go ahead and do an npm install in the ts-app root to grab the node modules for the project.

    cd /helius-webhooks-tutorial/ts-app
    npm install
    
Now let's take a look at the application itself:

    import express, { Request, Response } from 'express';
    import bodyParser from 'body-parser';
    
    const app = express();
    const port = 3000;
    
    //We are expecting the body to contain valid JSON from the Helius request
    app.use(bodyParser.json());
    
    //This is our actual webhook that is served locally at http://localhost:3000/webhook.
    //By using ngrok, the ngrok provided public url will map to our local webhook
    //https://3214-199-223-251-92.ngrok-free.app/webhook -> http://localhost:3000/webhook
    app.post('/webhook', (request: Request, response: Response) => {
    
        const requestBody = request.body;
        //Print the JSON to the console
        console.log('Data received by webook: ', requestBody);
    
        //Send a response that we received and processed the request.
        response.status(200).send('Webhook Request Received!');
    
    });
    
    app.listen(port, () => {
        console.log(`Server is running on port ${port}`);
    });

This is a simple application that listens for requests at http://localhost:3000/webhook and simply prints the JSON from the request body to the console. It's up to you what data you want from Helius and how to process it. For this example we are just going to use the test functionality from Helius.

Let's go ahead and start the web application:

    npm run start-server

You should see the following output:


    > ts-app@1.0.0 start-server
    > npx ts-node src/app.ts
    
    Server is running on port 3000

Before we test with Helius, we can check that everything is running okay locally. Issue the following cURL command:

    curl --header "Content-Type: application/json" --request POST --data '{"key":"value"}' http://localhost:3000/webhook

You should see the following output in the console:

    Data received by webook:  { key: 'value' }

Now let's send a test message through Helius:


You should see the following output in the console:

    Data received by webook:  [
      {
        blockTime: 1673445241,
        indexWithinBlock: 2557,
        meta: {
          err: null,
          fee: 10000,
          innerInstructions: [Array],
          loadedAddresses: [Array],
          logMessages: [Array],
          postBalances: [Array],
          postTokenBalances: [Array],
          preBalances: [Array],
          preTokenBalances: [Array],
          rewards: []
        },
        slot: 171942732,
        transaction: { message: [Array], signatures: [Array] }
      }
    ]
So far so good! The JSON we sent via cURL was successfully printed to the console like we expected. How let's try it from Helius.

