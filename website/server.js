import { handler } from './build/handler.js';
import express from 'express';
import fs from 'fs';
import https from 'https';

const privateKey = fs.readFileSync('/cert/key.pem', 'utf8');
const certificate = fs.readFileSync('/cert/cert.pem', 'utf8');
const credentials = { key: privateKey, cert: certificate };

const app = express();

const httpsServer = https.createServer(credentials, app);

const SSLPORT = 443;

httpsServer.listen(SSLPORT, function () {
    console.log('HTTPS Server is running', SSLPORT);
});

app.get('/healthcheck', (_, res) => {
    res.end('ok');
});

app.use(handler);
