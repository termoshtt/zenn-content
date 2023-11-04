import * as path from 'path';
import * as fs from 'fs';
import { fileURLToPath } from 'url';
import { mdToPdf } from 'md-to-pdf';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const target = path.join(__dirname, "intro.md");

(async () => {
    const pdf = await mdToPdf({ path: target }).catch(console.error);

    if (pdf) {
        fs.writeFileSync('book.pdf', pdf.content);
    }
})();