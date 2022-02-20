//! # cmp_macro
//! This macro allows you to write a summary comparison, just like in Python.

/// less than
///
/// # Example
/// 
/// ```
/// assert!(lt!(1, 2, 3, 4, 5));
/// ```
#[macro_export]
macro_rules! lt {
    ($lhs:expr, $rhs:expr, $( $args:expr ),+) => {
        $lhs < $rhs && lt!($rhs, $( $args ),*)
    };
    ($lhs:expr, $rhs:expr) => {
        $lhs < $rhs
    }
}

/// less than with type cast
///
/// # Example
/// 
/// ```
/// assert!(ltty!(u32; 1.0, 2, 3.0, 4, 5.0));
/// ```
#[macro_export]
macro_rules! ltty {
    ($t:ty; $lhs:expr, $rhs:expr, $( $args:expr ),+) => {
        ($lhs as $t) < ($rhs as $t) && ltty!($t; $rhs, $( $args ),*)
    };
    ($t:ty; $lhs:expr, $rhs:expr) => {
        ($lhs as $t) < ($rhs as $t)
    }
}

/// less than or equal
///
/// # Example
/// 
/// ```
/// assert!(le!(1, 2, 3, 4, 4));
/// ```
#[macro_export]
macro_rules! le {
    ($lhs:expr, $rhs:expr, $( $args:expr ),+) => {
        $lhs <= $rhs && le!($rhs, $( $args ),*)
    };
    ($lhs:expr, $rhs:expr) => {
        $lhs <= $rhs
    }
}

/// less than or equal with type cast
///
/// # Example
/// 
/// ```
/// assert!(lety!(u32; 1.0, 2, 3.0, 4, 4));
/// ```
#[macro_export]
macro_rules! lety {
    ($t:ty; $lhs:expr, $rhs:expr, $( $args:expr ),+) => {
        ($lhs as $t) <= ($rhs as $t) && lety!($t; $rhs, $( $args ),*)
    };
    ($t:ty; $lhs:expr, $rhs:expr) => {
        ($lhs as $t) <= ($rhs as $t)
    }
}

/// greater than
///
/// # Example
/// 
/// ```
/// assert!(gt!(5, 4, 3, 2, 1));
/// ```
#[macro_export]
macro_rules! gt {
    ($lhs:expr, $rhs:expr, $( $args:expr ),+) => {
        $lhs > $rhs && gt!($rhs, $( $args ),*)
    };
    ($lhs:expr, $rhs:expr) => {
        $lhs > $rhs
    }
}

/// greater than with type cast
///
/// # Example
/// 
/// ```
/// assert!(gtty!(u32, 5.0, 4, 3.0, 2, 1.0));
/// ```
#[macro_export]
macro_rules! gtty {
    ($t:ty; $lhs:expr, $rhs:expr, $( $args:expr ),+) => {
        ($lhs as $t) > ($rhs as $t) && gtty!($t; $rhs, $( $args ),*)
    };
    ($t:ty; $lhs:expr, $rhs:expr) => {
        ($lhs as $t) > ($rhs as $t)
    }
}

/// greater than or equal
///
/// # Example
/// 
/// ```
/// assert!(ge!(5, 4, 3, 2, 2));
/// ```
#[macro_export]
macro_rules! ge {
    ($lhs:expr, $rhs:expr, $( $args:expr ),+) => {
        $lhs >= $rhs && ge!($rhs, $( $args ),*)
    };
    ($lhs:expr, $rhs:expr) => {
        $lhs >= $rhs
    }
}

/// greater than or equal with type cast
///
/// # Example
/// 
/// ```
/// assert!(gety!(u32, 5, 4.0, 3, 2, 2));
/// ```
#[macro_export]
macro_rules! gety {
    ($t:ty; $lhs:expr, $rhs:expr, $( $args:expr ),+) => {
        ($lhs as $t) >= ($rhs as $t) && gety!($t; $rhs, $( $args ),*)
    };
    ($t:ty; $lhs:expr, $rhs:expr) => {
        ($lhs as $t) >= ($rhs as $t)
    }
}

/// equal
///
/// # Example
/// 
/// ```
/// assert!(eq!(1, 1, 1));
/// ```
#[macro_export]
macro_rules! eq {
    ($lhs:expr, $rhs:expr, $( $args:expr ),+) => {
        $lhs == $rhs && eq!($rhs, $( $args ),*)
    };
    ($lhs:expr, $rhs:expr) => {
        $lhs == $rhs
    }
}

/// equal
///
/// # Example
/// 
/// ```
/// assert!(eq!(u32, 1u64, 1i32, 1u8));
/// ```
#[macro_export]
macro_rules! eqty {
    ($t:ty; $lhs:expr, $rhs:expr, $( $args:expr ),+) => {
        ($lhs as $t) == ($rhs as $t) && eqty!($t; $rhs, $( $args ),*)
    };
    ($t:ty; $lhs:expr, $rhs:expr) => {
        ($lhs as $t) == ($rhs as $t)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_lt() {
        assert!(lt!(1, 2));
        assert!(lt!(1, 2, 3, 4, 5));
        assert!(!lt!(2, 1));
        assert!(!lt!(1, 2, 3, 5, 4));

        assert!(!lt!(1, 1));
        assert!(!lt!(1, 2, 3, 4, 4));
    }

    #[test]
    fn test_ltty() {
        assert!(ltty!(u32; 1.0, 2.0));
        assert!(ltty!(u32; 1.0, 2.0, 3.0, 4.0, 5.0));
        assert!(!ltty!(u32; 2.0, 1.0));
        assert!(!ltty!(u32; 1.0, 2.0, 3.0, 5.0, 4.0));

        assert!(!ltty!(u32; 1.0, 1.0));
        assert!(!ltty!(u32; 1.0, 2.0, 3.0, 4.0, 4.0));
    }


    #[test]
    fn test_le() {
        assert!(le!(1, 2));
        assert!(le!(1, 2, 3, 4, 5));
        assert!(!le!(2, 1));
        assert!(!le!(1, 2, 3, 5, 4));

        assert!(le!(1, 1));
        assert!(le!(1, 2, 3, 4, 4));
    }

    #[test]
    fn test_lety() {
        assert!(lety!(u32; 1.0, 2.0));
        assert!(lety!(u32; 1.0, 2.0, 3.0, 4.0, 5.0));
        assert!(!lety!(u32; 2.0, 1.0));
        assert!(!lety!(u32; 1.0, 2.0, 3.0, 5.0, 4.0));

        assert!(lety!(u32; 1.0, 1.0));
        assert!(lety!(u32; 1.0, 2.0, 3.0, 4.0, 4.0));
    }


    #[test]
    fn test_gt() {
        assert!(gt!(2, 1));
        assert!(gt!(5, 4, 3, 2, 1));
        assert!(!gt!(1, 2));
        assert!(!gt!(5, 4, 3, 1, 2));

        assert!(!gt!(1, 1));
        assert!(!gt!(5, 4, 3, 2, 2));
    }

    #[test]
    fn test_gtty() {
        assert!(gtty!(u32; 2.0, 1.0));
        assert!(gtty!(u32; 5.0, 4.0, 3.0, 2.0, 1.0));
        assert!(!gtty!(u32; 1.0, 2.0));
        assert!(!gtty!(u32; 5.0, 4.0, 3.0, 1.0, 2.0));

        assert!(!gtty!(u32; 1.0, 1.0));
        assert!(!gtty!(u32; 5.0, 4.0, 3.0, 2.0, 2.0));
    }


    #[test]
    fn test_ge() {
        assert!(ge!(2, 1));
        assert!(ge!(5, 4, 3, 2, 1));
        assert!(!ge!(1, 2));
        assert!(!ge!(5, 4, 3, 1, 2));

        assert!(ge!(1, 1));
        assert!(ge!(5, 4, 3, 2, 2));
    }

    #[test]
    fn test_gety() {
        assert!(gety!(u32; 2.0, 1.0));
        assert!(gety!(u32; 5.0, 4.0, 3.0, 2.0, 1.0));
        assert!(!gety!(u32; 1.0, 2.0));
        assert!(!gety!(u32; 5.0, 4.0, 3.0, 1.0, 2.0));

        assert!(gety!(u32; 1.0, 1.0));
        assert!(gety!(u32; 5.0, 4.0, 3.0, 2.0, 2.0));
    }


    #[test]
    fn test_eq() {
        assert!(!eq!(2, 1));
        assert!(!eq!(5, 4, 3, 2, 1));
        assert!(eq!(1, 1));
        assert!(eq!(1, 1, 1, 1, 1));
    }

    #[test]
    fn test_eqty() {
        assert!(!eqty!(u32; 2.0, 1.0));
        assert!(!eqty!(u32; 5.0, 4.0, 3.0, 2.0, 1.0));
        assert!(eqty!(u32; 1.0, 1.0));
        assert!(eqty!(u32; 1.0, 1.0, 1.0, 1.0, 1.0));
    }

    #[test]
    fn readme() {
        assert!(le!(1, 3, 4));
        assert!(lety!(i32; -2.0, -1, 3, 4, 5.0));
    }
}