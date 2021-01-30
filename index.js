const puppeteer = require("puppeteer");
const imagesToPdf = require("images-to-pdf");
const rimraf = require('rimraf');
const fs = require("fs");
const SCREENSHOTS_PATH = "./screenshots";


async function createPuppeteerInstance(pageUrl) {
    console.log("Creating new browser instance");
    const browser = await puppeteer.launch();
    const page = await browser.newPage();
    await page.goto(pageUrl);
    console.log("Browser instance created");
    return page;
}

async function extractUrls(pageInstance) {
    console.log(`Extracting urls`);
    const urls = await pageInstance.evaluate(() => {
        return [...document.querySelectorAll(".sitemapPageLink")]
            .map(element => element.attributes)
            .map(attribute => Object.keys(attribute).map(attributeKey => attribute[attributeKey]))
            .flatMap(attribute => attribute)
            .filter(attribute => attribute.localName === "nodeurl")
            .map(attribute => attribute.value);
    });
    console.log(`I've found this urls: ${urls}`);
    return urls;
}

async function openPopUp(pageInstance) {
    await pageInstance.evaluate(() => {
        [...document.querySelectorAll(".annnoteimage")]
            .forEach((element, index) => {
                element.click();
                const elementBounding = element.getBoundingClientRect();
                const currentPopUp = document.querySelector(`[aria-labelledby='ui-dialog-title-${index + 1}']`);
                currentPopUp.style.top = elementBounding.y + "px";
                currentPopUp.style.left = elementBounding.x + "px";
            });
    });
}

async function takeScreenshot(browserPage, pageUrl, index, { baseUrl, shouldOpenPopUp = false }) {
    const pageToGo = `${baseUrl}/${pageUrl}`;
    console.log(`Taking screenshot ${index} for page ${pageToGo}`);
    await browserPage.goto(pageToGo);
    if (shouldOpenPopUp) {
        await openPopUp(browserPage);
    }
    const screenshotName = `${index} - ${pageUrl.replace(".html", "")}.png`;
    await browserPage.screenshot({ path: `${SCREENSHOTS_PATH}/${screenshotName}`, fullPage: true });
}

async function createPdf(pdfName = 'axshare-wireframe-exporter') {
    console.log(`Creating pdf ${pdfName}`);
    const imagesToJoin = fs.readdirSync(SCREENSHOTS_PATH).map(imageName => `${SCREENSHOTS_PATH}/${imageName}`);
    await imagesToPdf(imagesToJoin, `${pdfName}.pdf`);
}

function createScreenshotsDirectory() {
    console.log(`Creating screenshots folder ${SCREENSHOTS_PATH}`);
    rimraf.sync(SCREENSHOTS_PATH);
    fs.mkdirSync(SCREENSHOTS_PATH);
}

function sanitizeBaseUrl(baseUrl) {
    return baseUrl.endsWith("/") ? baseUrl.slice(0, -1) : baseUrl;
}

function extractArgs([baseUrl, pdfName = 'axshare-wireframe-exporter', openPopUp = false]) {
    return { baseUrl: sanitizeBaseUrl(baseUrl), pdfName, openPopUp };
}

async function begin() {
    const params = extractArgs(process.argv.slice(2));
    console.log(`Backup of ${params.baseUrl}`);
    const browserPage = await createPuppeteerInstance(params.baseUrl);
    const urls = await extractUrls(browserPage);
    createScreenshotsDirectory();
    await urls.reduce(async (promise, url, index) => {
        await promise;
        await takeScreenshot(browserPage, url, index, params);
    }, Promise.resolve());
    await createPdf(params.pdfName);
    process.exit(0);
}

begin();