use std::collections::{HashMap, HashSet};
use std::future::Future;
use std::str::FromStr;

use anyhow::Result;
use futures::FutureExt;
use pyo3::prelude::*;

use crate::input::InputSink;
use crate::values::{ColorValue, RangeValue};
use crate::{Attr, AttrId, Client, Input, InputId, Node, NodeId};

fn asyncify<R: for<'py> IntoPyObject<'py>>(
    py: Python,
    f: impl Future<Output = Result<R>> + Send + 'static,
) -> PyResult<Bound<PyAny>> {
    let f = f.map(|r| r.map_err(PyErr::from));
    return pyo3_async_runtimes::tokio::future_into_py(py, f);
}

#[pymodule]
fn photonic_grpc_client(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyNode>()?;
    m.add_class::<PyAttr>()?;
    m.add_class::<PyInput>()?;

    m.add_class::<PyNodeId>()?;
    m.add_class::<PyAttrId>()?;
    m.add_class::<PyInputId>()?;

    m.add_function(wrap_pyfunction!(connect, m)?)?;

    return Ok(());
}

#[pyfunction]
fn connect<'py>(py: Python<'py>, url: String) -> PyResult<Bound<'py, PyAny>> {
    asyncify(py, async move {
        let client = Client::connect(url.parse()?).await?;
        return Ok(PyClient(client));
    })
}

#[pyclass(frozen, name = "Client")]
struct PyClient(#[allow(unused)] Client);

#[pymethods]
impl PyClient {
    fn nodes(slf: Py<Self>, py: Python) -> PyResult<Bound<PyAny>> {
        asyncify(py, async move {
            return Ok(slf.get().0.nodes().await?);
        })
    }

    fn inputs(slf: Py<Self>, py: Python) -> PyResult<Bound<PyAny>> {
        asyncify(py, async move {
            return Ok(slf.get().0.inputs().await?);
        })
    }

    fn root(slf: Py<Self>, py: Python) -> PyResult<Bound<PyAny>> {
        asyncify(py, async move {
            return Ok(PyNode(slf.get().0.root().await?));
        })
    }

    fn node(slf: Py<Self>, py: Python, name: PyNodeId) -> PyResult<Bound<PyAny>> {
        asyncify(py, async move {
            return Ok(PyNode(slf.get().0.node(&name.0).await?));
        })
    }

    fn attribute(slf: Py<Self>, py: Python, name: PyAttrId) -> PyResult<Bound<PyAny>> {
        asyncify(py, async move {
            return Ok(PyAttr(slf.get().0.attr(&name.0).await?));
        })
    }

    fn input(slf: Py<Self>, py: Python, name: PyInputId) -> PyResult<Bound<PyAny>> {
        asyncify(py, async move {
            return Ok(PyInput(slf.get().0.input(&name.0).await?));
        })
    }
}

#[pyclass(frozen, name = "Node")]
struct PyNode(#[allow(unused)] Node);

#[pymethods]
impl PyNode {
    #[getter]
    fn name(&self) -> &NodeId {
        return self.0.name();
    }

    #[getter]
    fn kind(&self) -> &str {
        return self.0.kind();
    }

    #[getter]
    fn nodes(&self) -> &HashMap<String, NodeId> {
        return self.0.nodes();
    }

    #[getter]
    fn attrs(&self) -> &HashSet<AttrId> {
        return &self.0.attrs();
    }

    fn node(slf: Py<Self>, py: Python, name: String) -> PyResult<Bound<PyAny>> {
        asyncify(py, async move {
            let Some(node) = slf.get().0.node(&name).await? else {
                return Ok(None);
            };

            return Ok(Some(PyNode(node)));
        })
    }

    fn attr(slf: Py<Self>, py: Python, name: String) -> PyResult<Bound<PyAny>> {
        asyncify(py, async move {
            let Some(attr) = slf.get().0.attr(&name).await? else {
                return Ok(None);
            };

            return Ok(Some(PyAttr(attr)));
        })
    }

    fn __repr__(&self) -> PyResult<String> {
        return Ok(format!("Node(name='{}', kind={})", self.0.name(), self.0.kind()));
    }
}

#[pyclass(frozen, name = "Attr")]
struct PyAttr(#[allow(unused)] Attr);

#[pymethods]
impl PyAttr {
    #[getter]
    fn name(&self) -> &AttrId {
        return self.0.name();
    }

    #[getter]
    fn kind(&self) -> &str {
        return self.0.kind();
    }

    #[getter]
    fn r#type(&self) -> &str {
        return self.0.value_type();
    }

    #[getter]
    fn attrs(&self) -> &HashSet<AttrId> {
        return self.0.attrs();
    }

    #[getter]
    fn inputs(&self) -> &HashMap<String, InputId> {
        return self.0.inputs();
    }

    fn attr(slf: Py<Self>, py: Python, name: String) -> PyResult<Bound<PyAny>> {
        asyncify(py, async move {
            let Some(attr) = slf.get().0.attr(&name).await? else {
                return Ok(None);
            };

            return Ok(Some(PyAttr(attr)));
        })
    }

    fn input(slf: Py<Self>, py: Python, name: String) -> PyResult<Bound<PyAny>> {
        asyncify(py, async move {
            let Some(input) = slf.get().0.input(&name).await? else {
                return Ok(None);
            };

            return Ok(Some(PyInput(input)));
        })
    }

    fn __repr__(&self) -> PyResult<String> {
        return Ok(format!("Attr(name='{}', kind={}, type={})", self.0.name(), self.0.kind(), self.0.value_type()));
    }
}

#[pyclass(frozen, name = "Input")]
struct PyInput(#[allow(unused)] Input);

#[pymethods]
impl PyInput {
    #[getter]
    fn name(&self) -> &InputId {
        return self.0.name();
    }

    #[getter]
    pub fn r#type(&self) -> String {
        return self.0.value_type().to_string();
    }

    pub fn send(slf: Py<Self>, py: Python, value: Py<PyAny>) -> PyResult<Bound<PyAny>> {
        asyncify(py, async move {
            fn extract<T: for<'a> FromPyObject<'a>>(value: Py<PyAny>) -> PyResult<T> {
                Python::with_gil(|py| value.extract(py))
            }

            return match slf.get().0.sink() {
                InputSink::Trigger(sink) => sink.trigger().await,
                InputSink::Boolean(sink) => sink.send(extract(value)?).await,
                InputSink::Integer(sink) => sink.send(extract(value)?).await,
                InputSink::Decimal(sink) => sink.send(extract(value)?).await,
                InputSink::Color(sink) => sink.send(extract(value)?).await,
                InputSink::IntegerRange(sink) => sink.send(extract(value)?).await,
                InputSink::DecimalRange(sink) => sink.send(extract(value)?).await,
                InputSink::ColorRange(sink) => sink.send(extract(value)?).await,
            };
        })
    }

    fn __repr__(&self) -> PyResult<String> {
        return Ok(format!("Input(name='{}', type={})", self.0.name(), self.0.value_type()));
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
#[pyclass(frozen, name = "NodeId")]
pub struct PyNodeId(#[allow(dead_code)] NodeId);

#[pymethods]
impl PyNodeId {
    #[new]
    fn new(s: &str) -> Self {
        return Self(NodeId::from_str(s).expect("Infallible"));
    }

    fn __str__(&self) -> PyResult<String> {
        return Ok(self.0.to_string());
    }

    fn __repr__(&self) -> PyResult<String> {
        return Ok(format!("NodeId('{}')", self.0.to_string()));
    }
}

impl<'py> IntoPyObject<'py> for NodeId {
    type Target = PyNodeId;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        return Ok(Bound::new(py, PyNodeId(self))?);
    }
}

impl<'py> IntoPyObject<'py> for &NodeId {
    type Target = PyNodeId;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        return Ok(Bound::new(py, PyNodeId(self.clone()))?);
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
#[pyclass(frozen, name = "AttrId")]
pub struct PyAttrId(#[allow(dead_code)] AttrId);

#[pymethods]
impl PyAttrId {
    #[new]
    #[pyo3(signature = (node, * path))]
    fn new(node: PyNodeId, path: Vec<String>) -> Self {
        return Self(AttrId::new(node.0, path));
    }

    fn extend(&self, attr: String) -> PyResult<Self> {
        return Ok(PyAttrId(self.0.clone().extend(attr)));
    }

    fn __str__(&self) -> PyResult<String> {
        return Ok(self.0.to_string());
    }

    fn __repr__(&self) -> PyResult<String> {
        return Ok(format!("AttrId('{}')", self.0.to_string()));
    }
}

impl<'py> IntoPyObject<'py> for AttrId {
    type Target = PyAttrId;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        return Ok(Bound::new(py, PyAttrId(self.clone()))?);
    }
}

impl<'py> IntoPyObject<'py> for &AttrId {
    type Target = PyAttrId;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        return Ok(Bound::new(py, PyAttrId(self.clone()))?);
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
#[pyclass(frozen, name = "InputId")]
pub struct PyInputId(InputId);

#[pymethods]
impl PyInputId {
    #[new]
    fn new(s: &str) -> Self {
        return Self(InputId::from_str(s).expect("Infallible"));
    }

    fn __str__(&self) -> PyResult<String> {
        return Ok(self.0.to_string());
    }

    fn __repr__(&self) -> PyResult<String> {
        return Ok(format!("InputId('{}')", self.0.to_string()));
    }
}

impl<'py> IntoPyObject<'py> for InputId {
    type Target = PyInputId;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        return Ok(Bound::new(py, PyInputId(self.clone()))?);
    }
}

impl<'py> IntoPyObject<'py> for &InputId {
    type Target = PyInputId;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        return Ok(Bound::new(py, PyInputId(self.clone()))?);
    }
}

impl<'py> FromPyObject<'py> for ColorValue {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let ob: String = ob.extract()?;
        return Ok(ColorValue::from_str(&ob)?);
    }
}

impl<'py, T: FromPyObject<'py>> FromPyObject<'py> for RangeValue<T> {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let (a, b): (T, T) = ob.extract()?;
        return Ok(RangeValue {
            a,
            b,
        });
    }
}

static_assertions::assert_impl_all!(Client: Send, Sync);
