# COSMIC Application Template

A template for developing applications for the COSMIC™ desktop environment using [libcosmic][libcosmic].

## Documentation

Refer to the [libcosmic API documentation][api-docs] and [book][book] for help with building applications with [libcosmic][libcosmic].

## Getting Started

To generate a new project based on this template, [install `cargo generate`][cargo-generate-install] and run:

```bash
cargo generate gh:pop-os/cosmic-app-template
```

Then follow the prompts to pre-populate and customize your project.

### Manual Template Configuration

If you choose not to use `cargo generate`, you may still use this template.

Log into your GitHub account and navigate to [this template there][app-template] and click the "Use this template" button above. This will create a new repository in your account. Choose a name for this repository, and then clone it locally onto your system. After cloning it, you must:

- Manually replace all template fields matching `{{ <some field> }}` (regex `\{\{ .*\}\}`) in all source files.
- Create a `LICENSE` file containing your chosen software license.
- Rename the file `i18n/en/cosmic_app_template.ftl` by replacing the `cosmic_app_template` portion with the new crate `name` you set in `Cargo.toml`.
- Set your license within the SPDX tags at the top of each source file
- Replace this `README.md` with your own, optionally based on `README.md.liquid`, removing the postfix.
- Delete the `generate/` directory.

[api-docs]: https://pop-os.github.io/libcosmic/cosmic/
[book]: https://pop-os.github.io/libcosmic-book/
[libcosmic]: https://github.com/pop-os/libcosmic/
[just]: https://github.com/casey/just
[cargo-generate-install]: https://cargo-generate.github.io/cargo-generate/installation.html
[app-template]: https://github.com/pop-os/cosmic-app-template
