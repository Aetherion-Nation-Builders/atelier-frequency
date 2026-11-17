//! Atelier Frequency's own UI strings in Brazilian Portuguese (the brand layer, `docs/brand.md`).
//!
//! The catalog in `mod.rs` is upstream's file, keyed by SoundCraft's English strings; our renamed
//! strings live here so upstream catalog changes merge without conflicts. `localization.rs` looks
//! a string up here first, then in upstream's catalog. Keys must not repeat upstream's.

/// Returns the Brazilian Portuguese translation of one of our brand strings.
pub(super) fn translate(source: &str) -> Option<&'static str> {
    Some(match source {
        "Atelier Frequency" => "Atelier Frequency",
        "About Atelier Frequency" => "Sobre o Atelier Frequency",
        "Atelier Frequency Preferences" => "Preferências do Atelier Frequency",
        "Atelier Frequency Session" => "Sessão do Atelier Frequency",
        "Loading Atelier Frequency…" => "Carregando o Atelier Frequency…",
        "Atelier Frequency failed to start: {}" => "Falha ao iniciar o Atelier Frequency: {}",
        "The Nation's digital audio workstation. Built on SoundCraft by the ArtCraft team." => {
            "A estação de trabalho de áudio digital da Nação. Construída sobre o SoundCraft, da equipe ArtCraft."
        }
        "Join the Nation" => "Junte-se à Nação",
        "Use Reconnect audio device in Setup › Playback Engine, or restart Atelier Frequency." => {
            "Use Reconectar dispositivo de áudio em Configurar › Mecanismo de reprodução, ou reinicie o Atelier Frequency."
        }
        "Atelier Frequency hit an internal error (see the log). Your changes were autosaved: save them, then restart Atelier Frequency." => {
            "O Atelier Frequency encontrou um erro interno (consulte o registro). Suas alterações foram salvas automaticamente: salve-as e reinicie o Atelier Frequency."
        }
        "Atelier Frequency hit an internal error (see the log). Restart Atelier Frequency to be safe." => {
            "O Atelier Frequency encontrou um erro interno (consulte o registro). Reinicie o Atelier Frequency para garantir."
        }
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    /// Our keys, as written above.
    const KEYS: [&str; 11] = [
        "Atelier Frequency",
        "About Atelier Frequency",
        "Atelier Frequency Preferences",
        "Atelier Frequency Session",
        "Loading Atelier Frequency…",
        "Atelier Frequency failed to start: {}",
        "The Nation's digital audio workstation. Built on SoundCraft by the ArtCraft team.",
        "Join the Nation",
        "Use Reconnect audio device in Setup › Playback Engine, or restart Atelier Frequency.",
        "Atelier Frequency hit an internal error (see the log). Your changes were autosaved: save them, then restart Atelier Frequency.",
        "Atelier Frequency hit an internal error (see the log). Restart Atelier Frequency to be safe.",
    ];

    #[test]
    fn brand_strings_are_translated_and_do_not_shadow_upstream_rows() {
        for key in KEYS {
            let translation = super::translate(key).unwrap();
            assert_eq!(key.matches("{}").count(), translation.matches("{}").count(), "placeholders differ for {key:?}");
            assert!(!translation.contains("SoundCraft") || key.contains("SoundCraft"), "upstream name in {key:?}");
            assert!(super::super::pt_br_upstream::translate(key).is_none(), "{key:?} repeats an upstream catalog key");
        }
    }
}
