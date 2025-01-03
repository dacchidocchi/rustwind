class Section:
    # "Layout",
    # [Category("Aspect Ratio", "...", [Variant(name="aspect-auto", properties="aspect-ratio: auto;")])]
    def __init__(self, title, categories):
        self.title = title
        self.categories = categories

    def to_dict(self):
        return {
            "title": self.title,
            "categories": [
                {
                    "url": category.url,
                    "title": category.title,
                    "description": category.description,
                    "variants": [
                        {
                            "class": variant.class_name,
                            "properties": variant.properties,
                        }
                        for variant in category.variants
                    ],
                }
                for category in self.categories
            ],
        }


class Category:
    # "https://tailwindcss.com/docs/aspect-ratio",
    # "Aspect Ratio",
    # "Utilities for controlling the aspect ratio of an element.",
    # [Variant(name="aspect-auto", properties="aspect-ratio: auto;")]
    def __init__(self, url, title, description, variants):
        self.url = url
        self.title = title
        self.description = description
        self.variants = variants


class Variant:
    # "aspect-auto", "aspect-ratio: auto;"
    def __init__(self, class_name, properties):
        self.class_name = class_name
        self.properties = properties
