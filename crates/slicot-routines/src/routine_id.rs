//! Canonical routine identifiers for the initial pure-Rust SLICOT port.

/// Phase-one SLICOT routines selected from the `python-control` surface area.
pub const PHASE_ONE_ROUTINES: [RoutineId; 23] = [
    RoutineId::Ab08Nd,
    RoutineId::Ab09Ad,
    RoutineId::Ab09Md,
    RoutineId::Ab09Nd,
    RoutineId::Ab13Bd,
    RoutineId::Ab13Dd,
    RoutineId::Ab13Md,
    RoutineId::Mb03Rd,
    RoutineId::Sb01Bd,
    RoutineId::Sb02Md,
    RoutineId::Sb02Mt,
    RoutineId::Sb03Md,
    RoutineId::Sb03Od,
    RoutineId::Sb04Md,
    RoutineId::Sb04Qd,
    RoutineId::Sb10Ad,
    RoutineId::Sb10Hd,
    RoutineId::Sg02Ad,
    RoutineId::Sg03Ad,
    RoutineId::Tb01Pd,
    RoutineId::Tb04Ad,
    RoutineId::Tb05Ad,
    RoutineId::Td04Ad,
];

/// Known routine identifiers for the first implementation wave.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum RoutineId {
    Ab08Nd,
    Ab09Ad,
    Ab09Md,
    Ab09Nd,
    Ab13Bd,
    Ab13Dd,
    Ab13Md,
    Mb03Rd,
    Sb01Bd,
    Sb02Md,
    Sb02Mt,
    Sb03Md,
    Sb03Od,
    Sb04Md,
    Sb04Qd,
    Sb10Ad,
    Sb10Hd,
    Sg02Ad,
    Sg03Ad,
    Tb01Pd,
    Tb04Ad,
    Tb05Ad,
    Td04Ad,
}

impl RoutineId {
    /// Returns the uppercase SLICOT stem used by the upstream sources.
    #[must_use]
    pub const fn stem(self) -> &'static str {
        match self {
            Self::Ab08Nd => "AB08ND",
            Self::Ab09Ad => "AB09AD",
            Self::Ab09Md => "AB09MD",
            Self::Ab09Nd => "AB09ND",
            Self::Ab13Bd => "AB13BD",
            Self::Ab13Dd => "AB13DD",
            Self::Ab13Md => "AB13MD",
            Self::Mb03Rd => "MB03RD",
            Self::Sb01Bd => "SB01BD",
            Self::Sb02Md => "SB02MD",
            Self::Sb02Mt => "SB02MT",
            Self::Sb03Md => "SB03MD",
            Self::Sb03Od => "SB03OD",
            Self::Sb04Md => "SB04MD",
            Self::Sb04Qd => "SB04QD",
            Self::Sb10Ad => "SB10AD",
            Self::Sb10Hd => "SB10HD",
            Self::Sg02Ad => "SG02AD",
            Self::Sg03Ad => "SG03AD",
            Self::Tb01Pd => "TB01PD",
            Self::Tb04Ad => "TB04AD",
            Self::Tb05Ad => "TB05AD",
            Self::Td04Ad => "TD04AD",
        }
    }

    /// Resolves an uppercase SLICOT routine stem into a known phase-one identifier.
    ///
    /// # Examples
    ///
    /// ```
    /// use slicot_routines::RoutineId;
    ///
    /// assert_eq!(RoutineId::from_stem("SB03MD"), Some(RoutineId::Sb03Md));
    /// assert_eq!(RoutineId::from_stem("UNKNOWN"), None);
    /// ```
    #[must_use]
    pub fn from_stem(stem: &str) -> Option<Self> {
        match stem {
            "AB08ND" => Some(Self::Ab08Nd),
            "AB09AD" => Some(Self::Ab09Ad),
            "AB09MD" => Some(Self::Ab09Md),
            "AB09ND" => Some(Self::Ab09Nd),
            "AB13BD" => Some(Self::Ab13Bd),
            "AB13DD" => Some(Self::Ab13Dd),
            "AB13MD" => Some(Self::Ab13Md),
            "MB03RD" => Some(Self::Mb03Rd),
            "SB01BD" => Some(Self::Sb01Bd),
            "SB02MD" => Some(Self::Sb02Md),
            "SB02MT" => Some(Self::Sb02Mt),
            "SB03MD" => Some(Self::Sb03Md),
            "SB03OD" => Some(Self::Sb03Od),
            "SB04MD" => Some(Self::Sb04Md),
            "SB04QD" => Some(Self::Sb04Qd),
            "SB10AD" => Some(Self::Sb10Ad),
            "SB10HD" => Some(Self::Sb10Hd),
            "SG02AD" => Some(Self::Sg02Ad),
            "SG03AD" => Some(Self::Sg03Ad),
            "TB01PD" => Some(Self::Tb01Pd),
            "TB04AD" => Some(Self::Tb04Ad),
            "TB05AD" => Some(Self::Tb05Ad),
            "TD04AD" => Some(Self::Td04Ad),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{RoutineId, PHASE_ONE_ROUTINES};

    #[test]
    fn stems_round_trip() {
        for routine in PHASE_ONE_ROUTINES {
            assert_eq!(RoutineId::from_stem(routine.stem()), Some(routine));
        }
    }

    #[test]
    fn unknown_stems_are_rejected() {
        assert_eq!(RoutineId::from_stem("SB99ZZ"), None);
    }
}
