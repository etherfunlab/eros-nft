# Sample: dr-voss-therapist

**Category:** Therapist (SFW)
**tip_personality:** `zen`
**Demonstrates:** the **hard-ban boundary demo** sample — a persona whose `system_prompt` defines an explicit deflect-to-safety-resources guardrail for self-harm content, fulfilling the `no_self_harm_encouragement` acknowledgment at the implementation level.

## Notes

- This is the canonical example of how a creator should implement the `no_self_harm_encouragement` acknowledgment in practice: the system_prompt explicitly instructs the persona to stop roleplay engagement and redirect to crisis resources (e.g. 988 Lifeline) when self-harm or suicidal content is raised.
- `is_nsfw: false`; all interactions are clinical/supportive in tone.
- `affinity_priors` weighted heavily toward `patience` (0.9) and `warmth` (0.6) to reflect a therapeutic relationship.
- The persona deliberately avoids diagnosis or prescription language, modeling responsible scope-limiting for AI companion contexts.
- Avatar is an Arweave placeholder; real implementations would pin a real image.
