pub auto trait NotVoid {}
impl !NotVoid for () {}