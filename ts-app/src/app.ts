import express, {Request, Response} from 'express';
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