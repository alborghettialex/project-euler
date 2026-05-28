import html
import re
from pathlib import Path

SRC = Path("src/bin")
OUT = Path("generated")
TEMPLATES = Path("templates")

OUT.mkdir(exist_ok=True)

problem_template = (TEMPLATES / "problem.html").read_text()
index_template = (TEMPLATES / "index.html").read_text()


def parse_header(source: str):
    lines = source.splitlines()

    header = []
    for line in lines:
        if line.startswith("//"):
            header.append(line[2:].strip())
        else:
            break

    return "\n".join(header)


def extract(field: str, text: str, default=""):
    match = re.search(rf"{field}\s*:*\s*(.+)", text)
    return match.group(1).strip() if match else default


items = []

for file in sorted(SRC.glob("*.rs"), key=lambda p: int(p.stem)):
    source = file.read_text()

    header = parse_header(source)
    number = extract("#", header)
    answer = extract("Answer", header)
    title = f"Problem {number}"
    description = header

    description = re.sub(r"#\s*\d+", "", description)
    description = re.sub(r"Answer\s*:\s*.+", "", description).strip()

    if description.strip():
        description_html = (
            "<p>"
            + "</p><p>".join(
                [p.strip() for p in description.split("\n\n") if p.strip()]
            )
            + "</p>"
        )
    else:
        description_html = "<p>No description.</p>"

    code_html = html.escape(source)

    page = (
        problem_template.replace("{{NUMBER}}", number)
        .replace("{{TITLE}}", title)
        .replace("{{ANSWER}}", answer)
        .replace("{{DESCRIPTION}}", description_html)
        .replace("{{CODE}}", code_html)
    )

    output_name = f"{number}.html"
    (OUT / output_name).write_text(page)

    items.append(f"""
<a class="problem-card" href="{output_name}">
    <div class="problem-number">#{number}</div>
    <div class="problem-title">{title}</div>
</a>
""")

index = index_template.replace("{{LIST}}", "\n".join(items)).replace(
    "{{COUNT}}", str(len(items))
)

(OUT / "index.html").write_text(index)
