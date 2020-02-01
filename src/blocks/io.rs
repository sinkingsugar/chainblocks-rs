mod csv {
    mod read {
        use crate::block::Block;
        use crate::core::init;
        use crate::core::registerBlock;
        use crate::types::common_type;
        use crate::types::Context;
        use crate::types::ParameterInfo;
        use crate::types::Parameters;
        use crate::types::Type;
        use crate::types::Types;
        use crate::types::Var;

        // either all in one go or one record per iteration
        // from string variable or path (a file)

        struct CSVRead {
            input_types: Types,
            output_types: Types,
            parameters: Parameters,
        }

        impl Default for CSVRead {
            fn default() -> Self {
                Self {
                    input_types: vec![common_type::none],
                    output_types: vec![common_type::strings],
                    parameters: vec![ParameterInfo::from((
                        "Source",
                        "A path to a file or a string buffer variable with CSV data.",
                        vec![common_type::path],
                    ))],
                }
            }
        }

        impl Block for CSVRead {
            fn name(&mut self) -> &str {
                "Read"
            }
            fn inputTypes(&mut self) -> &Types {
                &self.input_types
            }
            fn outputTypes(&mut self) -> &Types {
                &self.output_types
            }
            fn parameters(&mut self) -> Option<&Parameters> {
                Some(&self.parameters)
            }
            fn activate(&mut self, _context: &Context, _input: &Var) -> Var {
                Var::default()
            }
        }

        #[ctor]
        fn attach() {
            init();
            registerBlock::<CSVRead>("CSV.Read");
        }
    }

    mod write {
        // either append to file/buffer or overwrite each activation
    }
}
