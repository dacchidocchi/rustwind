import requests
import random
import time
from bs4 import BeautifulSoup
from mako.template import Template
from models import Category, Variant, Section, Enum
from utils import to_pascal_case, to_snake_case, get_enum_variants


base_url = "https://tailwindcss.com"


def fetch_sections(selected_section=None):
    request = requests.get(f"{base_url}/docs/installation")
    soup = BeautifulSoup(request.text, 'html.parser')

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


def fetch_category(href):
    url = f"{base_url}{href}"
    request = requests.get(url)
    soup = BeautifulSoup(request.text, 'html.parser')

    title = soup.select_one("h1.inline-block.text-2xl").get_text()
    description = soup.select_one(
        "header.relative.z-20 p:nth-child(2)").get_text()

    variants = []
    for node in soup.select("tbody.align-baseline tr"):
        variant_entry = Variant(
            name=node.select_one("td:nth-child(1)").get_text(),
            properties=node.select_one("td:nth-child(2)").get_text().rstrip("\n")
        )
        variants.append(variant_entry)

    return Category(title=title, url=url, description=description, variants=variants)


def write_section_to_file(section):
    filename = "".join(c for c in section.title if c.isalnum()
                       or c in (' ', '_')).replace(" ", "_")

    enums = []
    for category in section.categories:
        enum_variants = get_enum_variants(category.variants)
        enum = Enum(
            docs=category.description,
            url=category.url,
            name=to_pascal_case(category.title),
            variants=enum_variants
        )
        enums.append(enum)

    generated_code = Template(filename='template.mako')
    with open(f"../typewind/{to_snake_case(section.title)}.rs", 'w') as f:
        f.write(generated_code.render(enums=enums))

    print(f"Section written to {filename}")


if __name__ == "__main__":
    print("Choose an option:")
    print("1. Fetch categories from a specific section")
    print("2. Fetch all categories (skipping configured sections)")

    choice = input("Enter your choice (1 or 2): ").strip()

    if choice == "1":
        selected_section = input(
            "Enter the name of the section you want to fetch: ").strip()
        sections = fetch_sections(selected_section=selected_section)
        for section in sections:
            write_section_to_file(section)

    elif choice == "2":
        fetch_sections()
    else:
        print("Invalid choice. Please run the program again.")
