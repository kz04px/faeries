pub mod options {
    use std::fmt;

    #[derive(Default)]
    pub struct Check {
        pub name: String,
        pub value: bool,
    }

    pub struct Spin {
        pub name: String,
        pub min: i32,
        pub max: i32,
        pub value: i32,
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
        pub name: i32,
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

    impl fmt::Display for Spin {
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
}
