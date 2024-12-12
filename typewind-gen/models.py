class Section:
    # "Layout",
    # [Category("Aspect Ratio", "...", [Variant(name="aspect-auto", properties="aspect-ratio: auto;")])]
    def __init__(self, title, categories):
        self.title = title
        self.categories = categories


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
    def __init__(self, name, properties):
        self.name = name
        self.properties = properties


class Enum:
    # "Utilities for controlling the aspect ratio of an element.",
    # "https://tailwindcss.com/docs/aspect-ratio",
    # "AspectRatio",
    # [...]
    def __init__(self, docs, url, name, variants):
        self.docs = docs
        self.url = url
        self.name = name
        self.variants = variants


class EnumVariant:
    # "Auto", "aspect-auto", "aspect-ratio: auto;"
    def __init__(self, key, value, docs):
        self.key = key
        self.value = value
        self.docs = docs
