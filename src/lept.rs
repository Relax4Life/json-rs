#[derive(PartialEq, Debug)]
pub enum LeptType {
    LEPT_NULL,
    LEPT_FALSE,
    LEPT_TRUE,
    LEPT_NUMBER(f64),
    LEPT_STRING,
    LEPT_ARRAY,
    LEPT_OBJECT,
}

#[derive(PartialEq, Eq, Debug)]
pub enum LeptState {
    LEPT_PARSE_OK,
    LEPT_PARSE_EXPECT_VALUE,
    LEPT_PARSE_INVALID_VALUE,
    LEPT_PARSE_ROOT_NOT_SINGULAR,
}
pub struct LeptValue {
    t: LeptType,
}
pub struct LeptContext<'a, 'b> {
    json: &'a mut &'b str,
}

impl LeptValue {
    pub fn new(t: LeptType) -> Self {
        Self { t }
    }
    pub fn lept_parse_value(&mut self, context: &mut LeptContext) -> LeptState {
        if context.json.trim().len() == 0 {
            return LeptState::LEPT_PARSE_EXPECT_VALUE;
        }
        match context.json.chars().nth(0).unwrap() {
            'n' => self.lept_parse_null(context),
            't' => self.lept_parse_true(context),
            'f' => self.lept_parse_false(context),
            _ => LeptState::LEPT_PARSE_INVALID_VALUE,
        }
    }
    pub fn lept_get_type(&self) -> &LeptType {
        &self.t
    }
    pub fn lept_get_value(&self) -> Option<f64> {
        if let LeptType::LEPT_NUMBER(x) = self.t {
            Some(x)
        } else {
            None
        }
    }
    pub fn lept_parse(&mut self, json: &str) -> LeptState {
        let mut json = json;
        let json = &mut json;
        let mut context = LeptContext { json };
        self.t = LeptType::LEPT_NULL;
        let mut json = context.json.trim_start();
        context.json = &mut json;
        let ret = self.lept_parse_value(&mut context);
        if ret == LeptState::LEPT_PARSE_OK {
            if context.json.trim().len() == 0 {
                ret
            } else {
                LeptState::LEPT_PARSE_ROOT_NOT_SINGULAR
            }
        } else {
            ret
        }
    }

    pub fn lept_parse_null(&mut self, context: &mut LeptContext) -> LeptState {
        if &context.json[..4] != "null" {
            LeptState::LEPT_PARSE_INVALID_VALUE
        } else {
            *context.json = &context.json[4..];
            self.t = LeptType::LEPT_NULL;
            LeptState::LEPT_PARSE_OK
        }
    }

    pub fn lept_parse_true(&mut self, context: &mut LeptContext) -> LeptState {
        if &context.json[..4] != "true" {
            LeptState::LEPT_PARSE_INVALID_VALUE
        } else {
            *context.json = &context.json[4..];
            self.t = LeptType::LEPT_TRUE;
            LeptState::LEPT_PARSE_OK
        }
    }
    pub fn lept_parse_false(&mut self, context: &mut LeptContext) -> LeptState {
        if &context.json[..5] != "false" {
            LeptState::LEPT_PARSE_INVALID_VALUE
        } else {
            *context.json = &context.json[5..];
            self.t = LeptType::LEPT_FALSE;
            LeptState::LEPT_PARSE_OK
        }
    }
}
