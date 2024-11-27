
pub trait VecExt<T> {
    fn to_sql(&self) -> String;
}
impl<T: ToSql> VecExt<T> for Vec<T> {
    fn to_sql(&self) -> String {
        let mut s = String::new();
        for item in self {
            s.push_str(&item.to_sql());
            s.push_str(",");
        }
        s.pop();
        s.insert_str(0, "(");
        s.push_str(")");
        s
    }
}

pub trait ToSql: ToString {
    fn to_sql(&self) -> String;
}

impl ToSql for &str {
    fn to_sql(&self) -> String {
        format!("'{}'", self)
    }
}
impl ToSql for String {
    fn to_sql(&self) -> String {
        format!("'{}'", self)
    }
}
impl ToSql for isize {
    fn to_sql(&self) -> String {
        self.to_string()
    }
}
impl ToSql for i128 {
    fn to_sql(&self) -> String {
        self.to_string()
    }
}
impl ToSql for i64 {
    fn to_sql(&self) -> String {
        self.to_string()
    }
}
impl ToSql for i32 {
    fn to_sql(&self) -> String {
        self.to_string()
    }
}
impl ToSql for i16 {
    fn to_sql(&self) -> String {
        self.to_string()
    }
}
impl ToSql for i8 {
    fn to_sql(&self) -> String {
        self.to_string()
    }
}
impl ToSql for usize {
    fn to_sql(&self) -> String {
        self.to_string()
    }
}
impl ToSql for u128 {
    fn to_sql(&self) -> String {
        self.to_string()
    }
}
impl ToSql for u64 {
    fn to_sql(&self) -> String {
        self.to_string()
    }
}
impl ToSql for u32 {
    fn to_sql(&self) -> String {
        self.to_string()
    }
}
impl ToSql for u16 {
    fn to_sql(&self) -> String {
        self.to_string()
    }
}
impl ToSql for u8 {
    fn to_sql(&self) -> String {
        self.to_string()
    }
}
impl ToSql for f64 {
    fn to_sql(&self) -> String {
        self.to_string()
    }
}
impl ToSql for f32 {
    fn to_sql(&self) -> String {
        self.to_string()
    }
}
impl ToSql for bool {
    fn to_sql(&self) -> String {
        self.to_string()
    }
}
