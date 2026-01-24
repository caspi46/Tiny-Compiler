struct relOp(String);

impl relOp {
    fn isRelOp(str: String) {
        if str == "==" || str == "!=" || str == "<" || str == ">" || str == "<=" || str == ">=" {
            return true;
        }
        false
    }
}
