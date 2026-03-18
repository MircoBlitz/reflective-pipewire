---
name: icon_style_reflective
description: Verbindlicher Stilplan für alle Icons – Layout, Proportionen, Reflection, Regeln
type: feedback
---

# Icon-Stilplan "Reflective"

## Canvas
- ViewBox: 0 0 144 144
- Hintergrund: `<rect width="144" height="144" rx="16" fill="#0a0a0a"/>`

## Layout (fest, identisch für JEDES Icon)
- **Haupticon**: y=8 bis y=82 (obere Zone, nach oben ausgerichtet)
- **Mirror-Line**: y=90, x1=20 x2=124, stroke=#ffffff, stroke-width=1, opacity=0.15
- **Reflection**: ab y=98, unteres Drittel, wird vom Rand natürlich abgeschnitten

## Gruppen-Opacity (keine Einzel-Opacities!)
- Haupticon: `<g opacity="0.85">` – alle Elemente darin opacity=1
- Reflection: `<g opacity="0.15">` – alle Elemente darin opacity=1
- **Grund:** Halbtransparente Flächen dürfen sich NICHT überlappen und doppelt auftragen. Gruppen-Opacity verhindert das.

## Stil
- Geometrische Formen: rects, polygons, arcs, lines – KEINE FontAwesome-Paths
- Stroke-width: 3 für Hauptelemente
- Farbe: monochrom #ffffff (per State überschreibbar)
- Formen leicht abgerundet (rx=4 bei rects, stroke-linecap="round")
- Schlanke Proportionen (z.B. Mic-Kapsel 24px breit bei 144er Canvas)

## Ausrichtung
- Motiv zeigt/strahlt nach oben (Wellen, Spitzen etc.)
- **Vertikale Zentrierung: Icon-Mittelpunkt MUSS bei y=46 liegen** (= Mittelpunkt des Strike-through)
- Das Icon verteilt sich symmetrisch um y=46 innerhalb der Zone y=8–82

## Reflection-Regeln
- Exakte Y-Spiegelung um y=90: reflected_y = 180 - original_y
- Nur das sichtbar was ab y=98 in den Canvas passt – Rest wird natürlich abgeschnitten
- Reflection muss alle Hauptelemente enthalten (gespiegelt), nicht nur Teile davon
- Kürze Wellen/Details die komplett außerhalb des Canvas liegen

## Deactivated-Variante
- Haupticon-Gruppe: opacity 0.35 statt 0.85
- Reflection-Gruppe: opacity 0.08 statt 0.15
- Rote Durchstreich-Linie: FESTE Position, identisch bei JEDEM Icon:
  `<line x1="40" y1="14" x2="104" y2="78" stroke="#ff4444" stroke-width="3.5" opacity="0.85" stroke-linecap="round"/>`
- Reflection der Durchstreich-Linie ebenfalls fest:
  `<line x1="40" y1="166" x2="104" y2="102" stroke="#ff4444" stroke-width="3.5" stroke-linecap="round"/>`
- Die Durchstreich-Linie sitzt IMMER an derselben Stelle, Icons richten sich danach.

## Referenz-Dateien
- `assets/experiment/mic.svg` – Template/Referenz für Placement
- `assets/experiment/mic_deactivated.svg` – Deactivated-Referenz
- `assets/experiment/volume.svg` – Zweites Icon im Stil

**Why:** Konsistentes, auf Stream Deck gut erkennbares Icon-System. Geometrischer Stil passend zum PipeWire-Plugin-Logo.
**How to apply:** Jedes neue Icon exakt nach diesem Schema bauen. mic.svg ist das Placement-Template.