mod str_builder {
    use std::ops::Add;
    use std::fmt;

pub struct StrBuilder<'a> {
    data: Vec<&'a str>,
    total_len: usize,
}

impl<'a> StrBuilder<'a>{
   pub fn new() -> Self {
        StrBuilder{
            data:vec![],
            total_len: 0
        }
    }
    
    
    
    pub fn push(mut self, s: &'a str) -> Self {
        self.data.push(s);
        self.total_len += s.len();
        self
    }
}

impl<'a> Add<&'a str> for StrBuilder<'a>{
    type Output = StrBuilder<'a>;
    fn add(self, s:&'a str)->Self::Output{
        self.push(s)
    }
}

impl<'a> fmt::Display for StrBuilder<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut st = String::with_capacity(self.total_len);
        for s in self.data.iter() {
            st.push_str(s);
        }
        write!(f,"{}", st)
    }
}
}
