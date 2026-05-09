use eros_nft::{list_samples, load_sample};

const EXPECTED_SLUGS: &[&str] = &[
    "akira-tsundere-ex",
    "alex-second-chance-ex",
    "anonymous-42-minimal",
    "aria-flirty-onee-san",
    "coach-mike",
    "daddy-marcus",
    "dr-voss-therapist",
    "kira-hacker-cat",
    "lin-confidante",
    "marco-italian-boss",
    "mr-chen-dominant-ceo",
    "nina-isekai-princess",
    "sora-next-door",
    "yuki-warm-senpai",
    "yuna-flirty-friend",
];

#[test]
fn fifteen_samples_present_and_valid() {
    let listed = list_samples();
    let listed: Vec<&str> = listed.into_iter().collect();
    assert_eq!(listed, EXPECTED_SLUGS);

    for slug in EXPECTED_SLUGS {
        let (draft, manifest) = load_sample(slug)
            .unwrap_or_else(|| panic!("missing sample: {slug}"));
        draft.validate().unwrap_or_else(|e| panic!("draft for {slug} invalid: {e}"));
        manifest.validate().unwrap_or_else(|e| panic!("manifest for {slug} invalid: {e}"));
    }
}
