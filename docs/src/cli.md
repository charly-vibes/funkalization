# CLI reference

Current shipped surface (scaffold):

```text
funkalization --version    # print `funkalization <semver>`
funkalization --help       # usage
```

Unknown commands exit 2 with an error — the workspace subcommands (`init`,
`propose`, `decide`, `report`, `views`, `export`) arrive gate by gate per the
[implementation contract](https://github.com/charly-vibes/funkalization/tree/main/openspec/design)
and will be documented here as they ship.
