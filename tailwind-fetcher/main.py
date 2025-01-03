# This script extracts Tailwind sections along with their categories, class names, and properties.
# It is intended to be executed only once to generate a JSON file, which will then be used
# to create the necessary types in Rust. This script is NOT intended to be executed repeatedly
# every time the types in Rust need to be generated.

import os
import requests
import random
import time
import json
from bs4 import BeautifulSoup
from models import Section, Category, Variant

base_url = "https://tailwindcss.com"
output_file = "../types.json"


def fetch_sections(selected_section=None):
    request = requests.get(f"{base_url}/docs/installation")
    soup = BeautifulSoup(request.text, "html.parser")

    skip_sections = [
        "Getting Started",
        "Core Concepts",
        "Customization",
        "Base Styles",
        "Official Plugins",
    ]

    sections = []
    for node in soup.select("li.mt-12.lg\\:mt-8"):
        section_title = node.select_one("h5").get_text()

        if not selected_section and section_title in skip_sections:
            print(f"Skipping section: {section_title}")
            continue

        if selected_section and section_title != selected_section:
            continue

        categories = fetch_categories(node)
        sections.append(Section(title=section_title, categories=categories))

    return sections


def fetch_categories(node):
    categories = []
    for li in node.select("ul.space-y-6 li"):
        anchor = li.select_one("a")
        print(f"Processing category: {anchor.get_text()}")

        category = fetch_category(anchor["href"])
        categories.append(category)

        # Wait for a random time between 4 and 7 seconds before making the next request
        # to avoid sending too many requests in a short period of time to the server
        wait_time = random.randint(4, 7)
        print(f"Waiting {wait_time} seconds before next request...")
        time.sleep(wait_time)

    return categories


# FIXME: Layout -> Container, gets the values of the “breakpoint” column as class names
# since the function does not retrieve the data considering the <table> element to group the data, so
# for the moment the additional saved data are manually removed from the final JSON.
def fetch_category(href):
    url = f"{base_url}{href}"
    request = requests.get(url)
    soup = BeautifulSoup(request.text, "html.parser")

    title = soup.select_one("h1.inline-block.text-2xl").get_text()
    description = soup.select_one("header.relative.z-20 p:nth-child(2)").get_text()

    variants = []
    for node in soup.select("tbody.align-baseline tr"):
        variant_entry = Variant(
            class_name=node.select_one("td:nth-child(1)")
            .get_text()
            .replace(" > * + *", ""),
            properties=node.select_one("td:nth-child(2)").get_text().rstrip("\n"),
        )
        variants.append(variant_entry)

    return Category(title=title, url=url, description=description, variants=variants)


def write_all_sections_to_json(sections):
    if os.path.exists(output_file):
        with open(output_file, "r") as file:
            existing_data = json.load(file)
    else:
        existing_data = []

    for new_section in sections:
        section_found = False
        for existing_section in existing_data:
            if existing_section["title"] == new_section.title:
                existing_section["categories"] = new_section.to_dict()["categories"]
                section_found = True
                break

        if not section_found:
            existing_data.append(new_section.to_dict())

    with open(output_file, "w") as file:
        json.dump(existing_data, file, indent=4)

    print(f"All sections written to {output_file}")


if __name__ == "__main__":
    print("Choose an option:")
    print("1. Fetch categories from a specific section")
    print("2. Fetch all categories (skipping configured sections)")

    choice = input("Enter your choice (1 or 2): ").strip()

    if choice == "1":
        selected_section = input(
            "Enter the name of the section you want to fetch: "
        ).strip()
        sections = fetch_sections(selected_section=selected_section)
        write_all_sections_to_json(sections)

    elif choice == "2":
        sections = fetch_sections()
        write_all_sections_to_json(sections)
    else:
        print("Invalid choice. Please run the program again.")
