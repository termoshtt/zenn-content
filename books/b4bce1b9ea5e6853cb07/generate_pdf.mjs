import * as path from 'path';
import * as fs from 'fs';
import { fileURLToPath } from 'url';
import { mdToPdf } from 'md-to-pdf';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const page_break = '\n<div class="page-break"></div>\n'

function asTitle(title) {
    return `\n# ${title}\n`;
}

const about = fs.readFileSync(path.join(__dirname, "about.md"), { encoding: 'utf8' });
const intro = fs.readFileSync(path.join(__dirname, "intro.md"), { encoding: 'utf8' });

function trimHeaders(content) {
    const lines = content.split('\n');
    const title = lines[1].replace(/^title:/, '').trim();
    lines.splice(0, 3);
    return [title, lines.join('\n')];
}

let contents = '';

['about', 'intro'].forEach(page => {
    const raw = fs.readFileSync(path.join(__dirname, `${page}.md`), { encoding: 'utf8' });
    const [title, body] = trimHeaders(raw);
    contents += asTitle(title);
    contents += body;
    contents += page_break;
});

console.log(contents);

(async () => {
    const pdf = await mdToPdf({ content: contents }).catch(console.error);
    if (pdf) {
        fs.writeFileSync('book.pdf', pdf.content);
    }
})();