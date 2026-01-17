Hallways
=====

<p align="center">
  <img src="asset/release/logo.png" alt="Hallways logo" width="220" />
</p>

Hallways is a free, open-source, first-person "web browser" for 3D spaces. Instead of hyperlinks, spaces are connected by portals that can be seamlessly walked through with no interruption/loading screen.

## Usage

Download a prebuilt binary from [Releases](https://github.com/tlonny/hallways/releases) for Linux and Windows.

On macOS, build from source:

```
cargo run --bin hallways --release
```

## Navigating Levels

- Double-tap jump to enter god mode.
- Press jump again to cancel god mode.

## Level Creation

A level is a collection of assets linked together by a `manifest.json`. The manifest schema:

```json
{
    "_version": "coco",
    "meta": {
        "name": "My Level",
        "author": "Your Name"
    },
    "level": {
        "model": "model.glb",
        "collider": "collision.glb",
        "material": {
            "MyMaterial": {
                "frames": ["texture1.png", "texture2.png"],
                "animation_speed": 1.0,
                "color": [255, 255, 255, 255],
                "texture_addressing": "Linear"
            }
        }
    },
    "portal": {
        "portal_a": {
            "collider": "portal_a.glb",
            "target": { "href": "other_level.json", "name": "portal_b" }
        }
    }
}
```

Required fields:

- `_version` (must be `"coco"`)
- `meta.name`
- `level.model`
- `level.material`
- `portal`

Optional fields:

- `meta.author`
- `level.collider`
- `level.spawn`
- `level.track`

Limits:

- `portal` may be empty (`{}`), but cannot contain more than 4 entries.

### Manifest Fields

- `meta.name`: level name shown in UI.
- `meta.author`: optional author credit shown in UI.
- `level.model`: level render model (`.glb`).
- `level.collider`: optional separate collision model (`.glb`). If omitted, `level.model` is used for collision.
- `level.spawn`: optional player spawn position `[x, y, z]` (defaults to origin).
- `level.track`: optional background music file.
- `level.material`: required material map keyed by glTF material name.
- `portal`: required portal map (can be empty), max 4 entries.
- `portal.<name>.collider`: portal collider (`.glb`).
- `portal.<name>.target`: optional destination object with `href` and `name`.

### Track Format

- `level.track` must point to an Ogg container with Vorbis audio codec (`.ogg` + Vorbis).

### Material Fields

Each `level.material.<material_name>` entry uses one shape:

```json
{
    "frames": ["frame1.png", "frame2.png"],
    "animation_speed": 1.0,
    "color": [255, 255, 255, 255],
    "texture_addressing": "Linear"
}
```

Rules and defaults:

- `frames` is optional; if omitted it defaults to `[]`.
- `animation_speed` is optional (defaults to `0.5`).
- `color` is optional (defaults to white).
- `texture_addressing` is optional (`"Linear"` or `"Nearest"`); defaults to `"Linear"`.
- Transparency: diffuse alpha is evaluated after texture + `color` tinting.
- If final diffuse alpha is `0.0`, the fragment is treated as a cut-out (`discard`).
- If final diffuse alpha is `1.0`, the fragment is treated as opaque.
- If final diffuse alpha is between `0.0` and `1.0`, the fragment is treated as transparent and rendered with OIT.

### Material Mapping

- `level.material` keys should match material names in the level `model.glb`.
- Ensure every material referenced by your model has a matching entry.

### Texture Constraints

Texture dimensions must be one of the following sizes, each with a maximum number of textures per level:

| Size      | Max |
|-----------|-----|
| 2048x2048 | 1   |
| 1024x1024 | 4   |
| 512x512   | 8   |
| 256x256   | 32  |
| 128x128   | 64  |
| 64x64     | 256 |

### Portals

- Portal geometry can be any coplanar polygon.
- Portals must be either **Horizontal** (wall-aligned, vertical surface) or **Vertical** (floor/ceiling-aligned, horizontal surface).
- When using vertical portals, a single vertex must be colored magenta (`#FF00FFFF`) in order for portal-induced player rotation to be derived.
- The `target.href` field is a relative URL to the destination manifest.
- The `target.name` field is the destination portal name in that manifest.

#### Linking Criteria

- **Horizontal** portals can only link to **horizontal** portals.
- **Vertical** portals can only link to **vertical** portals, and their normals must match.
- Linked portals should have the same polygon shape. Shape compatibility is not validated at runtime.

### Tips

- Keep vertex counts low — every vertex is processed per frame.
- Avoid geometric seams — vertices that should meet must share the exact same position. Small gaps or overlaps cause collision detection issues.
- Keep open space on both sides of each portal. The teleport only triggers after the player has already crossed the portal plane, so blocking geometry too close to either face can prevent crossing.
- Use a separate `level.collider` mesh for complex scenes. This lets you keep detailed visuals in `level.model` without forcing all visual geometry to be collidable.

## Thanks

- [JDWasabi](https://jdwasabi.itch.io/8-bit-16-bit-sound-effects-pack) — Sound effects
- [Jayvee Enaguas](https://www.dafont.com/pixel-operator.font) — Font
- [Ji-Hoon Myung](https://github.com/edwardmyung) - SVG logo
