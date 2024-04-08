import puppeteer from 'puppeteer';
import type { CookieParam } from 'puppeteer';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = async ({ url, params, cookies }) => {

    const gotoUrl = `${url.origin}/cookbook/${params.id}`;

    const value = cookies.get('token')!;
    const authCookie: CookieParam = {
        name: 'token',
        value,
        url: gotoUrl,
        httpOnly: true,
        sameSite: 'Strict'
    };

    const browser = await puppeteer.launch({
        args: ['--no-sandbox', '--disable-setuid-sandbox'],
    });

    const page = await browser.newPage();
    await page.setCookie(authCookie);
    await page.goto(gotoUrl, { waitUntil: 'networkidle0' });
    const pdf = await page.pdf({ format: 'A4' });

    await browser.close();

    return new Response(pdf);

};
