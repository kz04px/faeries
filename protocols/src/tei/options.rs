use std::fmt::{self, Display};

#[derive(Default)]
pub struct Check {
    pub name: String,
    pub value: bool,
}

pub struct Spin<T: PartialOrd + PartialEq + Display> {
    pub name: String,
    pub min: T,
    pub max: T,
    pub value: T,
}

pub struct Combo {
    pub name: String,
    pub value: String,
    pub options: Vec<String>,
}

pub struct Button {
    pub name: String,
}

pub struct Stringy {
    pub name: String,
}

impl fmt::Display for Check {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "option name {} type check default {}",
            self.name, self.value
        )?;
        Ok(())
    }
}

impl<T: PartialOrd + PartialEq + Display> fmt::Display for Spin<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "option name {} type spin default {} min {} max {}",
            self.name, self.value, self.min, self.max
        )?;
        Ok(())
    }
}

impl fmt::Display for Combo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "option name {} type combo default {}",
            self.name, self.value
        )?;
        for word in &self.options {
            write!(f, " var {}", word)?;
        }
        Ok(())
    }
}

impl fmt::Display for Button {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "option name {} type button", self.name)?;
        Ok(())
    }
}
