# Helius Webhooks Tutorial

This tutorial will show you how easy it is to create your own webhook to tap into Solana events and send them to your applications, either parsed or in raw formats.

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

## Step 3 - Webhook (Node)

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
    
So far so good! The JSON we sent via cURL was successfully printed to the console like we expected. Now let's try it from Helius. On the webhook page, there is a button on the the far right where you see the webhook you created earlier. You can see it circled in yellow below. Click that link and approve the confirmation modal that pops up. You should see the following output in the console:

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
You can now start requesting data from Helius for your use cases.

## Step 4 - Webhook (Rust)

This is similar to Step 3, meaning all we're doing is creating a single POST endpoint to receive the Helius data. Let's start by taking a look at the dependencies in /rust-app/Cargo.toml.

    warp = "0.3.5"  
    serde = { version = "1.0.183", features = ["derive"] }  
    serde_json = "1.0.104"  
    tokio = { version = "1.30.0", features = ["rt-multi-thread", "macros"] }  
    bytes = "1.4.0"

The "warp" dependency is the web server to serve our webhook. Since warp handles requests asynchronously, tokio is needed to create an asynchronous runtime for warp to operate in. Lastly, serde is used to parse the request body into a JSON struct value.

In the /rust-app directory run:

    cargo build

This will download and build all the dependencies needed for this project.

Open the main.rs file in /rust-app/src:

    use serde_json::Value;  
    use warp::Filter;  
    use warp::http::StatusCode;  
      
    #[tokio::main]  
    async fn main() {  
        //Create an HTTP POST endpoint "webhook" that takes the request body as bytes  
        //The bytes are then transformed into serde_json Value and printed to the console.  
        //If the JSON is valid, then a 200 code is returned   
        let webhook = warp::post()  
            .and(warp::path!("webhook"))  
            .and(warp::body::bytes())  
            .map(|bytes: bytes::Bytes| {  
                let v: Value = serde_json::from_slice(bytes.iter().as_slice()).expect("Error deserializing from slice");  
                println!("{}", v.to_string());  
                StatusCode::OK  
            });  
      
        //Create a simple healthcheck endpoint  
        let health_route = warp::path!("health")  
            .map(|| StatusCode::OK);  
      
        //Create the routes to pass to the server  
        let routes = health_route.or(webhook)  
            .with(warp::cors().allow_any_origin());  
      
        println!("Webhook Started!");  
      
        //Start the server on port 3000  
        warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;  
    }

When this program is run, it creates a web application server running on port 3000, with two endpoints "/webhook" and "/health". To run the program type:

    cargo run

The console output will read:

        Finished dev [unoptimized + debuginfo] target(s) in 0.03s
         Running `target/debug/rust-app`
    Webhook Started!

We can test that our server is running correctly using the same cURL command as in step 3.

    curl --header "Content-Type: application/json" --request POST --data '{"key":"value"}' http://localhost:3000/webhook

In the console you'll see the following output:

    {"key":"value"}

Now let's try the Helius test button like we did in Step 3. The test output will be displayed in the console, in my case it looked like:

    [
       {
          "accountData":[
             {
                "account":"CKs1E69a2e9TmH4mKKLrXFF8kD3ZnwKjoEuXa6sz9WqX",
                "nativeBalanceChange":-72938049280,
                "tokenBalanceChanges":[
                   
                ]
             },
             {
                "account":"NTYeYJ1wr4bpM5xo6zx5En44SvJFAd35zTxxNoERYqd",
                "nativeBalanceChange":0,
                "tokenBalanceChanges":[
                   
                ]
             },
             {
                "account":"AAaTGaA3uVqikfVEwoSG7EwkCb4bBDsMEyueiVUS5CaU",
                "nativeBalanceChange":0,
                "tokenBalanceChanges":[
                   
                ]
             },
             {
                "account":"autMW8SgBkVYeBgqYiTuJZnkvDZMVU2MHJh9Jh7CSQ2",
                "nativeBalanceChange":0,
                "tokenBalanceChanges":[
                   
                ]
             },
             {
                "account":"D8TxfGwdu9MiNMoJmUoC9wQfNfNT7Lnm6DzifQHRTy6B",
                "nativeBalanceChange":0,
                "tokenBalanceChanges":[
                   
                ]
             },
             {
                "account":"5DxD5ViWjvRZEkxQEaJHZw2sBsso6xoXx3wGFNKgXUzE",
                "nativeBalanceChange":71860273440,
                "tokenBalanceChanges":[
                   
                ]
             },
             {
                "account":"25DTUAd1roBFoUQaxJQByL6Qy2cKQCBp4bK9sgfy9UiM",
                "nativeBalanceChange":-2039280,
                "tokenBalanceChanges":[
                   {
                      "mint":"FdsNQE5EeCe57tbEYCRV1JwW5dzNCof7MUTaGWhmzYqu",
                      "rawTokenAmount":{
                         "decimals":0,
                         "tokenAmount":"-1"
                      },
                      "tokenAccount":"25DTUAd1roBFoUQaxJQByL6Qy2cKQCBp4bK9sgfy9UiM",
                      "userAccount":"1BWutmTvYPwDtmw9abTkS4Ssr8no61spGAvW1X6NDix"
                   }
                ]
             },
             {
                "account":"DTYuh7gAGGZg2okM7hdFfU1yMY9LUemCiPyD5Z5GCs6Z",
                "nativeBalanceChange":2039280,
                "tokenBalanceChanges":[
                   {
                      "mint":"FdsNQE5EeCe57tbEYCRV1JwW5dzNCof7MUTaGWhmzYqu",
                      "rawTokenAmount":{
                         "decimals":0,
                         "tokenAmount":"1"
                      },
                      "tokenAccount":"DTYuh7gAGGZg2okM7hdFfU1yMY9LUemCiPyD5Z5GCs6Z",
                      "userAccount":"CKs1E69a2e9TmH4mKKLrXFF8kD3ZnwKjoEuXa6sz9WqX"
                   }
                ]
             },
             {
                "account":"rFqFJ9g7TGBD8Ed7TPDnvGKZ5pWLPDyxLcvcH2eRCtt",
                "nativeBalanceChange":1080000000,
                "tokenBalanceChanges":[
                   
                ]
             },
             {
                "account":"CgXS5xC3qAGSg9txD9bS7BUgugZwshivGXpCJcGmdwrd",
                "nativeBalanceChange":-2234160,
                "tokenBalanceChanges":[
                   
                ]
             },
             {
                "account":"M2mx93ekt1fmXSVkTrUL9xVFHkmME8HTUi5Cyc5aF7K",
                "nativeBalanceChange":0,
                "tokenBalanceChanges":[
                   
                ]
             },
             {
                "account":"E8cU1WiRWjanGxmn96ewBgk9vPTcL6AEZ1t6F6fkgUWe",
                "nativeBalanceChange":0,
                "tokenBalanceChanges":[
                   
                ]
             },
             {
                "account":"11111111111111111111111111111111",
                "nativeBalanceChange":0,
                "tokenBalanceChanges":[
                   
                ]
             },
             {
                "account":"FdsNQE5EeCe57tbEYCRV1JwW5dzNCof7MUTaGWhmzYqu",
                "nativeBalanceChange":0,
                "tokenBalanceChanges":[
                   
                ]
             },
             {
                "account":"AYZsWahcrSnkwqbA1ji7wEzgAnGjLNJhVUMDPfACECZf",
                "nativeBalanceChange":0,
                "tokenBalanceChanges":[
                   
                ]
             },
             {
                "account":"TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
                "nativeBalanceChange":0,
                "tokenBalanceChanges":[
                   
                ]
             },
             {
                "account":"SysvarRent111111111111111111111111111111111",
                "nativeBalanceChange":0,
                "tokenBalanceChanges":[
                   
                ]
             },
             {
                "account":"ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
                "nativeBalanceChange":0,
                "tokenBalanceChanges":[
                   
                ]
             },
             {
                "account":"1BWutmTvYPwDtmw9abTkS4Ssr8no61spGAvW1X6NDix",
                "nativeBalanceChange":0,
                "tokenBalanceChanges":[
                   
                ]
             }
          ],
          "description":"5DxD5ViWjvRZEkxQEaJHZw2sBsso6xoXx3wGFNKgXUzE sold Fox #7637 to CKs1E69a2e9TmH4mKKLrXFF8kD3ZnwKjoEuXa6sz9WqX for 72 SOL on MAGIC_EDEN.",
          "events":{
             "nft":{
                "amount":72000000000,
                "buyer":"CKs1E69a2e9TmH4mKKLrXFF8kD3ZnwKjoEuXa6sz9WqX",
                "description":"5DxD5ViWjvRZEkxQEaJHZw2sBsso6xoXx3wGFNKgXUzE sold Fox #7637 to CKs1E69a2e9TmH4mKKLrXFF8kD3ZnwKjoEuXa6sz9WqX for 72 SOL on MAGIC_EDEN.",
                "fee":10000,
                "feePayer":"CKs1E69a2e9TmH4mKKLrXFF8kD3ZnwKjoEuXa6sz9WqX",
                "nfts":[
                   {
                      "mint":"FdsNQE5EeCe57tbEYCRV1JwW5dzNCof7MUTaGWhmzYqu",
                      "tokenStandard":"NonFungible"
                   }
                ],
                "saleType":"INSTANT_SALE",
                "seller":"5DxD5ViWjvRZEkxQEaJHZw2sBsso6xoXx3wGFNKgXUzE",
                "signature":"5nNtjezQMYBHvgSQmoRmJPiXGsPAWmJPoGSa64xanqrauogiVzFyGQhKeFataHGXq51jR2hjbzNTkPUpP787HAmL",
                "slot":171942732,
                "source":"MAGIC_EDEN",
                "staker":"",
                "timestamp":1673445241,
                "type":"NFT_SALE"
             }
          },
          "fee":10000,
          "feePayer":"CKs1E69a2e9TmH4mKKLrXFF8kD3ZnwKjoEuXa6sz9WqX",
          "nativeTransfers":[
             {
                "amount":72936000000,
                "fromUserAccount":"CKs1E69a2e9TmH4mKKLrXFF8kD3ZnwKjoEuXa6sz9WqX",
                "toUserAccount":"AAaTGaA3uVqikfVEwoSG7EwkCb4bBDsMEyueiVUS5CaU"
             },
             {
                "amount":2011440,
                "fromUserAccount":"CKs1E69a2e9TmH4mKKLrXFF8kD3ZnwKjoEuXa6sz9WqX",
                "toUserAccount":"D8TxfGwdu9MiNMoJmUoC9wQfNfNT7Lnm6DzifQHRTy6B"
             },
             {
                "amount":71856000000,
                "fromUserAccount":"AAaTGaA3uVqikfVEwoSG7EwkCb4bBDsMEyueiVUS5CaU",
                "toUserAccount":"5DxD5ViWjvRZEkxQEaJHZw2sBsso6xoXx3wGFNKgXUzE"
             },
             {
                "amount":1080000000,
                "fromUserAccount":"AAaTGaA3uVqikfVEwoSG7EwkCb4bBDsMEyueiVUS5CaU",
                "toUserAccount":"rFqFJ9g7TGBD8Ed7TPDnvGKZ5pWLPDyxLcvcH2eRCtt"
             },
             {
                "amount":2039280,
                "fromUserAccount":"CKs1E69a2e9TmH4mKKLrXFF8kD3ZnwKjoEuXa6sz9WqX",
                "toUserAccount":"DTYuh7gAGGZg2okM7hdFfU1yMY9LUemCiPyD5Z5GCs6Z"
             }
          ],
          "signature":"5nNtjezQMYBHvgSQmoRmJPiXGsPAWmJPoGSa64xanqrauogiVzFyGQhKeFataHGXq51jR2hjbzNTkPUpP787HAmL",
          "slot":171942732,
          "source":"MAGIC_EDEN",
          "timestamp":1673445241,
          "tokenTransfers":[
             {
                "fromTokenAccount":"25DTUAd1roBFoUQaxJQByL6Qy2cKQCBp4bK9sgfy9UiM",
                "fromUserAccount":"1BWutmTvYPwDtmw9abTkS4Ssr8no61spGAvW1X6NDix",
                "mint":"FdsNQE5EeCe57tbEYCRV1JwW5dzNCof7MUTaGWhmzYqu",
                "toTokenAccount":"DTYuh7gAGGZg2okM7hdFfU1yMY9LUemCiPyD5Z5GCs6Z",
                "toUserAccount":"CKs1E69a2e9TmH4mKKLrXFF8kD3ZnwKjoEuXa6sz9WqX",
                "tokenAmount":1,
                "tokenStandard":"NonFungible"
             }
          ],
          "type":"NFT_SALE"
       }
    ]
Now that you have the data, you can extract values, for example:

    println!("{}", v[0]["type"]);  
    println!("{}", v[0]["accountData"][0]);
    
or you can create a struct and use serde to transform the JSON payload into a struct for you to use in your application.

