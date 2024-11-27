#![allow(dead_code)]

#[derive(Debug, Default)]
struct Cond {
    and: bool,
    key: String,
    op: Op,
    value: String,
}
impl Cond {
    fn new(and: bool, key: &str, op: Op, value: String) -> Self {
        Self {
            and,
            key: key.to_string(),
            op,
            value,
        }
    }
}

#[derive(Debug, Default)]
pub enum Op {
    #[default]
    Eq,
    NotEq,
    Gt,
    Lt,
    GtEq,
    LtEq,
    In,
    NotIn,
    Like,
    NotLike,
    // IsNull,
    // IsNotNull,
}
impl ToString for Op {
    fn to_string(&self) -> String {
        match self {
            Op::Eq => "=".to_string(),
            Op::NotEq => "!=".to_string(),
            Op::Gt => ">".to_string(),
            Op::Lt => "<".to_string(),
            Op::GtEq => ">=".to_string(),
            Op::LtEq => "<=".to_string(),
            Op::In => "in".to_string(),
            Op::NotIn => "not in".to_string(),
            Op::Like => "like".to_string(),
            Op::NotLike => "not like".to_string(),
            // Op::IsNull => "is null".to_string(),
            // Op::IsNotNull => "is not null".to_string(),
        }
    }
}

pub enum Object {
    Int(isize),
    Float(f64),
    Bool(bool),
    Null,
    String(String),
    Array(String),
}
impl Object {
    fn parse(&self) -> String {
        match self {
            Object::Int(v) => v.to_string(),
            Object::Float(v) => v.to_string(),
            Object::Bool(v) => v.to_string(),
            Object::Null => "null".to_string(),
            Object::String(v) => format!("'{}'", v),
            Object::Array(v) => v.to_string(),
        }
    }
}

#[derive(Debug, Default)]
pub struct Matcher {
    table: String,
    fields: Vec<String>,
    count_field: String,
    cond_list: Vec<Cond>,
    limit: Option<isize>,
    offset: Option<isize>,
    order_by: Option<String>,
    order_desc: bool,
    group_by: Option<String>,
}
impl Matcher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn table(&mut self, table: &str) -> &mut Self {
        self.table = table.to_string();
        self
    }

    pub fn fields(&mut self, fields: &[&str]) -> &mut Self {
        self.fields = fields.iter().map(|f| f.to_string()).collect();
        self
    }

    pub fn count_field(&mut self, count_field: &str) -> &mut Self {
        self.count_field = count_field.to_string();
        self
    }

    pub fn group_by(&mut self, group_by: &str) -> &mut Self {
        self.group_by = Some(group_by.to_string());
        self
    }

    pub fn limit(&mut self, limit: isize) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(&mut self, offset: isize) -> &mut Self {
        self.offset = Some(offset);
        self
    }

    pub fn order_by(&mut self, order_by: &str, order_desc: bool) -> &mut Self {
        self.order_by = Some(order_by.to_string());
        self.order_desc = order_desc;
        self
    }

    pub fn and(&mut self, key: &str, op: Op, value: Object) -> &mut Self {
        self.cond_list.push(Cond::new(true, key, op, value.parse()));
        self
    }

    pub fn or(&mut self, key: &str, op: Op, value: Object) -> &mut Self {
        self.cond_list
            .push(Cond::new(false, key, op, value.parse()));
        self
    }

    pub fn build_where(&self, with_where: bool) -> String {
        let mut s = String::new();
        for cond in &self.cond_list {
            if s.len() > 0 {
                s.push_str(if cond.and { " and " } else { " or " });
            }
            let mut op = cond.op.to_string();
            if cond.value == "null" {
                op = match cond.op {
                    Op::Eq => "is".to_string(),
                    Op::NotEq => "is not".to_string(),
                    _ => cond.op.to_string(),
                };
            }
            s.push_str(&format!("{} {} {}", cond.key, op, cond.value));
        }
        if with_where && self.cond_list.len() > 0 {
            s.insert_str(0, " where ");
        }
        s
    }

    pub fn build(&self) -> String {
        if self.table.is_empty() {
            panic!("table is empty");
        }
        let mut s = String::from("SELECT ");
        if self.fields.is_empty() {
            s.push_str("*");
        } else {
            s.push_str(&self.fields.join(", "));
        }
        s.push_str(" FROM ");
        s.push_str(&self.table);
        s.push_str(&self.build_where(true));

        // group by
        if let Some(group_by) = &self.group_by {
            s.push_str(&format!(" group by {}", group_by));
        }

        // order by
        if let Some(order_by) = &self.order_by {
            s.push_str(&format!(
                " order by {} {}",
                order_by,
                if self.order_desc { "desc" } else { "" }
            ));
        }

        // limit and start
        match (self.limit, self.offset) {
            (Some(limit), Some(offset)) => {
                if limit > 0 && offset >= 0 {
                    s.push_str(&format!(" limit {} offset {}", limit, offset));
                }
            }
            _ => {}
        }
        s
    }

    pub fn build_count(&self) -> String {
        if self.table.is_empty() {
            panic!("table is empty");
        }
        let mut s = String::from("SELECT COUNT(*)");
        if !self.count_field.is_empty() {
            s.push_str(&format!(" as {}", self.count_field));
        }
        s.push_str(" FROM ");
        s.push_str(&self.table);
        s.push_str(&self.build_where(true));
        s
    }
}
