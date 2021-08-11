pub mod convert;

#[cfg(test)]
mod tests {
    use crate::convert::*;

    #[test]
    fn test_language_to_latin() {
        assert_eq!(to_latin("ˈlæŋɡwɪd͡ʒ"), "langwidj");
        assert_eq!(to_latin("ˈlẽŋ.ɡwa"), "lengwa");
        assert_eq!(to_latin("bʱɑː.ʂɑː"), "baca");
        assert_eq!(to_latin("lĩ.ɡwɐ"), "ligwa");
        assert_eq!(to_latin("(j)ɪˈzɨk"), "yizik");
        assert_eq!(to_latin("lɑ̃ɡ"), "lag");
        assert_eq!(to_latin("ɡẽ̞ŋɡo̞"), "gengo");
        assert_eq!(to_latin("baˈhasa"), "bahasa");
        assert_eq!(to_latin("zʊ.bɑːn"), "zuban");
        assert_eq!(to_latin("ˈʃpʁaːxə"), "cpraxə");
    }

    #[test]
    fn test_cat_to_latin() {
        assert_eq!(to_latin("māo"), "mao");
        assert_eq!(to_latin("kæt"), "kat");
        assert_eq!(to_latin("ˈɡa.t̪o"), "gato");
        assert_eq!(to_latin("bɪl̪.l̪iː"), "billi");
        assert_eq!(to_latin("biṛal"), "biral");
        assert_eq!(to_latin("ˈɡä.t̪ʊ"), "gatu");
        assert_eq!(to_latin("ˈkoʂkə"), "kockə");
        assert_eq!(to_latin("ʃa"), "ca");
        assert_eq!(to_latin("qiṭṭ"), "kitt");
        assert_eq!(to_latin("ne̞ko̞"), "neko");
        assert_eq!(to_latin("kut͡ʃɪŋ"), "kutcin");
        assert_eq!(to_latin("billī"), "billi");
        assert_eq!(to_latin("ˈkatsə"), "katsə");
    }

    #[test]
    fn test_cat_to_ipa() {
        assert_eq!(to_latin(&to_ipa("貓", "zh").unwrap()), "mau");
        assert_eq!(to_latin(&to_ipa("cat", "en").unwrap()), "kat");
        assert_eq!(to_latin(&to_ipa("gato", "es").unwrap()), "gato");
        assert_eq!(to_latin(&to_ipa("बिल्ली", "hi").unwrap()), "billi");
        // assert_eq!(to_latin(&to_ipa("বিড়াল", "bn").unwrap()), "biral");
        assert_eq!(to_latin(&to_ipa("gato", "pt").unwrap()), "gato");
        assert_eq!(to_latin(&to_ipa("кошка", "ru").unwrap()), "kockə");
        // assert_eq!(to_latin(&to_ipa("chat", "fr").unwrap()), "ca");
        // assert_eq!(to_latin(&to_ipa("قط", "ar").unwrap()), "kitt");
        assert_eq!(to_latin(&to_ipa("ねこ", "ja").unwrap()), "neko");
        // assert_eq!(to_latin(&to_ipa("kucing", "id").unwrap()), "kutcin");
        // assert_eq!(to_latin(&to_ipa("بلّی", "ur").unwrap()), "billi");
        assert_eq!(to_latin(&to_ipa("katze", "de").unwrap()), "katsə");
    }

    #[test]
    fn test_ipa() {
        let lang = "ur";
        let word = "بلّی";
        println!("word: {}, IPA: {}", word, to_ipa(word, lang).unwrap());
    }

    /*
    use crate::main::*;
    #[test]
    fn test_w212() {
        assert!("pa".to_string().is_match_w212());
        assert!("ap".to_string().is_match_w212());
        assert!(!"aa".to_string().is_match_w212());
        assert!(!"pp".to_string().is_match_w212());
        assert!(!"baa".to_string().is_match_w212());
        assert!("a".to_string().is_match_w212());
        assert!("p".to_string().is_match_w212());
        assert!("".to_string().is_match_w212());
    }

    #[test]
    fn test_w204() {
        assert!("kt".to_string().is_match_w204());
        assert!("ap".to_string().is_match_w204());
        assert!("pa".to_string().is_match_w204());
        assert!(!"bp".to_string().is_match_w204());
        assert!(!"pb".to_string().is_match_w204());
        assert!("k".to_string().is_match_w204());
        assert!("t".to_string().is_match_w204());
        assert!("".to_string().is_match_w204());
    }

    #[test]
    fn test_w205() {
        assert!("xa".to_string().is_match_w205());
        assert!("ax".to_string().is_match_w205());
        assert!(!"xp".to_string().is_match_w205());
        assert!(!"hp".to_string().is_match_w205());
        assert!(!"yp".to_string().is_match_w205());
        assert!(!"wp".to_string().is_match_w205());
        assert!(!"px".to_string().is_match_w205());
        assert!(!"ph".to_string().is_match_w205());
        assert!(!"py".to_string().is_match_w205());
        assert!(!"pw".to_string().is_match_w205());
        assert!("x".to_string().is_match_w205());
        assert!("p".to_string().is_match_w205());
        assert!("".to_string().is_match_w205());
    }

    #[test]
    fn test_w206() {
        assert!("tsa".to_string().is_match_w206());
        assert!("tca".to_string().is_match_w206());
        assert!("dza".to_string().is_match_w206());
        assert!("dja".to_string().is_match_w206());
        assert!("ats".to_string().is_match_w206());
        assert!("atc".to_string().is_match_w206());
        assert!("adz".to_string().is_match_w206());
        assert!("adj".to_string().is_match_w206());
        assert!("apa".to_string().is_match_w206());
        assert!("apk".to_string().is_match_w206());
        assert!(!"pkt".to_string().is_match_w206());
        assert!("pk".to_string().is_match_w206());
        assert!("p".to_string().is_match_w206());
        assert!("".to_string().is_match_w206());
    }

    #[test]
    fn test_w207() {
        assert!("aa".to_string().is_match_w207());
        assert!("pp".to_string().is_match_w207());
        assert!(!"cj".to_string().is_match_w207());
        assert!("a".to_string().is_match_w207());
        assert!("c".to_string().is_match_w207());
        assert!("".to_string().is_match_w207());
    }

    #[test]
    fn test_w208() {
        assert!("t".to_string().is_match_w208());
        assert!("ta".to_string().is_match_w208());
        assert!(!"a".to_string().is_match_w208());
        assert!("".to_string().is_match_w208());
    }

    #[test]
    fn test_w209() {
        assert!("t".to_string().is_match_w209());
        assert!("at".to_string().is_match_w209());
        assert!(!"a".to_string().is_match_w209());
        assert!("".to_string().is_match_w209());
    }

    #[test]
    fn test_w210() {
        assert!("ts".to_string().is_match_w210());
        assert!("tc".to_string().is_match_w210());
        assert!("dz".to_string().is_match_w210());
        assert!("dj".to_string().is_match_w210());
        assert!(!"kt".to_string().is_match_w210());
        assert!("aa".to_string().is_match_w210());
        assert!("".to_string().is_match_w210());
    }

    #[test]
    fn test_w211() {
        assert!("pppp".to_string().is_match_w211());
        assert!(!"kaaa".to_string().is_match_w211());
        assert!("".to_string().is_match_w211());
    }
     */
}
