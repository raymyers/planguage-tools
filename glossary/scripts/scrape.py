#!/usr/bin/env python3
"""
scrape.py – Fetch the Competitive Engineering / Planguage Concept Glossary
from the Wayback Machine and save the raw HTML to disk.

Usage:
    python3 scripts/scrape.py [--output PATH]

Outputs:
    data/glossary_raw.html  (default)
"""

import argparse
import os
import sys
import time

import requests

URL = (
    "https://web.archive.org/web/20070702020021/"
    "http://www.gilb.com/community/tiki-page.php"
    "?pageName=Competitive%20Engineering%20Glossary"
)
DEFAULT_OUTPUT = os.path.join(os.path.dirname(__file__), "..", "data", "glossary_raw.html")


def fetch(url: str, retries: int = 3, backoff: float = 5.0) -> str:
    headers = {
        "User-Agent": (
            "Mozilla/5.0 (compatible; glossary-scraper/1.0; "
            "+https://github.com/gilb-glossary)"
        )
    }
    for attempt in range(1, retries + 1):
        try:
            print(f"Fetching (attempt {attempt}/{retries}): {url}")
            resp = requests.get(url, headers=headers, timeout=60)
            resp.raise_for_status()
            print(f"  OK – {len(resp.text):,} chars received")
            return resp.text
        except requests.RequestException as exc:
            print(f"  Error: {exc}", file=sys.stderr)
            if attempt < retries:
                print(f"  Waiting {backoff}s before retry …")
                time.sleep(backoff)
            else:
                raise


def main() -> None:
    parser = argparse.ArgumentParser(description="Fetch Gilb glossary HTML from Wayback Machine")
    parser.add_argument("--output", default=DEFAULT_OUTPUT, help="Output path for raw HTML")
    args = parser.parse_args()

    output_path = os.path.abspath(args.output)
    os.makedirs(os.path.dirname(output_path), exist_ok=True)

    html = fetch(URL)

    with open(output_path, "w", encoding="utf-8") as fh:
        fh.write(html)
    print(f"Saved to: {output_path}")


if __name__ == "__main__":
    main()
