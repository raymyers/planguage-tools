# Glossary Scripts

Two Python scripts that fetch Tom Gilb's *Planguage Concept Glossary* from a
Wayback Machine snapshot and convert it to a single well-formatted Markdown
file.

## Quick start

```bash
# Install dependencies (BeautifulSoup + requests)
pip install requests beautifulsoup4 lxml

# 1. Fetch the archived HTML (writes ../data/glossary_raw.html)
python3 scrape.py

# 2. Parse it into Markdown (writes ../glossary.md)
python3 parse_glossary.py
```

Both scripts accept `--help` for all options.

---

## `scrape.py`

Fetches a single Wayback Machine URL and saves the raw HTML to disk.

**Source URL**

```
https://web.archive.org/web/20070702020021/
  http://www.gilb.com/community/tiki-page.php
  ?pageName=Competitive%20Engineering%20Glossary
```

**Default output:** `../data/glossary_raw.html`

**Options**

| Flag | Default | Description |
|------|---------|-------------|
| `--output PATH` | `../data/glossary_raw.html` | Where to write the raw HTML |

The script retries up to three times with a five-second back-off on network
errors.

---

## `parse_glossary.py`

Reads the cached HTML and emits a single Markdown file.

**Default input:**  `../data/glossary_raw.html`  
**Default output:** `../glossary.md`

**Options**

| Flag | Default | Description |
|------|---------|-------------|
| `--input PATH`  | `../data/glossary_raw.html` | Raw HTML to parse |
| `--output PATH` | `../glossary.md` | Markdown file to write |

### What the parser does

The source page is a TikiWiki document containing 677 `<div class="box-title">`
elements.  Each one is either a glossary entry header or (in four cases)
mis-tagged body text; the parser handles both.

**Entry header layout** — two variants exist in the source:

| Layout | Example |
|--------|---------|
| A (most entries) | `<h1>Term <a name="cNNN"></a> <font>Concept *NNN <span class="cdate">DATE</span></font></h1>` |
| B (56 entries)   | `<h1>Term *NNN DATE</h1>` (plain text only) |

For each entry the parser extracts:

1. **Term name** — text before any `*NNN` marker; trailing `Concept` word,
   internal whitespace-before-punctuation artefacts, and editor stamps
   (`tg`, `tg2331`, etc.) are stripped.
2. **Concept ID** — from the `<a name="cNNN">` anchor, the `<font>` text, or
   a regex over the raw h1 text.  Accepts both `*` and `•` as the ID prefix.
3. **Date** — from `<span class="cdate">` or extracted from the h1 text.
   Leading punctuation artefacts, author initials, and single-letter edit
   codes are stripped.
4. **Body** — all sibling elements until the next entry header:
   - Recognised section labels (`Notes`, `Type`, `Related Concepts`,
     `Synonyms`, `Keyed Icon`, `Drawn Icon`, `Rationale`, `Example`,
     `Figure`, …) are bolded.  A bracket-depth-aware colon finder is used so
     that markdown link syntax inside labels is never broken.
   - `<p class="Example">` and illustration paragraphs become block-quotes.
   - `<p class="Quotation*">` becomes an italic block-quote.
   - `<ul>/<li>` lists become Markdown bullet lists.
   - Internal cross-reference links (`href="#cNNN"`) are kept as
     `[*NNN](#cNNN)` markdown links.
   - Spaces introduced before punctuation by HTML span concatenation are
     normalised away.

**Output structure**

```
# Planguage Concept Glossary          ← document title + metadata
## Table of Contents                  ← A–Z grouped, *NNN labels, ref-links
### A  ### B  …  ### Z  ### #
- [Term \*NNN](#cNNN)                 ← one line per unique term+concept pair
---
<a id="cNNN"></a>                     ← HTML anchor for cross-ref resolution
## Term · Concept \*NNN               ← entry heading
*Date*
Definition paragraph …
**Notes:** …                          ← bold section labels
**Related Concepts [Term [*NNN](#cNNN)]:**
- Other Term [*MMM](#cMMM)
**Type [Term [*NNN](#cNNN)]:** …
---
```

**Known source quirks handled**

| Issue | Handling |
|-------|----------|
| 4 `<div class="box-title">` elements contain full-sentence body text | Filtered: no concept ID + first word in `{a, an, the, see}` or text > 80 chars |
| 3 duplicate `(term, concept_id)` pairs in source | TOC deduplicated (first occurrence only); both entries kept in body |
| `•NNN` bullet char used instead of `*NNN` | `[*•]\s*(\d+)` regex handles both |
| Dates like `. . August 2, 2001` or `) . July 16, 2002` | Leading `. )` stripped |
| Editor stamps: `tg`, `tg2331`, `tg 2315` | Stripped with `\s*tg\d*$` |
| Single-letter edit codes: `March 4, 2003 C` | Stripped with `\s+[A-Z]$` |
