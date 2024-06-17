# Software Quality

Software quality is an extremely broad topic, for this document I'm going to focus on a rubric for production-grade systems.

## Merge/Pull Request Policy

Ensure that your CLs:

- have a good [Description](https://google.github.io/eng-practices/review/developer/cl-descriptions.html)
- are [Small CLs](https://google.github.io/eng-practices/review/developer/small-cls.html)

When responding to a review, ensure that you [handle comments appropriately](https://google.github.io/eng-practices/review/developer/handling-comments.html).

## Code Quality

### Style

Follow the [Rust Style Guide](https://rust-lang.github.io/api-guidelines/checklist.html)

We should strive for a code style that is consistent with the rest of the ecosystem. This means that if there is a standard used in the Rust community, we should use it.

Specifics:

- Although there is a lint in the root [Cargo.toml](/Cargo.toml) that enforces this, root Error types from crates should be exported as `Error`, likewise root result types should be exported as `Result`.

### Linting

The lints from the root [Cargo.toml](/Cargo.toml) should be used as indicated therein.

### Formatting

We use the nightly rust formatter because the rustfmt team has indicated they are not interested publishing the new formatting options to stable rust.

While normally we would not want to use a nightly toolchain, this is a dev tool and can be though of as a just a binary one would install on their dev machine. The reason to go to all this trouble is to further eliminate any variation in formatting between developers.
