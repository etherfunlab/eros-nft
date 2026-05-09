# 03 — Compliance

The compliance subdocument is split into a fixed `core` block (the same shape
across all platforms) and an open `regional[]` array (platform-defined extensions).

## Spec posture

This specification defines only hard-ban acknowledgments and creator-attestation
slots. All `regional` pack contents are defined by implementations. **This
specification makes no statement about, and offers no guarantee of compliance
with, any specific jurisdiction's regulations.** Implementing platforms are
solely responsible for choosing which `regional` packs to enforce and for any
platform-level moderation.

## `compliance.core` shape

```jsonc
{
  "is_nsfw": false,
  "contains_real_person_likeness": false,
  "uses_third_party_ip": false,
  "real_person_disclaimer_acknowledged": false,
  "third_party_ip_disclaimer_acknowledged": false,
  "creator_acknowledgments": [
    "no_self_harm_encouragement",
    "no_csam",
    "no_minor_sexualization"
  ]
}
```

## Hard-ban acknowledgments (schema-enforced)

The schema requires creators to attest acknowledgment of three platform-level
prohibitions:

- Self-harm or suicide encouragement
- Child sexual abuse material (CSAM)
- Sexualization of minors

A document missing any of these acknowledgments fails schema validation.
**Schema validation does not, and cannot, evaluate the persona's actual prompt
content for compliance.** Whether a persona's content respects them is
determined by implementation-side moderation.

## Conditional disclaimer acknowledgments

| Flag | Required acknowledgment field |
|---|---|
| `is_nsfw = true` | (no schema-required acknowledgment; implementation may add a regional pack) |
| `contains_real_person_likeness = true` | `real_person_disclaimer_acknowledged = true` |
| `uses_third_party_ip = true` | `third_party_ip_disclaimer_acknowledged = true` |

## `compliance.regional` shape

```jsonc
{
  "region": "<ISO-3166-1 alpha-2 OR custom string>",
  "pack_id": "<reverse-DNS, e.g. 'xyz.example.adult-content-jp'>",
  "pack_version": "1.0",
  "fields": { /* arbitrary JSON, schema determined by pack_id */ }
}
```

Spec does not enumerate or validate `fields`. `pack_id` is opaque to the spec.
