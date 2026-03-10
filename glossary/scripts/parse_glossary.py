#!/usr/bin/env python3
"""
parse_glossary.py – Parse the raw Gilb Planguage Concept Glossary HTML
and emit a single well-formatted Markdown file with a TOC.

Usage:
    python3 scripts/parse_glossary.py [--input PATH] [--output PATH]

Inputs:
    data/glossary_raw.html  (default)
Outputs:
    glossary.md             (default)
"""

import argparse
import os
import re
import unicodedata
import warnings
from dataclasses import dataclass, field
from typing import List, Optional, Tuple

from bs4 import BeautifulSoup, NavigableString, Tag, XMLParsedAsHTMLWarning

warnings.filterwarnings("ignore", category=XMLParsedAsHTMLWarning)

DEFAULT_INPUT = os.path.join(os.path.dirname(__file__), "..", "data", "glossary_raw.html")
DEFAULT_OUTPUT = os.path.join(os.path.dirname(__file__), "..", "glossary.md")

# Section-label prefixes that become bold labels inside an entry body
SECTION_LABELS = re.compile(
    r"^(Notes?|Related Concepts?|Type|Synonyms?|Synonym|Keyed Icon|Drawn Icon|"
    r"Rationale|Example|Examples?|Figure|Historical Note|Scale|Domain|Parameter|"
    r"Abbr\.|Abbreviation)\s*[\[:]",
    re.IGNORECASE,
)


# ---------------------------------------------------------------------------
# Data model
# ---------------------------------------------------------------------------

@dataclass
class Entry:
    term: str
    concept_id: Optional[str]   # e.g. "509" (without asterisk)
    anchor: str                  # e.g. "c509"
    date: str
    slug: str                    # URL-safe slug for TOC link
    body_md: str                 # Rendered markdown body


# ---------------------------------------------------------------------------
# Utility helpers
# ---------------------------------------------------------------------------

def slugify(text: str) -> str:
    """Convert a term name to a lowercase hyphen-separated slug."""
    text = unicodedata.normalize("NFD", text)
    text = text.encode("ascii", "ignore").decode("ascii")
    text = text.lower()
    text = re.sub(r"[^\w\s-]", "", text)
    text = re.sub(r"[\s_]+", "-", text)
    text = text.strip("-")
    return text


def clean_text(text: str) -> str:
    """Normalise whitespace in extracted text."""
    return re.sub(r"\s+", " ", text).strip()


def convert_links(tag: Tag) -> str:
    """
    Convert an HTML fragment to plain markdown text, preserving internal
    cross-reference links as [*NNN](#cNNN) and stripping other markup.
    """
    parts = []
    for node in tag.children:
        if isinstance(node, NavigableString):
            parts.append(str(node))
        elif isinstance(node, Tag):
            if node.name == "a":
                href = node.get("href", "")
                link_text = clean_text(node.get_text())
                if href.startswith("#c"):
                    parts.append(f"[{link_text}]({href})")
                elif href:
                    parts.append(f"[{link_text}]({href})")
                else:
                    parts.append(link_text)
            elif node.name in ("b", "strong"):
                parts.append(f"**{clean_text(node.get_text())}**")
            elif node.name in ("i", "em"):
                parts.append(f"*{clean_text(node.get_text())}*")
            elif node.name in ("span", "font", "small"):
                parts.append(convert_links(node))
            else:
                parts.append(convert_links(node))
    text = re.sub(r"\s+", " ", "".join(parts)).strip()
    # Remove spaces before punctuation that arise from span concatenation
    text = re.sub(r"\s+([.,;:!?)\]])", r"\1", text)
    # Remove spaces after opening brackets/parens that shouldn't be there
    text = re.sub(r"([\[(])\s+", r"\1", text)
    return text


def render_list(ul: Tag) -> str:
    """Convert a <ul> element to markdown bullet lines."""
    lines = []
    for li in ul.find_all("li", recursive=False):
        text = convert_links(li).strip()
        if text:
            lines.append(f"- {text}")
    return "\n".join(lines)


def tag_text(tag: Tag) -> str:
    return convert_links(tag)


def _find_label_end(text: str, start: int) -> int:
    """
    Return the index just after the first ':' that appears outside any
    bracket/paren groups, starting at `start`.  Returns -1 if not found.

    This correctly skips over markdown link syntax like [text](#url).
    """
    depth_sq = 0
    depth_paren = 0
    i = start
    while i < len(text):
        c = text[i]
        if c == "[":
            depth_sq += 1
        elif c == "]":
            depth_sq = max(depth_sq - 1, 0)
        elif c == "(":
            depth_paren += 1
        elif c == ")":
            depth_paren = max(depth_paren - 1, 0)
        elif c == ":" and depth_sq == 0 and depth_paren == 0:
            return i + 1          # position just after the colon
        i += 1
    return -1


def maybe_bold_label(text: str) -> str:
    """
    If a paragraph starts with a recognised section label (e.g. 'Notes:',
    'Type [Foo *123]:'), make the label portion bold.
    """
    m = SECTION_LABELS.match(text)
    if not m:
        return text

    # Scan from the start of the match for a colon outside any brackets
    colon_pos = _find_label_end(text, m.start())

    if colon_pos == -1:
        # No colon outside brackets – bold the entire text
        return f"**{text}**"

    label = text[:colon_pos].strip()
    rest = text[colon_pos:].strip()
    if not rest:
        return f"**{label}**"
    # Don't insert a space when rest is just trailing punctuation
    if rest[0] in ".,;":
        return f"**{label}**{rest}"
    return f"**{label}** {rest}"


# ---------------------------------------------------------------------------
# Entry header parsing
# ---------------------------------------------------------------------------

_CONCEPT_IN_TEXT = re.compile(r"[*•]\s*(\d+)")
_DATE_PATTERN = re.compile(
    r"\b("
    r"(?:January|February|March|April|May|June|July|August|September|October|November|December)"
    r"\.?\s+\d{1,2}(?:,\s*\d{4})?|\d{1,2}\s+"
    r"(?:January|February|March|April|May|June|July|August|September|October|November|December)"
    r"(?:\s+\d{4})?|\d{4}"
    r")",
    re.IGNORECASE,
)


def parse_header(h1: Tag) -> Tuple[str, Optional[str], str, str]:
    """
    Return (term_name, concept_id, anchor, date) for a glossary entry h1.

    Two layouts exist in the source:
      Layout A: <h1>TERM <a name="cNNN"></a> <font>Concept *NNN <span class="cdate">DATE</span></font></h1>
      Layout B: <h1>TERM *NNN DATE</h1>  (plain text only)
    """
    a_tag = h1.find("a", attrs={"name": True})
    font_tag = h1.find("font")
    cdate_tag = h1.find("span", class_="cdate")

    # --- Concept ID ---
    concept_id: Optional[str] = None
    anchor: str = ""

    if a_tag:
        name_val = a_tag.get("name", "")
        if name_val.startswith("c") and name_val[1:].isdigit():
            concept_id = name_val[1:]
            anchor = name_val
    if concept_id is None and font_tag:
        m = _CONCEPT_IN_TEXT.search(font_tag.get_text())
        if m:
            concept_id = m.group(1)
            anchor = f"c{concept_id}"

    # --- Term name ---
    # Strip the font/anchor sub-elements from the h1 clone to get just the term text
    h1_clone = BeautifulSoup(str(h1), "lxml").find("h1")
    for sub in h1_clone.find_all(["font", "a", "span"]):
        sub.decompose()
    raw_term = clean_text(h1_clone.get_text())

    # For Layout B the concept number is still in the raw_term; strip it
    if concept_id is None:
        m = _CONCEPT_IN_TEXT.search(raw_term)
        if m:
            concept_id = m.group(1)
            anchor = f"c{concept_id}"

    # Remove trailing *NNN / •NNN and everything after (date, letters)
    term = re.sub(r"\s*[*•]\s*\d+.*$", "", raw_term).strip()
    # Remove isolated trailing "Concept" word left after stripping
    term = re.sub(r"\s+Concept\s*$", "", term, flags=re.IGNORECASE).strip()
    # Collapse internal whitespace artefacts (spaces before punctuation)
    term = re.sub(r"\s+([.,])", r"\1", term).strip()

    # --- Date ---
    date = ""
    if cdate_tag:
        date = clean_text(cdate_tag.get_text())
    elif font_tag:
        font_text = clean_text(font_tag.get_text())
        # Remove "Concept *NNN" prefix
        font_text = re.sub(r"^Concept\s*\*\s*\d+\s*", "", font_text).strip()
        date = font_text
    else:
        # Layout B: extract date from raw_term after the concept number
        m_date = _DATE_PATTERN.search(raw_term)
        if m_date:
            date = clean_text(raw_term[m_date.start():])

    # Clean up dates: strip leading/trailing punctuation artefacts
    date = re.sub(r"^[\s.)\-]+", "", date).strip()
    date = re.sub(r"[\s.]+$", "", date).strip()
    # Strip author/editor annotation codes (e.g. "tg", "tg2331", " C", " B")
    date = re.sub(r"\s*tg\d*$", "", date, flags=re.IGNORECASE).strip()
    date = re.sub(r"\s+[A-Z]$", "", date).strip()

    # Fallback anchor when no concept number found
    if not anchor:
        anchor = slugify(term)

    return term, concept_id, anchor, date


# ---------------------------------------------------------------------------
# Body rendering
# ---------------------------------------------------------------------------

def render_body(siblings) -> str:
    """
    Convert the sequence of HTML siblings following an entry h1 into markdown.
    Siblings stop at the next box-title div (handled by the caller).
    """
    md_blocks: List[str] = []

    def flush(text: str) -> None:
        text = text.strip()
        if text:
            md_blocks.append(text)

    for sib in siblings:
        if not isinstance(sib, Tag):
            continue
        cls = sib.get("class", [])
        cls_str = " ".join(cls) if isinstance(cls, list) else cls

        # Skip empty / Wayback / header boilerplate
        if sib.name == "div" and "box-title" in cls_str:
            break  # shouldn't reach here but safety stop

        # Unordered list
        if sib.name in ("ul", "ol"):
            items = render_list(sib)
            if items:
                flush(items)
            continue

        # div > ul (e.g. <div align="left"><ul>...)
        if sib.name == "div":
            ul = sib.find("ul")
            if ul:
                items = render_list(ul)
                if items:
                    flush(items)
            else:
                # Generic div – recurse into its text
                text = convert_links(sib).strip()
                if text:
                    flush(text)
            continue

        # Paragraph
        if sib.name == "p":
            text = tag_text(sib).strip()
            if not text:
                continue
            # Apply bold labelling for known section heads
            text = maybe_bold_label(text)

            p_cls = cls_str.strip()
            if p_cls in ("Example", "Annotation-text-_28_illust-examples_29_",
                         "annotation-text", "Illustration"):
                flush(f"> {text}")
            elif p_cls in ("Quotation", "Quotation-2"):
                flush(f"> *{text}*")
            elif p_cls in ("Footnote",):
                flush(f"_{text}_")
            elif p_cls == "Definition-of-concept":
                flush(text)
            else:
                flush(text)
            continue

        # h2 sub-headings within an entry (rare)
        if sib.name == "h2":
            text = clean_text(sib.get_text())
            if text:
                flush(f"**{text}**")

    return "\n\n".join(md_blocks)


# ---------------------------------------------------------------------------
# Main parsing logic
# ---------------------------------------------------------------------------

def extract_entries(soup: BeautifulSoup) -> List[Entry]:
    content = soup.select_one("#tiki-center")
    if not content:
        raise ValueError("Could not find #tiki-center in the HTML")

    box_titles = content.find_all("div", class_="box-title")
    entries: List[Entry] = []
    seen_anchors: dict = {}

    for box in box_titles:
        h1 = box.find("h1")
        if not h1:
            continue

        term, concept_id, anchor, date = parse_header(h1)

        # Skip metadata/nav headings (no concept ID and short/empty term)
        if not term or (not concept_id and len(term) < 3):
            continue
        # Skip the page title itself
        if term.lower() in ("planguage concept glossary", "services offered by tom"):
            continue
        # Skip spurious box-titles that are actually body text (no concept ID,
        # term reads like a full sentence or a stray phrase)
        if not concept_id:
            first_word = term.split()[0] if term.split() else ""
            if (first_word.lower() in ("an", "a", "the", "see")
                    or len(term) > 80):
                continue

        # Collect siblings between this box-title and the next
        siblings = []
        node = box.find_next_sibling()
        while node:
            if isinstance(node, Tag):
                if "box-title" in node.get("class", []):
                    break
            siblings.append(node)
            node = node.find_next_sibling()

        body_md = render_body(siblings)

        # Unique-ify anchors (duplicate concept IDs in source)
        if anchor in seen_anchors:
            seen_anchors[anchor] += 1
            anchor = f"{anchor}-{seen_anchors[anchor]}"
        else:
            seen_anchors[anchor] = 1

        slug = f"{slugify(term)}-{concept_id}" if concept_id else slugify(term)

        entries.append(Entry(
            term=term,
            concept_id=concept_id,
            anchor=anchor,
            date=date,
            slug=slug,
            body_md=body_md,
        ))

    return entries


# ---------------------------------------------------------------------------
# Markdown generation
# ---------------------------------------------------------------------------

def build_toc(entries: List[Entry]) -> str:
    """Build a grouped A–Z table of contents (deduplicated by term + concept_id)."""
    seen: set = set()
    groups: dict = {}
    for e in entries:
        key = (e.term.lower(), e.concept_id)
        if key in seen:
            continue          # keep only the first occurrence in the TOC
        seen.add(key)

        first = e.term[0].upper() if e.term else "#"
        if not first.isalpha():
            first = "#"
        groups.setdefault(first, []).append(e)

    lines = ["## Table of Contents\n"]
    for letter in sorted(groups.keys()):
        lines.append(f"### {letter}\n")
        for e in groups[letter]:
            label = f"{e.term} \\*{e.concept_id}" if e.concept_id else e.term
            lines.append(f"- [{label}](#{e.anchor})")
        lines.append("")

    return "\n".join(lines)


def build_entry_md(e: Entry) -> str:
    lines = []
    # HTML anchor for cross-reference links (e.g. #c509)
    lines.append(f'<a id="{e.anchor}"></a>')
    lines.append("")
    # Entry heading
    heading_parts = [f"## {e.term}"]
    if e.concept_id:
        heading_parts.append(f" · Concept \\*{e.concept_id}")
    lines.append("".join(heading_parts))
    # Date sub-line
    if e.date:
        lines.append("")
        lines.append(f"*{e.date}*")
    # Body
    if e.body_md.strip():
        lines.append("")
        lines.append(e.body_md)
    lines.append("")
    lines.append("---")
    lines.append("")
    return "\n".join(lines)


def build_markdown(entries: List[Entry]) -> str:
    sections = []

    # Document header
    sections.append(
        "# Planguage Concept Glossary\n\n"
        "**Owner:** Tom Gilb  \n"
        "**Copyright:** © 1996–2006 Tom Gilb. All rights reserved.  \n"
        "**Contact:** Tom@Gilb.com  \n"
        "**Source:** [Wayback Machine capture, 2 July 2007]"
        "(https://web.archive.org/web/20070702020021/"
        "http://www.gilb.com/community/tiki-page.php"
        "?pageName=Competitive%20Engineering%20Glossary)\n"
    )

    # TOC
    sections.append(build_toc(entries))
    sections.append("---\n")

    # Entries
    for e in entries:
        sections.append(build_entry_md(e))

    return "\n".join(sections)


# ---------------------------------------------------------------------------
# CLI
# ---------------------------------------------------------------------------

def main() -> None:
    parser = argparse.ArgumentParser(description="Parse Gilb glossary HTML → Markdown")
    parser.add_argument("--input", default=DEFAULT_INPUT, help="Raw HTML input path")
    parser.add_argument("--output", default=DEFAULT_OUTPUT, help="Markdown output path")
    args = parser.parse_args()

    input_path = os.path.abspath(args.input)
    output_path = os.path.abspath(args.output)

    print(f"Reading: {input_path}")
    with open(input_path, encoding="utf-8") as fh:
        html = fh.read()

    soup = BeautifulSoup(html, "lxml")
    print("Extracting entries …")
    entries = extract_entries(soup)
    print(f"  {len(entries)} entries found")

    print("Building markdown …")
    md = build_markdown(entries)

    os.makedirs(os.path.dirname(output_path), exist_ok=True)
    with open(output_path, "w", encoding="utf-8") as fh:
        fh.write(md)
    print(f"Written: {output_path}  ({len(md):,} chars, {md.count(chr(10))+1} lines)")


if __name__ == "__main__":
    main()
