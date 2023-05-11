#[macro_use]
extern crate random_struct_layout;

#[cfg(test)]
mod tests {
    #[allow(unused)]
    use super::*;

    #[test]
    fn randomize_test() {
        use offset;

        struct DataStruct {
            a: i32,
            b: i32,
            c: i32,
            d: i32,
            e: i32,
            f: i32,
            g: i32,
        }

        assert!(
            (offset::offset_of!(DataStruct::a) == 0
                && offset::offset_of!(DataStruct::b) == 4
                && offset::offset_of!(DataStruct::c) == 8
                && offset::offset_of!(DataStruct::d) == 12
                && offset::offset_of!(DataStruct::e) == 16
                && offset::offset_of!(DataStruct::f) == 20
                && offset::offset_of!(DataStruct::g) == 24)
        );

        #[layout_randomize]
        struct RandomizedDataStruct {
            a: i32,
            b: i32,
            c: i32,
            d: i32,
            e: i32,
            f: i32,
            g: i32,
        }

        assert!(
            !(offset::offset_of!(RandomizedDataStruct::a) == 0
                && offset::offset_of!(RandomizedDataStruct::b) == 4
                && offset::offset_of!(RandomizedDataStruct::c) == 8
                && offset::offset_of!(RandomizedDataStruct::d) == 12
                && offset::offset_of!(RandomizedDataStruct::e) == 16
                && offset::offset_of!(RandomizedDataStruct::f) == 20
                && offset::offset_of!(RandomizedDataStruct::g) == 24)
        );
    }

    #[test]
    fn normal_access() {
        #[layout_randomize]
        struct Data {
            a: i32,
            b: u32,
            c: u128,
            s: String,
        }

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
        }
    }

    #[test]
    fn debug_output() {
        #[layout_randomize(Debug)]
        struct Randomized {
            a: i32,
            b: i32,
            c: i32,
            d: i32,
            e: i32,
            f: i32,
            g: i32,
        }

        #[derive(Debug)]
        #[allow(unused)]
        struct NotRandomized {
            a: i32,
            b: i32,
            c: i32,
            d: i32,
            e: i32,
            f: i32,
            g: i32,
        }

        let randomized = Randomized {
            a: 1,
            b: 2,
            c: 3,
            d: 4,
            e: 5,
            f: 6,
            g: 7,
        };
        let not_randomized = NotRandomized {
            a: 1,
            b: 2,
            c: 3,
            d: 4,
            e: 5,
            f: 6,
            g: 7,
        };

        let debug_randomized = format!("{:?}", randomized).replace("Randomized", "");
        let debug_not_randomized = format!("{:?}", not_randomized).replace("NotRandomized", "");

        assert_eq!(debug_randomized, debug_not_randomized);
    }

    #[test]
    fn empty_debug() {
        #[layout_randomize(Debug)]
        struct Empty {}
    }

    #[test]
    fn vis() {
        mod my_module {
            use core::default::Default; 

            #[layout_randomize]
            #[derive(Default)]
            pub struct DataStruct {
                pub a: i32,
                b: i32,
                c: i32,
                d: i32,
                e: i32,
            }
        }

        use my_module::DataStruct;

        let mut ds = DataStruct::default();
        ds.a = 30;
        // ds.b = 40;
    }

    #[test]
    fn dst() {
        #[layout_randomize(Debug)]
        struct HasDST {
            a: i32,
            b: i32,
            c: i32,
            d: i32,
            #[dst]
            dst: [u8],
        }
    }

    // any other tests?
}
