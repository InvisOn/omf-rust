use crate::array::{
    PyBooleanArray, PyBoundaryArray, PyColorArray, PyFreeformSubblockArray, PyGradientArray,
    PyImageArray, PyIndexArray, PyNameArray, PyNumberArray, PyRegularSubblockArray, PyScalarArray,
    PySegmentArray, PyTexcoordArray, PyTextArray, PyTriangleArray, PyVectorArray, PyVertexArray,
};
use crate::errors::OmfException;
use crate::validate::PyProblem;
use crate::PyProject;
use omf::error::Error::IoError;
use omf::file::{Limits, Reader};
use omf::Color;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3_stub_gen::derive::*;
use std::fs::File;

#[gen_stub_pyclass]
#[pyclass(name = "Limits")]
/// Memory limits for reading OMF files.
#[derive(Clone, Copy)]
pub struct PyLimits {
    #[pyo3(get, set)]
    pub json_bytes: Option<u64>,
    #[pyo3(get, set)]
    pub image_bytes: Option<u64>,
    #[pyo3(get, set)]
    pub image_dim: Option<u32>,
    #[pyo3(get, set)]
    pub validation: Option<u32>,
}

impl From<Limits> for PyLimits {
    fn from(limits: Limits) -> Self {
        Self {
            json_bytes: limits.json_bytes,
            image_bytes: limits.image_bytes,
            image_dim: limits.image_dim,
            validation: limits.validation,
        }
    }
}

impl From<PyLimits> for Limits {
    fn from(py_limits: PyLimits) -> Self {
        Self {
            json_bytes: py_limits.json_bytes,
            image_bytes: py_limits.image_bytes,
            image_dim: py_limits.image_dim,
            validation: py_limits.validation,
        }
    }
}

impl Default for PyLimits {
    fn default() -> Self {
        Self::new()
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyLimits {
    #[new]
    pub fn new() -> Self {
        Limits::default().into()
    }
}

#[gen_stub_pyclass]
#[pyclass(name = "Reader")]
/// OMF reader object.
///
/// Typical usage pattern is:
///
/// - Create the reader object.
/// - Optional: retrieve the file version with `reader.version()`.
/// - Optional: adjust the limits with `reader.set_limits(...)`.
/// - Read the project from the file with `reader.project()`.
/// - Iterate through the project's contents to find the elements and attributes you want to load.
/// - For each of those items load the array or image data.
///
/// **Warning:**
///     When loading arrays and images from OMF files, beware of "zip bombs"
///     where data is maliciously crafted to expand to an excessive size when decompressed,
///     leading to a potential denial of service attack.
///     Use the limits provided and check arrays sizes before allocating memory.
pub struct PyReader(Reader);
#[gen_stub_pymethods]
#[pymethods]
impl PyReader {
    #[new]
    /// Creates a reader from an OMF file path.
    ///
    /// Makes only the minimum number of reads to check the file header and footer.
    /// Fails with an error if an IO error occurs or the file isn’t in OMF 2 format.
    pub fn new(filepath: &str) -> PyResult<Self> {
        let file = File::open(filepath).map_err(|e| OmfException::py_err(IoError(e)))?;
        let reader = Reader::new(file).map_err(OmfException::py_err)?;
        Ok(Self(reader))
    }

    /// Returns the current limits.
    fn limits(&self) -> PyLimits {
        self.0.limits().into()
    }

    /// Sets the memory limits.
    ///
    /// These limits prevent the reader from consuming excessive system resources, which might
    /// allow denial of service attacks with maliciously crafted files. Running without limits
    /// is not recommended.
    fn set_limits(&mut self, limits: &PyLimits) {
        self.0.set_limits((*limits).into());
    }

    /// Return the version number of the file, which can only be [2, 0] right now.
    pub fn version(&self) -> [u32; 2] {
        self.0.version()
    }

    /// Reads, validates, and returns the root `Project` object from the file.
    ///
    /// Fails with an error if an IO error occurs, the `json_bytes` limit is exceeded, or validation
    /// fails. Validation warnings are returned alongside the project if successful or included
    /// with the errors if not.
    fn project(&self) -> PyResult<(PyProject, Vec<PyProblem>)> {
        let (project, problems) = self.0.project().map_err(OmfException::py_err)?;

        let problems_array: Vec<PyProblem> =
            problems.iter().map(|e| PyProblem(e.clone())).collect();
        Ok((PyProject(project), problems_array))
    }

    /// Read a Scalar array.
    pub fn array_scalars(&self, array: &PyScalarArray) -> PyResult<Vec<f64>> {
        self.0
            .array_scalars(&array.0)
            .map_err(OmfException::py_err)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(OmfException::py_err)
    }

    /// Read a Vertex array.
    pub fn array_vertices(&self, array: &PyVertexArray) -> PyResult<Vec<[f64; 3]>> {
        self.0
            .array_vertices(&array.0)
            .map_err(OmfException::py_err)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(OmfException::py_err)
    }

    /// Read a Segment array.
    pub fn array_segments(&self, array: &PySegmentArray) -> PyResult<Vec<[u32; 2]>> {
        self.0
            .array_segments(&array.0)
            .map_err(OmfException::py_err)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(OmfException::py_err)
    }

    /// Read an Index array.
    pub fn array_indices(&self, array: &PyIndexArray) -> PyResult<Vec<Option<u32>>> {
        self.0
            .array_indices(&array.0)
            .map_err(OmfException::py_err)?
            .collect::<Result<Vec<Option<u32>>, _>>()
            .map_err(OmfException::py_err)
    }

    /// Read a Triangle array.
    pub fn array_triangles(&self, array: &PyTriangleArray) -> PyResult<Vec<[u32; 3]>> {
        self.0
            .array_triangles(&array.0)
            .map_err(OmfException::py_err)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(OmfException::py_err)
    }

    /// Read a Color array.
    pub fn array_color(&self, array: &PyColorArray) -> PyResult<Vec<Option<Color>>> {
        self.0
            .array_colors(&array.0)
            .map_err(OmfException::py_err)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(OmfException::py_err)
    }

    /// Read a Gradient array.
    pub fn array_gradient(&self, array: &PyGradientArray) -> PyResult<Vec<[u8; 4]>> {
        self.0
            .array_gradient(&array.0)
            .map_err(OmfException::py_err)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(OmfException::py_err)
    }

    /// Read a Name array.
    pub fn array_names(&self, array: &PyNameArray) -> PyResult<Vec<String>> {
        self.0
            .array_names(&array.0)
            .map_err(OmfException::py_err)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(OmfException::py_err)
    }

    /// Read a Texcoord array.
    pub fn array_texcoords(&self, array: &PyTexcoordArray) -> PyResult<Vec<[f64; 2]>> {
        self.0
            .array_texcoords(&array.0)
            .map_err(OmfException::py_err)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(OmfException::py_err)
    }

    /// Read bytes of an Image.
    pub fn image_bytes<'p>(
        &self,
        py: Python<'p>,
        array: &PyImageArray,
    ) -> PyResult<Bound<'p, PyBytes>> {
        self.0
            .array_bytes(&array.0)
            .map_err(OmfException::py_err)
            .map(|b| PyBytes::new_bound(py, &b))
    }

    /// Read a Number array.
    pub fn array_numbers(&self, array: &PyNumberArray) -> PyResult<Vec<f64>> {
        let numbers_f64 = self
            .0
            .array_numbers(&array.0)
            .map_err(OmfException::py_err)?
            .try_into_f64()
            .map_err(OmfException::py_err)?;

        Ok(numbers_f64
            .into_iter()
            .filter_map(|item| match item {
                Ok(Some(value)) => Some(value),
                _ => None,
            })
            .collect())
    }

    /// Read a Vector array.
    pub fn array_vectors(&self, array: &PyVectorArray) -> PyResult<Vec<Option<[f64; 3]>>> {
        self.0
            .array_vectors(&array.0)
            .map_err(OmfException::py_err)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(OmfException::py_err)
    }

    /// Read a Text array.
    pub fn array_text(&self, array: &PyTextArray) -> PyResult<Vec<Option<String>>> {
        self.0
            .array_text(&array.0)
            .map_err(OmfException::py_err)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(OmfException::py_err)
    }

    /// Read a Boolean array.
    pub fn array_booleans(&self, array: &PyBooleanArray) -> PyResult<Vec<Option<bool>>> {
        self.0
            .array_booleans(&array.0)
            .map_err(OmfException::py_err)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(OmfException::py_err)
    }

    /// Read a Boundary array.
    pub fn array_boundaries(&self, array: &PyBoundaryArray) -> PyResult<Vec<f64>> {
        let numbers_f64 = self
            .0
            .array_boundaries(&array.0)
            .map_err(OmfException::py_err)?
            .try_into_f64()
            .map_err(OmfException::py_err)?;

        Ok(numbers_f64
            .into_iter()
            .filter_map(|item| match item {
                Ok(boundary) => Some(boundary.value()),
                _ => None,
            })
            .collect())
    }

    /// Read a RegularSubblock array.
    pub fn array_regular_subblocks(
        &self,
        array: &PyRegularSubblockArray,
    ) -> PyResult<Vec<([u32; 3], [u32; 6])>> {
        self.0
            .array_regular_subblocks(&array.0)
            .map_err(OmfException::py_err)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(OmfException::py_err)
    }

    /// Read a FreeformSubblock array.
    pub fn array_freeform_subblocks(
        &self,
        array: &PyFreeformSubblockArray,
    ) -> PyResult<Vec<([u32; 3], [f64; 6])>> {
        self.0
            .array_freeform_subblocks(&array.0)
            .map_err(OmfException::py_err)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(OmfException::py_err)
    }
}
