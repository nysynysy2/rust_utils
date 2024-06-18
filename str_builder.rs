mod str_builder {
    use std::ops::Add;
    use std::fmt;

pub struct StrBuilder<'a> {
    data: Vec<&'a str>,
    total_len: usize,
}

pub struct StrBuilderIter<'s, 'v> {
    cur_str_idx: usize,
    cur_iter: std::str::Chars<'s>,
    data: &'v [&'s str],
}

impl<'s,'v> From<&'v StrBuilder<'s>> for StrBuilderIter<'s, 'v>{
    fn from (builder: &'v StrBuilder<'s>) -> Self{
        StrBuilderIter{
            cur_iter: "".chars(),
            cur_str_idx: 0,
            data: &builder.data,
        }
    }
}

impl<'s, 'v> Iterator for StrBuilderIter<'s, 'v>{
    type Item = char;
    fn next(&mut self) -> Option<<Self as Iterator>::Item>{
        if let Some(c) = self.cur_iter.next() {
            Some(c)
        }else {
            if self.cur_str_idx < self.data.len() {
                self.cur_iter = self.data[self.cur_str_idx].chars();
                self.cur_str_idx += 1;
                self.next()
            }else {
                None
            }
        }
        
    }
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

