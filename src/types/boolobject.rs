// Copyright (c) 2017-present PyO3 Project and Contributors
use conversion::{IntoPyObject, PyTryFrom, ToPyObject};
use ffi;
use object::PyObject;
use python::{Python, ToPyPointer};
use types::PyObjectRef;
use FromPyObject;
use PyResult;

/// Represents a Python `bool`.
#[repr(transparent)]
pub struct PyBool(PyObject);

pyobject_native_type!(PyBool, ffi::PyBool_Type, ffi::PyBool_Check);

impl PyBool {
    /// Depending on `val`, returns `py.True()` or `py.False()`.
    #[inline]
    pub fn new(py: Python, val: bool) -> &PyBool {
        unsafe { py.from_borrowed_ptr(if val { ffi::Py_True() } else { ffi::Py_False() }) }
    }

    /// Gets whether this boolean is `true`.
    #[inline]
    pub fn is_true(&self) -> bool {
        self.as_ptr() == unsafe { ::ffi::Py_True() }
    }
}

/// Converts a rust `bool` to a Python `bool`.
impl ToPyObject for bool {
    #[inline]
    fn to_object(&self, py: Python) -> PyObject {
        unsafe {
            PyObject::from_borrowed_ptr(
                py,
                if *self {
                    ffi::Py_True()
                } else {
                    ffi::Py_False()
                },
            )
        }
    }
}

impl IntoPyObject for bool {
    #[inline]
    fn into_object(self, py: Python) -> PyObject {
        PyBool::new(py, self).into()
    }
}

/// Converts a Python `bool` to a rust `bool`.
///
/// Fails with `TypeError` if the input is not a Python `bool`.
impl<'source> FromPyObject<'source> for bool {
    fn extract(obj: &'source PyObjectRef) -> PyResult<Self> {
        Ok(<PyBool as PyTryFrom>::try_from(obj)?.is_true())
    }
}

#[cfg(test)]
mod test {
    use conversion::ToPyObject;
    use objectprotocol::ObjectProtocol;
    use python::Python;
    use types::{PyBool, PyObjectRef};

    #[test]
    fn test_true() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        assert!(PyBool::new(py, true).is_true());
        let t: &PyObjectRef = PyBool::new(py, true).into();
        assert_eq!(true, t.extract().unwrap());
        assert_eq!(true.to_object(py), PyBool::new(py, true).into());
    }

    #[test]
    fn test_false() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        assert!(!PyBool::new(py, false).is_true());
        let t: &PyObjectRef = PyBool::new(py, false).into();
        assert_eq!(false, t.extract().unwrap());
        assert_eq!(false.to_object(py), PyBool::new(py, false).into());
    }
}
