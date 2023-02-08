#[cfg(test)]
mod tests {
    use dshot_encoder::*;

    #[test]
    fn test_throttle_bounds_check() {
        assert!( throttle( THROTTLE_MIN - 1 , false).is_err() );
        assert!( throttle( THROTTLE_MAX + 1 , false).is_err() );

        assert!( throttle( THROTTLE_MIN , false).is_ok() );
        assert!( throttle( THROTTLE_MAX , false).is_ok() );

        assert_eq!( throttle( THROTTLE_MIN , false).unwrap() , 0x0606);
        assert_eq!( throttle( THROTTLE_MAX , false).unwrap() , 0xFFEE );
    }

    #[test]
    fn test_throttle_clamp() {
        // Without telemetry flag
        assert_eq!( throttle_clamp( THROTTLE_MIN, false), 0x0606 );
        assert_eq!( throttle_clamp( THROTTLE_MAX, false), 0xFFEE );
        assert_eq!( throttle_clamp( THROTTLE_MIN - 1 , false), 0x0606  );
        assert_eq!( throttle_clamp( THROTTLE_MAX + 1 , false), 0xFFEE );

        // With telemetry flag
        assert_eq!( throttle_clamp( THROTTLE_MIN, true), 0x0617 );
        assert_eq!( throttle_clamp( THROTTLE_MAX, true), 0xFFFF );
        assert_eq!( throttle_clamp( THROTTLE_MIN - 1 , true), 0x0617 );
        assert_eq!( throttle_clamp( THROTTLE_MAX + 1 , true), 0xFFFF );
    }
}