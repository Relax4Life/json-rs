pub mod lept;
#[cfg(test)]
mod tests {
    use crate::lept::*;

    #[test]
    fn test_parse_null() {
        let mut v = LeptValue::new(LeptType::LEPT_TRUE);

        assert_eq!(LeptState::LEPT_PARSE_OK, v.lept_parse("null"));
        assert_eq!(LeptType::LEPT_NULL, *v.lept_get_type());
    }
    #[test]
    fn test_parse_true() {
        let mut v = LeptValue::new(LeptType::LEPT_NULL);
        assert_eq!(LeptState::LEPT_PARSE_OK, v.lept_parse("true"));
        assert_eq!(LeptType::LEPT_TRUE, *v.lept_get_type());
    }
    #[test]
    fn test_parse_false() {
        let mut v = LeptValue::new(LeptType::LEPT_NULL);
        assert_eq!(LeptState::LEPT_PARSE_OK, v.lept_parse("false"));
        assert_eq!(LeptType::LEPT_FALSE, *v.lept_get_type());
    }
    #[test]
    fn test_parse_error() {
        let mut v = LeptValue::new(LeptType::LEPT_NULL);
        assert_eq!(LeptState::LEPT_PARSE_INVALID_VALUE, v.lept_parse("xlse"));
        assert_eq!(LeptState::LEPT_PARSE_OK, v.lept_parse("   false   "));
        assert_eq!(
            LeptState::LEPT_PARSE_ROOT_NOT_SINGULAR,
            v.lept_parse("   false  x ")
        );
    }

    fn TEST_NUMBER(expect: f64, json: &str) {
        use std::matches;
        let mut v = LeptValue::new(LeptType::LEPT_NULL);
        assert_eq!(LeptState::LEPT_PARSE_OK, v.lept_parse(json));
        assert_eq!(v.lept_get_value(), Some(expect));
    }
    fn TEST_ERROR(expect: LeptState, json: &str) {
        let mut v = LeptValue::new(LeptType::LEPT_NULL);
        assert_eq!(expect, v.lept_parse(json));
    }
    #[test]
    fn test_parse_number() {
        TEST_NUMBER(0.0, "0");
        TEST_NUMBER(0.0, "-0");
        TEST_NUMBER(0.0, "-0.0");
        TEST_NUMBER(1.0, "1");
        TEST_NUMBER(-1.0, "-1");
        TEST_NUMBER(1.5, "1.5");
        TEST_NUMBER(-1.5, "-1.5");
        TEST_NUMBER(3.1416, "3.1416");
        TEST_NUMBER(1E10, "1E10");
        TEST_NUMBER(1e10, "1e10");
        TEST_NUMBER(1E+10, "1E+10");
        TEST_NUMBER(1E-10, "1E-10");
        TEST_NUMBER(-1E10, "-1E10");
        TEST_NUMBER(-1e10, "-1e10");
        TEST_NUMBER(-1E+10, "-1E+10");
        TEST_NUMBER(-1E-10, "-1E-10");
        TEST_NUMBER(1.234E+10, "1.234E+10");
        TEST_NUMBER(1.234E-10, "1.234E-10");
        TEST_NUMBER(0.0, "1e-10000");
        TEST_ERROR(LEPT_PARSE_INVALID_VALUE, "+0");
        TEST_ERROR(LEPT_PARSE_INVALID_VALUE, "+1");
        TEST_ERROR(LEPT_PARSE_INVALID_VALUE, ".123"); /* at least one digit before '.' */
        TEST_ERROR(LEPT_PARSE_INVALID_VALUE, "1."); /* at least one digit after '.' */
        TEST_ERROR(LEPT_PARSE_INVALID_VALUE, "INF");
        TEST_ERROR(LEPT_PARSE_INVALID_VALUE, "inf");
        TEST_ERROR(LEPT_PARSE_INVALID_VALUE, "NAN");
        TEST_ERROR(LEPT_PARSE_INVALID_VALUE, "nan");
    }
}
