---
name: color_palette
description: Farbpalette des Projekts – alle verwendeten Farben mit Rolle und Default-Status
type: reference
---

# Farbpalette Reflective PipeWire

## Basis
| Farbe | Hex | Rolle |
|-------|-----|-------|
| Schwarz | `#0a0a0a` | Hintergrund (Canvas, Knob-Innenfläche) |
| Weiß | `#ffffff` | Icon-Farbe (Standard), Mirror-Line, Reflection |

## Akzent – Deactivated
| Farbe | Hex | Rolle |
|-------|-----|-------|
| Rot (Strike) | `#ff4444` | Durchstreich-Linie bei deactivated Icons |

## Akzent – Volume-Stufen (Default, user-konfigurierbar)
| Stufe | Hex | Rolle |
|-------|-----|-------|
| Grün | `#22c55e` | Stufe 1 (leise / Ziel) |
| Gelbgrün | `#a3b532` | Stufe 2 |
| Orange | `#d98a3a` | Stufe 3 |
| Rot | `#ef4444` | Stufe 4 (laut / Warnung) |

## Akzent – Plugin-Logo (plugin.svg)
| Farbe | Hex | Rolle |
|-------|-----|-------|
| Blau | `#3b82f6` | Node 1, Verbindungslinie, Gradient |
| Grün | `#22c55e` | Node 2, Verbindungslinie |
| Lila | `#8b5cf6` | Node 3 (zentral) |

## Opacity-Stufen
| Wert | Verwendung |
|------|------------|
| 0.85 | Haupticon (aktiv) |
| 0.35 | Haupticon (deactivated) |
| 0.15 | Reflection (aktiv), Mirror-Line |
| 0.08 | Reflection (deactivated) |
| 0.85 | Strike-through Linie |