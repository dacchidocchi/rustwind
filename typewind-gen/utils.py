import re
from models import EnumVariant


def to_pascal_case(text):
    return ''.join(word.capitalize() for word in text.split())


def to_snake_case(text):
    text = re.sub(r'[\s-]+', '_', text)
    return re.sub(r'([A-Z])', r'_\1', text).lower().lstrip('_')


# FIXME: Isolation (isolate, isolation-auto) returns a wrong prefix -> "isolat"
# Width - Height has 1 char prefix "W" and "H" respectively
def find_common_prefix(variants):
    if not variants:
        return ""
    shortest = min(variants, key=len)
    for i, char in enumerate(shortest):
        if any(variant[i] != char for variant in variants):
            # Returns the prefix only if it contains more than 2 characters,
            # this is to avoid cases like in the Padding category:
            # "p-0", "px-0", "py-0", "ps-0"...
            return shortest[:i].rsplit('-', 1)[0] if i > 2 else ""

    return shortest if len(shortest) > 2 else ""


def get_enum_variants(variants):
    """Returns a list of EnumVariant objects from a list of Variant objects."""
    prefix = find_common_prefix([variant.name for variant in variants])

    enum_variants = []
    for variant in variants:
        variant.name.replace("> * + *", "")
        variant_name = variant.name[len(prefix):].lstrip(
            '-') if prefix else variant.name

        # e.g. prefix: "transition", variant name: "transition"
        if variant_name == '':
            variant_name = variant.name
        # e.g. "100" -> "_100"
        elif variant_name[0].isdigit():
            variant_name = '_' + variant_name

        variant_name = (
            variant_name.replace('-', ' ')  # e.g. h-4 -> _4
            .replace('.', '_')  # e.g. h-0.5 -> _0_5
            .replace('/', 'over')  # e.g. w-1/2 -> _1over2
            .replace('%', "percent")  # e.g. from-0% -> 0percent
        )
        enum_variant = EnumVariant(key=to_pascal_case(
            variant_name), value=variant.name, docs=variant.properties)
        enum_variants.append(enum_variant)

    return enum_variants
