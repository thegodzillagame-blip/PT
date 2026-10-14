# PT team (c01) - move code

Smashline 2 plugin for the Pokémon Trainer costume-2 team (Iris / Scraggy / Bisharp / Haxorus).
Every script is installed for **costume slot c01 only**, so the other Trainer costumes keep vanilla behaviour.

## Install (emulator or Switch)
Both files go in the Smash plugins folder on the (emulated) SD card:
`atmosphere/contents/01006A800016E000/romfs/skyline/plugins/`

1. `libsmashline_plugin.nro` - Smashline 2 itself (HDR-Development/smashline releases). Needed once.
2. `libpt_team_c01.nro` - this plugin (built by GitHub Actions: Actions tab -> latest run -> Artifacts).

ARCropolis + Skyline must already be installed (they are, since the mod loads).

## What's in it
### Haxorus (Charizard slot, `src/haxorus`)
| move | frames (anim) | notes |
|---|---|---|
| jab 1/2 Scratch | hit 5-6 | 3% each, purple claw swipes |
| jab 3 X-Scissor | hit 9-11 | 6%, 50° |
| dash Take Down | 7-10 clean 13%, 11-22 late 9% | rides the Trans bone (moves with his travel) |
| ftilt push kick | 9-11 12%, 12-13 8% | horizontal 361° push |
| angled ftilt | Charizard's | tip moved off the parked fire bone, no fire effect |
| utilt Dragon Claw | 7-13 | front -> overhead -> behind |
| usmash Guillotine | charge f11, hit 15-19 | 16-17% tusk chop |
| dsmash Dragon Tail | charge f7, tail sweep 14-25, stomp 29-30 | 15% low horizontal (30°) both sides |
| nair Roar | burst 8-11 11%, ring 12-20 6% | autocancel from f40 |
| bair dropkick | 8-9 15%, 10-12 11% | |
| uair tusk swipe | 10-14 13% | |
| uthrow Dragon Pulse | release f50, 10% | beam column also hits bystanders |
| dthrow Body Press | release f55, 7% | belly-flop shockwave hits bystanders |
| get-ups | Charizard's | tail tip moved off the fire bone |
| tail flame | killed every frame | backup for the effect-file edit |

Not yet: specials (Dragon Dance / Outrage / Giga Impact still run Charizard's statuses), nair facing-left camera turn.

## Building
Push to GitHub - `.github/workflows/build.yml` builds the `.nro` with cargo-skyline.
