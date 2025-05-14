use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use anyhow::Result;
use pyo3::prelude::*;

use crate::input::InputSink;
use crate::values::{ColorValue, RangeValue};
use crate::{Attr, AttrId, Client, Input, InputId, Node, NodeId};

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
async fn connect(url: String) -> PyResult<PyClient> {
    let uri = url.parse().map_err(anyhow::Error::from)?;

    let client = Client::connect(uri).await?;

    return Ok(PyClient(client));
}

#[pyclass(frozen, name = "Client")]
struct PyClient(#[allow(unused)] Client);

#[pymethods]
impl PyClient {
    async fn nodes(&self) -> PyResult<HashSet<NodeId>> {
        return Ok(self.0.nodes().await?);
    }

    async fn inputs(&self) -> PyResult<HashSet<InputId>> {
        return Ok(self.0.inputs().await?);
    }

    async fn root(&self) -> PyResult<PyNode> {
        return Ok(PyNode(self.0.root().await?));
    }

    async fn node(&self, name: PyNodeId) -> PyResult<PyNode> {
        return Ok(PyNode(self.0.node(&name.0).await?));
    }

    async fn attribute(&self, name: PyAttrId) -> PyResult<PyAttr> {
        return Ok(PyAttr(self.0.attr(&name.0).await?));
    }

    async fn input(&self, name: PyInputId) -> PyResult<PyInput> {
        return Ok(PyInput(self.0.input(&name.0).await?));
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

    async fn node(&self, name: String) -> PyResult<Option<PyNode>> {
        let Some(node) = self.0.node(&name).await? else {
            return Ok(None);
        };

        return Ok(Some(PyNode(node)));
    }

    async fn attr(&self, name: String) -> PyResult<Option<PyAttr>> {
        let Some(attr) = self.0.attr(&name).await? else {
            return Ok(None);
        };

        return Ok(Some(PyAttr(attr)));
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

    async fn attr(&self, name: String) -> PyResult<Option<PyAttr>> {
        let Some(attr) = self.0.attr(&name).await? else {
            return Ok(None);
        };

        return Ok(Some(PyAttr(attr)));
    }

    async fn input(&self, name: String) -> PyResult<Option<PyInput>> {
        let Some(input) = self.0.input(&name).await? else {
            return Ok(None);
        };

        return Ok(Some(PyInput(input)));
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

    pub async fn send(&self, value: Py<PyAny>) -> PyResult<()> {
        fn extract<T: for<'a> FromPyObject<'a>>(value: Py<PyAny>) -> PyResult<T> {
            Python::with_gil(|py| value.extract(py))
        }

        return Ok(match self.0.sink() {
            InputSink::Trigger(sink) => sink.trigger().await?,
            InputSink::Boolean(sink) => sink.send(extract(value)?).await?,
            InputSink::Integer(sink) => sink.send(extract(value)?).await?,
            InputSink::Decimal(sink) => sink.send(extract(value)?).await?,
            InputSink::Color(sink) => sink.send(extract(value)?).await?,
            InputSink::IntegerRange(sink) => sink.send(extract(value)?).await?,
            InputSink::DecimalRange(sink) => sink.send(extract(value)?).await?,
            InputSink::ColorRange(sink) => sink.send(extract(value)?).await?,
        });
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
