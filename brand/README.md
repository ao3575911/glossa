# Glossa brand

**Manuscript terminal** identity: engraved capital **G**, cool vellum paper, graphite ink, iron-blue accent.

Tagline: *every linguist's research tool, in one fast terminal workspace.*

## Tokens

See [`tokens.json`](./tokens.json).

| Token | Hex |
|-------|-----|
| vellum | `#E8E6E1` |
| graphite | `#1C1F24` |
| iron-blue | `#2F4A6E` |
| rule | `#9AA1AB` |
| ink-muted | `#5C6370` |

## Assets

| Path | Use |
|------|-----|
| `icon/glossa-mark.svg` | Master mark (light) |
| `icon/glossa-mark-dark.svg` | Mark on graphite |
| `icon/glossa-mark.png` | Raster master 1024 |
| `icon/glossa-mark-{16..512}.png` | Favicon / app sizes |
| `logo/glossa-horizontal.svg` | G + wordmark |
| `logo/glossa-stacked.svg` | Square lockup |
| `letterhead/glossa-letterhead.svg` | A4 print + export chrome |
| `social/github-og.svg` | Vector OG |
| `social/github-og.png` | GitHub social preview (1280×640) |
| `terminal/banner-compact.txt` | ASCII banner source |

## Naming

- Prose / product: **Glossa**
- Binary / wordmark: **`glossa`**

## In-terminal

Quiet by default. Compact banner via:

```bash
glossa about
GLOSSA_BANNER=1 glossa
```

Respects `--no-color` and `NO_COLOR`.

## GitHub social image

Repo → Settings → Social preview → upload `social/github-og.png`.

## Do not

- Purple gradients, glow, cream+#terracotta stacks
- Encode meaning by colour alone
- Flourish the G into anatomy or calligraphy that fails at 16×16
