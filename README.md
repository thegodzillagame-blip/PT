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
| ftilt push kick | 9-11 13% (+body 11%), 12-14 9% | lunging 361° shove, quake on hit |
| angled ftilt | Charizard's | tip moved off the parked fire bone, no fire effect |
| utilt tail flip (Yoshi style) | 6 7%, 7-8 9%, 9-10 8% | tail whips from behind up over his head; Charizard's smash arc trails it |
| usmash Guillotine | charge f11, scoop 14-15 16%, chop 16 18%, late 17-18 13% | tusk chop front -> overhead -> behind; Yoshi arc trails it |
| dsmash Dragon Tail | charge f7, tail sweep 14-25, stomp 29-30 | 15% low horizontal (30°) both sides |
| nair Roar | burst 8-11 11%, ring 12-20 6% | autocancel from f40, half landing lag (config_param.toml), rings sized to the hitbox |
| bair dropkick | 8-9 15%, 10-12 11% | |
| uair tusk swipe | 10-14 13% | |
| uthrow Dragon Pulse | release f50, 10% | plays 1.67x speed, recovery 2.2x |
| fair | Charizard's hits | endlag 1.67x faster |
| dthrow Body Press | release f36 (belly lands), 7% | quick get-up after |
| get-ups | Charizard's | tail tip moved off the fire bone |
| tail flame | killed every frame | backup for the effect-file edit |

| neutral B Dragon Dance | hold to dance 60f+ | release: +30% movement speed for 5 s and +40% damage on the NEXT attack only; no breath |
| side B Outrage | 2% autolink rakes (rehit 4), finisher 7% (bkb 85) | 65% of Flare Blitz's speed, short run; ends after 4 rakes; no explosion/recoil/blow-back; charging run on the ground, flat body in the air; purple aura from mouth and claws |
| up B Giga Impact | 3% scoop f9, 0.8% rakes f13-41, 4% down-knock f42 (+30f stun) | cuts to the dive at f48; dive falls 4.5/frame from frame 1 with an 8% spike; crash 8% launches |

## Building
Push to GitHub - `.github/workflows/build.yml` builds the `.nro` with cargo-skyline.
