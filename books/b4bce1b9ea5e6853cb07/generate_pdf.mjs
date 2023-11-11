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

let contents = `---
document_title: "Rustで数値計算"
pdf_options:
  format: a4
  margin: 30mm 20mm
  printBackground: true
  headerTemplate: |-
    <style>
      section {
        margin: 0 auto;
        font-family: system-ui;
        font-size: 11px;
      }
    </style>
    <section>
      <span class="title"></span>
    </section>
  footerTemplate: |-
    <section>
      <div>
        Page <span class="pageNumber"></span>
        of <span class="totalPages"></span>
      </div>
    </section>
---

<img src="./books/b4bce1b9ea5e6853cb07/cover.jpg" width="100%" />

<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.css" integrity="sha384-n8MVd4RsNIU0tAv4ct0nTaAbDJwPJzDEaqSD1odI+WdtXRGWt2kTvGFasHpSy3SV" crossorigin="anonymous">
<script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.js" integrity="sha384-XjKyOOlGwcjNTAIQHIpgOno0Hl1YQqzUOEleOLALmuqehneUG+vnGctmUb0ZY0l8" crossorigin="anonymous"></script>
<script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/contrib/auto-render.min.js" integrity="sha384-+VBxd3r6XgURycqtZ117nYw44OOcIax56Z4dCRWbxyPt0Koah1uHoK0o4+/RRE05" crossorigin="anonymous"></script>
<script>
    document.addEventListener("DOMContentLoaded", function() {
        renderMathInElement(document.body, {
          // customised options
          // • auto-render specific keys, e.g.:
          delimiters: [
              {left: '$$', right: '$$', display: true},
              {left: '$', right: '$', display: false},
          ],
          // • rendering keys, e.g.:
          throwOnError : false
        });
    });
</script>

<link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/css/bootstrap.min.css" rel="stylesheet" integrity="sha384-9ndCyUaIbzAi2FUVXJi0CjmCapSmO7SnpJef0486qhLnuZ2cdeRhO02iuK6FUUVM" crossorigin="anonymous">
<script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/js/bootstrap.bundle.min.js" integrity="sha384-geWF76RCwLtnZ8qwWowPQNguL3RmwHVBC9FhGdlKrxdiJJigb/j/68SIy3Te4Bkz" crossorigin="anonymous"></script>
`;

[
  'about_pdf',
  'intro',
  'language',
  'thread',
  'simd',
  'num_traits',
  'development',
  'versioning',
  'document',
  'criterion',
  'flamegraph',
  'nom',
  'data_format',
  'library',
  'ndarray_linalg',
  'eom',
  'fftw',
  'rand',
  'sfmt',
  'rayon',
  'plotlib',
  'rug',
  'mkl',
  'interop',
  'link',
  'cc',
  'pyo3',
].forEach(page => {
  const raw = fs.readFileSync(path.join(__dirname, `${page}.md`), { encoding: 'utf8' });
  const [title, body] = trimHeaders(raw);
  contents += asTitle(title);
  contents += body;
  contents += page_break;
});

contents = contents.replaceAll(":::message alert", "<div class=\"alert alert-danger\" role=\"alert\">");
contents = contents.replaceAll(":::message", "<div class=\"alert alert-info\" role=\"alert\">");
contents = contents.replaceAll(":::", "</div>");

(async () => {
  const pdf = await mdToPdf({ content: contents }).catch(console.error);
  if (pdf) {
    fs.writeFileSync('book.pdf', pdf.content);
  }
})();