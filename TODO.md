# libuniverse — Missing Functions vs libnova

This file tracks functions present in [libnova](http://libnova.sourceforge.net/) that are not yet
implemented in libuniverse. Grouped by module. Items are ordered roughly by
implementation priority (foundational algorithms first).

---

## Legend

- [ ] = not started
- [~] = partially implemented (notes inline)

---

## Calendar / Date Utilities

| Status | libnova function | Description |
|--------|-----------------|-------------|
| [~] | `ln_get_julian_day` | Implemented as `get_julian_day()` / `Date::to_julian_day()` |
| [~] | `ln_get_date` | Implemented as `JulianDay::to_calendar_date()` |
| [~] | `ln_get_day_of_week` | Implemented as `Date::week_day()` (1=Mon … 7=Sun vs libnova's 0=Sun) |
| [ ] | `ln_get_julian_from_sys` | Get Julian Day from the local system clock |
| [ ] | `ln_get_date_from_sys` | Get calendar date from the local system clock |
| [ ] | `ln_get_julian_from_timet` | Convert a Unix `time_t` value to Julian Day |
| [ ] | `ln_get_timet_from_julian` | Convert a Julian Day to Unix `time_t` |
| [ ] | `ln_get_julian_local_date` | Julian Day from a timezone-aware local date |
| [ ] | `ln_get_date_from_mpc` | Parse an MPC packed date string into a calendar date |
| [ ] | `ln_get_julian_from_mpc` | Parse an MPC packed date string directly to Julian Day |

---

## Dynamical Time

| Status | libnova function | Description |
|--------|-----------------|-------------|
| [~] | `ln_get_dynamical_time_diff` | Implemented as `delta_t()` |
| [ ] | `ln_get_jde` | Compute Julian Ephemeris Day (JDE = JD + ΔT/86400) |

---

## Sidereal Time

| Status | libnova function | Description |
|--------|-----------------|-------------|
| [~] | `ln_get_mean_sidereal_time` | Implemented as `get_mean_sidereal_time_from_date()` |
| [ ] | `ln_get_apparent_sidereal_time` | Apparent sidereal time (mean + equation of equinoxes from nutation) |

---

## Nutation

| Status | libnova function | Description |
|--------|-----------------|-------------|
| [ ] | `ln_get_nutation` | Nutation in longitude (Δψ) and obliquity (Δε) from JDE — Meeus ch. 22 |

---

## Aberration

| Status | libnova function | Description |
|--------|-----------------|-------------|
| [ ] | `ln_get_equ_aber` | Equatorial coordinates corrected for annual aberration — Meeus ch. 23 |
| [ ] | `ln_get_ecl_aber` | Ecliptical coordinates corrected for annual aberration |

---

## Precession

| Status | libnova function | Description |
|--------|-----------------|-------------|
| [ ] | `ln_get_equ_prec` | Precess equatorial coordinates from a given epoch to J2000 — Meeus ch. 21 |
| [ ] | `ln_get_equ_prec2` | Precess equatorial coordinates between two arbitrary epochs |
| [ ] | `ln_get_ecl_prec` | Precess ecliptical coordinates — Meeus ch. 21 |

---

## Coordinate Transformations

| Status | libnova function | Description |
|--------|-----------------|-------------|
| [~] | *(DMS/HMS conversions)* | `ra_to_deg`, `deg_to_ra`, `dec_to_deg`, `deg_to_dms` implemented |
| [ ] | `ln_get_hrz_from_equ` | Equatorial → Horizontal (altitude/azimuth) — Meeus ch. 13 |
| [ ] | `ln_get_hrz_from_equ_sidereal_time` | Same, accepting pre-computed mean sidereal time |
| [ ] | `ln_get_equ_from_hrz` | Horizontal → Equatorial |
| [ ] | `ln_get_equ_from_ecl` | Ecliptical → Equatorial — Meeus ch. 13 |
| [ ] | `ln_get_ecl_from_equ` | Equatorial → Ecliptical |
| [ ] | `ln_get_rect_from_helio` | Heliocentric → Geocentric rectangular coordinates |
| [ ] | `ln_get_ecl_from_rect` | Rectangular → Ecliptical coordinates |
| [ ] | `ln_get_equ_from_gal` | Galactic → B1950 equatorial coordinates |
| [ ] | `ln_get_equ2000_from_gal` | Galactic → J2000 equatorial coordinates |
| [ ] | `ln_get_gal_from_equ` | B1950 equatorial → Galactic coordinates |
| [ ] | `ln_get_gal_from_equ2000` | J2000 equatorial → Galactic coordinates |

---

## Angular Separation

| Status | libnova function | Description |
|--------|-----------------|-------------|
| [ ] | `ln_get_angular_separation` | Angular separation between two equatorial positions — Meeus ch. 17 |
| [ ] | `ln_get_rel_posn_angle` | Relative position angle between two equatorial positions |

---

## Atmospheric Refraction

| Status | libnova function | Description |
|--------|-----------------|-------------|
| [ ] | `ln_get_refraction_adj` | Altitude correction for atmospheric refraction given pressure & temperature — Meeus ch. 16 |

---

## Apparent Position of a Star

| Status | libnova function | Description |
|--------|-----------------|-------------|
| [ ] | `ln_get_apparent_posn` | Full apparent position: proper motion + precession + nutation + aberration |

---

## Heliocentric Time

| Status | libnova function | Description |
|--------|-----------------|-------------|
| [ ] | `ln_get_heliocentric_time_diff` | Barycentric/heliocentric time correction for a given object and date |

---

## Lunar (Moon)

| Status | libnova function | Description |
|--------|-----------------|-------------|
| [~] | `ln_get_lunar_equ_coords` | `get_moon_position()` returns RA/Dec — lower precision path still TODO |
| [~] | `ln_get_lunar_equ_coords_prec` | High-precision path — verify against Meeus ch. 47 full table |
| [~] | `ln_get_lunar_ecl_coords` | Ecliptic longitude/latitude returned by `get_moon_position()` |
| [~] | `ln_get_lunar_earth_dist` | Distance field already in `MoonPosition.distance` |
| [ ] | `ln_get_lunar_geo_posn` | Rectangular geocentric coordinates of the Moon |
| [ ] | `ln_get_lunar_phase` | Phase angle of the Moon — Meeus ch. 48 |
| [ ] | `ln_get_lunar_disk` | Illuminated fraction of the lunar disk |
| [ ] | `ln_get_lunar_bright_limb` | Position angle of the bright limb |
| [ ] | `ln_get_lunar_long_asc_node` | Longitude of the Moon's mean ascending node |
| [ ] | `ln_get_lunar_long_perigee` | Longitude of the Moon's mean perigee |
| [ ] | `ln_get_lunar_sdiam` | Semidiameter of the Moon in arc seconds |
| [ ] | `ln_get_lunar_rst` | Rise, set, and transit times for the Moon |

---

## Solar (Sun)

| Status | libnova function | Description |
|--------|-----------------|-------------|
| [ ] | `ln_get_solar_equ_coords` | Apparent equatorial coordinates of the Sun — Meeus ch. 25/27 |
| [ ] | `ln_get_solar_ecl_coords` | Apparent ecliptical coordinates of the Sun |
| [ ] | `ln_get_solar_geo_coords` | Geocentric rectangular coordinates of the Sun (AU) |
| [ ] | `ln_get_solar_geom_coords` | Geometric (high-precision VSOP87) position of the Sun |
| [ ] | `ln_get_solar_sdiam` | Semidiameter of the Sun in arc seconds |
| [ ] | `ln_get_solar_rst` | Rise, set, and transit times for the Sun |
| [ ] | `ln_get_solar_rst_horizon` | Rise/set over a custom horizon elevation |

---

## Planets

Each planet needs the same set of functions. libnova covers: Mercury, Venus,
Mars, Jupiter, Saturn, Uranus, Neptune, Pluto.

| Status | Function pattern | Description |
|--------|-----------------|-------------|
| [ ] | `get_<planet>_helio_coords` | Heliocentric coordinates (FK5) — VSOP87 |
| [ ] | `get_<planet>_equ_coords` | Equatorial coordinates |
| [ ] | `get_<planet>_earth_dist` | Planet–Earth distance (AU) |
| [ ] | `get_<planet>_solar_dist` | Planet–Sun distance (AU) |
| [ ] | `get_<planet>_magnitude` | Apparent visual magnitude |
| [ ] | `get_<planet>_disk` | Illuminated fraction of the disk |
| [ ] | `get_<planet>_phase` | Phase angle |
| [ ] | `get_<planet>_sdiam` | Semidiameter in arc seconds |
| [ ] | `get_<planet>_rst` | Rise, set, and transit times |
| [ ] | `get_<planet>_rect_helio` | Rectangular heliocentric coordinates (AU) |

Planets to implement: **Mercury, Venus, Mars, Jupiter, Saturn, Uranus, Neptune, Pluto**

---

## Earth

| Status | libnova function | Description |
|--------|-----------------|-------------|
| [ ] | `ln_get_earth_helio_coords` | Heliocentric coordinates of Earth (VSOP87) |
| [ ] | `ln_get_earth_solar_dist` | Earth–Sun distance (AU) |
| [ ] | `ln_get_earth_rect_helio` | Rectangular heliocentric coordinates of Earth |

---

## Rise, Set, and Transit

| Status | libnova function | Description |
|--------|-----------------|-------------|
| [ ] | `ln_get_object_rst` | Generic rise/set/transit for any celestial object — Meeus ch. 15 |
| [ ] | `ln_get_object_next_rst` | Next rise/set/transit after a given JD |
| [ ] | `ln_get_object_rst_horizon` | Rise/set over a custom horizon elevation |

---

## Orbital Mechanics

| Status | libnova module | Description |
|--------|---------------|-------------|
| [ ] | Elliptic motion | Position/velocity on an elliptic orbit — Meeus ch. 30 |
| [ ] | Parabolic motion | Position on a parabolic (cometary) orbit — Meeus ch. 34 |
| [ ] | Hyperbolic motion | Position on a hyperbolic orbit — Meeus ch. 35 |

---

## Asteroids & Comets

| Status | libnova module | Description |
|--------|---------------|-------------|
| [ ] | Asteroids | Heliocentric coords, magnitude, phase, RST for minor planets |
| [ ] | Comets | Heliocentric coords, RST, perihelion calculations |

---

## Miscellaneous Utilities

| Status | libnova function | Description |
|--------|-----------------|-------------|
| [~] | `ln_get_dec_location` | Partially covered by `HoursMinSec::from_string` / `DegMinSec::from_string` |
| [~] | `ln_get_humanr_location` | Partially covered by `Display` impls for `HoursMinSec` / `DegMinSec` |
| [ ] | `ln_interpolate3` | 3-point interpolation — Meeus ch. 3 |
| [ ] | `ln_interpolate5` | 5-point interpolation — Meeus ch. 3 |

---

## Implementation Notes

1. **Start with foundations** — Nutation (`delta_psi`, `delta_eps`) is a dependency of
   apparent sidereal time, aberration, and apparent stellar positions. Implement it first.
2. **Coordinate transforms** — `hrz_from_equ` and `equ_from_ecl` are broadly needed;
   implement these before planet modules.
3. **Solar position** is required by most planet magnitude/phase calculations.
4. **VSOP87** — Planet heliocentric coordinates require the VSOP87 series. Consider
   a separate module or feature flag for the large coefficient tables.
5. **Rise/Set/Transit** — Generic implementation in `src/rise_set.rs` can be reused by
   Sun, Moon, and all planets.
6. All new functions must cite the relevant Meeus chapter/equation in doc comments and
   include unit tests with values from Meeus' worked examples.
