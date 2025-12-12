import datetime as dt
import re
import sys
from pathlib import Path

from invoke import task

ROOT = Path(__file__).parent


def get_version() -> str:
    for line in (ROOT / "Cargo.toml").read_text("utf-8").splitlines():
        if line.startswith("version ="):
            return line.replace("version = ", "").strip('"')

    raise RuntimeError("Could not determine version")


def replace_pattern_in_file(file: Path, old: str, new: str, count: int = 1):
    content = file.read_text("utf-8")
    updated = re.sub(old, new, content, count=count)
    file.write_text(updated, "utf-8")


def confirm(prompt: str):
    response = input(f"Confirm by typing '{prompt}': ")
    if response.lower() != prompt.lower():
        sys.exit(1)


@task
def version(ctx):
    print(get_version())


@task
def prerelease(ctx, new_version, update_lang=True):
    date = dt.datetime.now().strftime("%Y-%m-%d")

    replace_pattern_in_file(
        ROOT / "Cargo.toml",
        'version = ".+"',
        f'version = "{new_version}"',
    )

    replace_pattern_in_file(
        ROOT / "CHANGELOG.md",
        "## Unreleased",
        f"## v{new_version} ({date})",
    )

    # Update version in Cargo.lock
    ctx.run("cargo build")


@task
def release(ctx):
    version = get_version()

    confirm(f"release {version}")

    ctx.run(f'git commit -m "Release v{version}"')
    ctx.run(f'git tag v{version} -m "Release"')
    ctx.run("git push")
    ctx.run(f"git push origin tag v{version}")
