#[macro_use]
extern crate random_struct_layout;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_access() {
        #[layout_randomize]
        struct Data {
            a: i32,
            b: u32,
            c: u128,
            s: String,
        };

        let d = Data {
            a: 42,
            b: 1337,
            c: 0xdeadbeefcafebabe,
            s: "Hello world".to_string(),
        };

        assert_eq!(d.a, 42);
        assert_eq!(d.b, 1337);
        assert_eq!(d.c, 0xdeadbeefcafebabe);
        assert_eq!(d.s, "Hello world");
    }

    #[test]
    fn derives() {
        #[layout_randomize]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        struct Data {
            a: i32,
            b: u32,
            c: u128,
        };
    }

    // any other tests?
}
