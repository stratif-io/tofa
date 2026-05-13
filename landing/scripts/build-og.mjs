import puppeteer from 'puppeteer';
import { fileURLToPath } from 'node:url';
import path from 'node:path';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const input  = `file://${path.resolve(__dirname, 'og.html')}`;
const output = path.resolve(__dirname, '../public/og.png');

const browser = await puppeteer.launch();
try {
  const page = await browser.newPage();
  await page.setViewport({ width: 1200, height: 630, deviceScaleFactor: 1 });
  await page.goto(input, { waitUntil: 'networkidle0' });
  await page.screenshot({ path: output, type: 'png' });
} finally {
  await browser.close();
}
console.log(`✓ wrote ${output}`);
