mod csv {
    extern crate csv;
    mod read {
        use crate::block::Block;
        use crate::chainblocksc::CBStrings;
        use crate::chainblocksc::CBTypeInfo_Details;
        use crate::chainblocksc::CBTypeInfo_Details_Path;
        use crate::chainblocksc::CBType_ContextVar;
        use crate::chainblocksc::CBType_Path;
        use crate::chainblocksc::CBTypesInfo;
        use crate::core::getRootPath;
        use crate::core::init;
        use crate::core::registerBlock;
        use crate::types::common_type;
        use crate::types::ClonedVar;
        use crate::types::Context;
        use crate::types::ParameterInfo;
        use crate::types::Parameters;
        use crate::types::Type;
        use crate::types::Types;
        use crate::types::Var;
        use core::any::Any;
        use core::cell::Cell;
        use csv::Error;
        use csv::Reader;
        use csv::StringRecord;
        use std::convert::TryFrom;
        use std::ffi::CString;
        use std::io::Read;
        use std::path::Path;

        // either all in one go or one record per iteration
        // from string variable or path (a file)

        struct Row {
            strs: Vec<CString>,
            vars: Vec<Var>,
        }

        struct CSVRead {
            input_types: Types,
            output_types: Types,
            buffer_types: Types,
            parameters: Parameters,
            iterating: bool,
            looped: bool,
            reinit: bool,
            source: ClonedVar,
            records: Option<Box<dyn Iterator<Item = Result<StringRecord, Error>>>>,
            rows: Vec<Row>,
            slurp: Vec<Var>,
        }

        impl Default for CSVRead {
            fn default() -> Self {
                let path = Type {
                    basicType: CBType_Path,
                    details: CBTypeInfo_Details {
                        path: CBTypeInfo_Details_Path {
                            extensions: CBStrings::default(),
                            existing: true,
                            isFile: true,
                            relative: true,
                        },
                    },
                };
                let buffer_types = vec![common_type::string, path];
                let buffer = Type {
                    basicType: CBType_ContextVar,
                    details: CBTypeInfo_Details {
                        contextVarTypes: CBTypesInfo::from(&buffer_types),
                    },
                };
                Self {
                    input_types: vec![common_type::none],
                    output_types: vec![common_type::strings],
                    buffer_types: buffer_types,
                    parameters: vec![
                        ParameterInfo::from((
                            "Source",
                            "A path or path variable to a file or a string buffer variable with CSV data.",
                            vec![path, buffer],
                        )),
                        ParameterInfo::from((
                            "Iterate",
                            "Reads the data one record at a time as in one record per chain iteration. Will output a empty sequence when done (if Looped is not selected).",
                            vec![common_type::bool],
                        )),
                        ParameterInfo::from((
                            "Looped",
                            "When Iterate is selected, if Looped is also selected, the iteration will restart from the first record rather than output a empty sequence.",
                            vec![common_type::bool],
                        )),
                    ],
                    iterating: false,
                    looped: false,
                    reinit: true,
                    source: ClonedVar(Var::default()),
                    records: None,
                    rows: Vec::<Row>::new(),
                    slurp: Vec::<Var>::new()
                }
            }
        }

        impl CSVRead {
            fn load_file_reader(&mut self, path: &str) {
                let root_path = Path::new(getRootPath());
                let fullpath = root_path.join(path);
                let reader = Reader::from_path(fullpath).unwrap();
                self.records = Some(Box::new(reader.into_records()));
            }

            fn load_str_reader(&mut self, text: &'static [u8]) {
                let reader = Reader::from_reader(text);
                self.records = Some(Box::new(reader.into_records()));
            }
        }

        impl Block for CSVRead {
            fn name(&mut self) -> &str {
                "CSV.Read"
            }
            fn inputTypes(&mut self) -> &Types {
                &self.input_types
            }
            fn outputTypes(&mut self) -> &Types {
                // this depends on self.iterating!
                &self.output_types
            }
            fn parameters(&mut self) -> Option<&Parameters> {
                Some(&self.parameters)
            }
            fn setParam(&mut self, idx: i32, value: &Var) {
                match idx {
                    0 => {
                        self.source = ClonedVar::from(value);
                        self.reinit = true;
                    }
                    1 => {
                        self.iterating = bool::try_from(value).unwrap();
                    }
                    2 => {
                        self.looped = bool::try_from(value).unwrap();
                    }
                    _ => {
                        unimplemented!();
                    }
                };
            }
            fn activate(&mut self, _context: &Context, _input: &Var) -> Var {
                if self.reinit {
                    self.records = None;
                    match self.source.0.valueType {
                        CBType_Path => {
                            let vcpath = CString::try_from(&self.source.0).unwrap();
                            let vpath = vcpath.to_str().unwrap();
                            self.load_file_reader(vpath);
                        }
                        _ => panic!("Wrong Source type."),
                    };
                    self.reinit = false;
                }

                if let Some(records) = self.records.as_mut() {
                    if self.iterating {
                        // a single seq of strings
                        self.rows.clear();

                        if let Some(record) = records.next() {
                            if let Ok(data) = record {
                                let it = data.iter();
                                let mut row = Row {
                                    strs: Vec::<CString>::new(),
                                    vars: Vec::<Var>::new(),
                                };
                                for item in it {
                                    let s = CString::new(item).unwrap();
                                    row.vars.push(Var::from(&s));
                                    row.strs.push(s);
                                }
                                self.rows.push(row);
                            }
                        }

                        Var::from(&self.rows[0].vars)
                    } else {
                        self.rows.clear();
                        self.slurp.clear();

                        for record in records {
                            if let Ok(data) = record {
                                let it = data.iter();
                                let mut row = Row {
                                    strs: Vec::<CString>::new(),
                                    vars: Vec::<Var>::new(),
                                };
                                for item in it {
                                    let s = CString::new(item).unwrap();
                                    row.vars.push(Var::from(&s));
                                    row.strs.push(s);
                                }
                                self.rows.push(row);
                            }
                        }

                        for row in &self.rows {
                            self.slurp.push(Var::from(&row.vars));
                        }

                        Var::from(&self.slurp)
                    }
                } else {
                    Var::default()
                }
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
