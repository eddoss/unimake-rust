use rustpython_vm::builtins::{PyCode, PyDict, PyFunction};
use rustpython_vm::{AsObject, PyObjectRef, PyResult, VirtualMachine};
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
//////////////////////////////////////////////////////////////////
// Argument kind
//////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArgumentKind {
    Regular,
    Positional,
    Keyword,
    Variadic,
    VariadicKeyword,
}

impl std::fmt::Display for ArgumentKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

//////////////////////////////////////////////////////////////////
// Argument
//////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Argument {
    name: String,
    kind: ArgumentKind,
    index: usize,
    annotation: Option<String>,
}

impl Argument {
    fn new(name: String, kind: ArgumentKind, index: usize) -> Self {
        Self {
            name,
            kind,
            index,
            annotation: None,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn kind(&self) -> ArgumentKind {
        self.kind.clone()
    }

    pub fn annotation(&self) -> Option<String> {
        self.annotation.clone()
    }

    pub fn is_regular(&self) -> bool {
        matches!(self.kind, ArgumentKind::Regular)
    }

    pub fn is_positional(&self) -> bool {
        matches!(self.kind, ArgumentKind::Positional)
    }

    pub fn is_keyword(&self) -> bool {
        matches!(self.kind, ArgumentKind::Keyword)
    }

    pub fn is_variadic(&self) -> bool {
        matches!(self.kind, ArgumentKind::Variadic)
    }

    pub fn is_variadic_keyword(&self) -> bool {
        matches!(self.kind, ArgumentKind::VariadicKeyword)
    }
}

//////////////////////////////////////////////////////////////////
// Signature
//////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    args: Vec<Argument>,
}

impl Signature {
    pub fn iter(&self) -> impl Iterator<Item = &Argument> {
        self.args.iter()
    }

    pub fn regular(&self) -> impl Iterator<Item = &Argument> {
        self.args.iter().filter(|&a| a.is_regular())
    }

    pub fn positional(&self) -> impl Iterator<Item = &Argument> {
        self.args.iter().filter(|&a| a.is_positional())
    }

    pub fn keyword(&self) -> impl Iterator<Item = &Argument> {
        self.args.iter().filter(|&a| a.is_keyword())
    }

    pub fn variadic(&self) -> Option<Argument> {
        self.args
            .iter()
            .rfind(|&a| a.is_variadic())
            .map(|a| a.clone())
    }

    pub fn variadic_keyword(&self) -> Option<Argument> {
        self.args
            .iter()
            .rfind(|&a| a.is_variadic_keyword())
            .map(|a| a.clone())
    }
}

impl Signature {
    pub fn of(callable: PyObjectRef, vm: &VirtualMachine) -> PyResult<Self> {
        // There is a 3 case we deal with:
        //   1) Given object is a direct function.
        //   2) Given object is an instance of the callable class.
        //   3) Given object is something else.
        //
        // Cases 1 and 2 are positive, and we must guess which one
        // of the type object is. The last case is a negative, we
        // must return an exception.

        let func = if callable.payload_is::<PyFunction>() {
            // Case 1: direct function
            match callable.downcast::<PyFunction>() {
                Ok(v) => v,
                Err(_) => {
                    return Err(vm.new_runtime_error(
                        "Failed to downcast callable object to the 'PyFunction'".to_string(),
                    ));
                }
            }
        } else {
            // Case 2: instance of the callable class
            match callable.get_attr("__call__", vm)?.downcast::<PyFunction>() {
                Ok(v) => v,
                Err(_) => {
                    // Case 3: not callable
                    return Err(vm.new_runtime_error(
                        "Failed to downcast '__call__' to the 'PyFunction'".to_string(),
                    ));
                }
            }
        };

        // The signature info exists in __code__ attribute.

        let code = match func
            .as_object()
            .get_attr("__code__", vm)?
            .downcast::<PyCode>()
        {
            Ok(v) => v,
            Err(_) => {
                return Err(vm.new_runtime_error(
                    "Failed to get '__code__' attribute from 'PyFunction'".to_string(),
                ));
            }
        };

        let mut result = Signature { args: Vec::new() };

        // Read var names and its indices.

        code.varnames.iter().enumerate().for_each(|(index, name)| {
            let name = name.to_string();
            result
                .args
                .push(Argument::new(name, ArgumentKind::Regular, index));
        });

        let set = |result: &mut Self, name: String, kind: ArgumentKind| match result
            .args
            .iter_mut()
            .find(|arg| arg.name == name.to_string())
        {
            None => {}
            Some(arg) => arg.kind = kind.clone(),
        };

        let args = code.arg_names();

        // Lookup which vars are positional or keyword.

        for (kind, names) in [
            (ArgumentKind::Positional, args.posonlyargs),
            (ArgumentKind::Keyword, args.kwonlyargs),
        ] {
            names.iter().for_each(|name| {
                set(&mut result, name.to_string(), kind.clone());
            });
        }

        // Lookup which vars are variadic or variadic-keyword.

        for (kind, name) in [
            (ArgumentKind::Variadic, args.vararg),
            (ArgumentKind::VariadicKeyword, args.varkwarg),
        ] {
            if let Some(name) = name {
                set(&mut result, name.to_string(), kind.clone());
            }
        }

        // Extract annotation info from __annotations__ attribute.

        let annotations = func.as_object().get_attr("__annotations__", vm);

        if let Ok(annotations) = annotations {
            let annotations = annotations.downcast::<PyDict>();
            if let Err(_) = annotations {
                return Err(vm.new_runtime_error(
                    "Failed to downcast '__annotations__' to the dict".to_string(),
                ));
            }
            let annotations = annotations.unwrap();
            for (k, v) in annotations {
                if let Ok(name) = k.str(vm) {
                    result
                        .args
                        .iter_mut()
                        .find(|arg| arg.name() == name.to_string())
                        .map(|arg| arg.annotation = Some(v.str(vm).unwrap().to_string()));
                }
            }
        }

        Ok(result)
    }
}
