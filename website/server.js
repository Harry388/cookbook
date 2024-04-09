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
    console.log('HTTPS Server is running on: https://localhost:%s', SSLPORT);
});

// add a route that lives separately from the SvelteKit app
app.get('/healthcheck', (_, res) => {
    res.end('ok');
});

// let SvelteKit handle everything else, including serving prerendered pages and static assets
app.use(handler);
