# AXAML for Zed

[Zed](https://zed.dev) support for **AXAML**, the XAML of [Avalonia®](https://avaloniaui.net) apps:

- **Syntax highlighting**: tree-sitter XML, with AXAML-aware queries (`xmlns`, `x:` directives, attached properties,
  property elements, `{Markup}` values), plus outline, brackets, indents and text objects.
- **Language features** from [axaml-ls](https://github.com/Engel-167/axaml-ls): diagnostics that match Avalonia's XAML
  compiler, completion of bindings, types, properties, resources, style selectors and file paths, hover,
  go-to-definition, references, rename, quick fixes, inlay hints, color swatches and more. See its README for the full
  list.

For a live preview of your views beside Zed, see [axaml-previewer](https://github.com/Engel-167/axaml-previewer).

## Requirements

The .NET SDK (10 or later) on the PATH of your project: the language server loads your project through MSBuild.
`dotnet` from mise, asdf or direnv works, since the extension uses the project's shell environment.

## Language server

The extension starts `axaml-ls` from the first of:

1. `lsp.axaml-ls.binary.path` in your settings,
2. `axaml-ls` on the PATH (a local build: `mise run install` in the axaml-ls repository),
3. the latest [axaml-ls release](https://github.com/Engel-167/axaml-ls/releases), downloaded on first use and run with
   your `dotnet`. It checks for a newer release when Zed starts the server; offline it keeps the one it has.

```jsonc
// ~/.config/zed/settings.json (optional)
{
  "lsp": {
    "axaml-ls": {
      "binary": { "path": "/path/to/axaml-ls", "arguments": ["--verbose"] },
      "initialization_options": { "verbose": true }
    }
  }
}
```

## Settings

Selector coloring uses LSP semantic tokens, which Zed turns off by default. Turn them on for AXAML:

```jsonc
// ~/.config/zed/settings.json
{
  "languages": {
    "AXAML": { "semantic_tokens": "combined" }
  }
}
```

The extension's `languages/axaml/semantic_token_rules.json` maps the token types (`selectorType`, `styleClass`, `styleClassUnused`, `pseudoClass`, `templateSelector`, `selectorName`, `nestingSelector`) to theme styles; override them with `global_lsp_settings.semantic_token_rules`. Unused classes are faded even with semantic tokens off.

Outline/breadcrumbs, folding and code lenses from the language server are also off by default in Zed:

```jsonc
// ~/.config/zed/settings.json
{
  "inlay_hints": { "enabled": true },
  "code_lens": "on",
  "languages": {
    "AXAML": { "document_symbols": "on", "document_folding_ranges": "on", "semantic_tokens": "combined" }
  }
}
```

Options the server reads from `lsp.axaml-ls.initialization_options`:

```jsonc
{
  "lsp": {
    "axaml-ls": {
      "initialization_options": {
        "verbose": false,
        "inlayHints": { "bindingSource": true }   // false hides the (MainViewModel). hint before binding paths
      }
    }
  }
}
```

Server logs: **`dev: open language server logs`** → AXAML Language Server.

## Development

```sh
mise install          # Rust with the wasm32-wasip2 target
mise run build        # compile the extension (a quick check; Zed builds it itself)
```

In Zed, run **`zed: install dev extension`** and pick this folder.

## License

MIT, see [LICENSE](LICENSE).

Avalonia is a registered trademark of AvaloniaUI OÜ. This project is not affiliated with or endorsed by AvaloniaUI OÜ.
All other trademarks are the property of their respective owners.
