pub mod main;

#[cfg(test)]
mod tests {
    use crate::main::*;

    #[test]
    fn test_w202_and_w203() {
        assert!("pa".to_string().is_match_w202_and_w203());
        assert!("ap".to_string().is_match_w202_and_w203());
        assert!(!"aa".to_string().is_match_w202_and_w203());
        assert!(!"pp".to_string().is_match_w202_and_w203());
        assert!(!"baa".to_string().is_match_w202_and_w203());
        assert!("a".to_string().is_match_w202_and_w203());
        assert!("p".to_string().is_match_w202_and_w203());
        assert!("".to_string().is_match_w202_and_w203());
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
}
