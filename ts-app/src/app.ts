import express, { Request, Response } from 'express';
import bodyParser from 'body-parser';

const app = express();
const PORT = 3000;

// Middleware
app.use(bodyParser.json());

// POST endpoint
app.post('/webhook', (req: Request, res: Response) => {

    const requestBody = req.body;
    // Do something with the request body
    console.log('Received POST request with data:', requestBody);

    // console.log(requestBody[0]);
    // console.log(JSON.stringify(requestBody[1].accountData[1]));
    // console.log(JSON.stringify(requestBody[2].accountData[2]));

    // return res.json({ message: 'POST request successful', data: requestBody });
    res.status(200).send('EVENT_RECEIVED');

});

app.listen(PORT, () => {
    console.log(`Server is running on port ${PORT}`);
});