//! Python bindings for the MyHospitals Data API client.
//!
//! This module provides a Python interface to the Rust client using PyO3.

use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3_async_runtimes::tokio::future_into_py;
use std::sync::Arc;

use crate::{Client, DEFAULT_BASE_URL};

/// Convert a serializable value to a Python object (dict/list/primitive).
fn to_py_object<T: serde::Serialize>(py: Python<'_>, value: &T) -> PyResult<PyObject> {
    let json_value = serde_json::to_value(value)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
    json_to_py(py, &json_value)
}

/// Convert a serde_json::Value to a Python object.
fn json_to_py(py: Python<'_>, value: &serde_json::Value) -> PyResult<PyObject> {
    match value {
        serde_json::Value::Null => Ok(py.None()),
        serde_json::Value::Bool(b) => Ok(b.into_pyobject(py)?.to_owned().into_any().unbind()),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i.into_pyobject(py)?.into_any().unbind())
            } else if let Some(f) = n.as_f64() {
                Ok(f.into_pyobject(py)?.into_any().unbind())
            } else {
                Ok(py.None())
            }
        }
        serde_json::Value::String(s) => Ok(s.into_pyobject(py)?.into_any().unbind()),
        serde_json::Value::Array(arr) => {
            let list = pyo3::types::PyList::empty(py);
            for item in arr {
                list.append(json_to_py(py, item)?)?;
            }
            Ok(list.into_any().unbind())
        }
        serde_json::Value::Object(map) => {
            let dict = PyDict::new(py);
            for (k, v) in map {
                dict.set_item(k, json_to_py(py, v)?)?;
            }
            Ok(dict.into_any().unbind())
        }
    }
}

/// Python client for the MyHospitals Data API.
///
/// This client provides async methods to access Australian hospital data
/// from the AIHW MyHospitals API.
///
/// Example:
///     ```python
///     import asyncio
///     from aihw_myhospitals_api import Client
///
///     async def main():
///         client = Client()
///         datasets = await client.get_datasets()
///         for ds in datasets["result"]:
///             print(ds["data_set_name"])
///
///     asyncio.run(main())
///     ```
#[pyclass(name = "Client")]
pub struct PyClient {
    client: Arc<Client>,
}

#[pymethods]
impl PyClient {
    /// Create a new client.
    ///
    /// Args:
    ///     base_url: Optional base URL for the API. Defaults to the production URL.
    #[new]
    #[pyo3(signature = (base_url=None))]
    fn new(base_url: Option<&str>) -> Self {
        let url = base_url.unwrap_or(DEFAULT_BASE_URL);
        PyClient {
            client: Arc::new(Client::new(url)),
        }
    }

    /// Get all caveats.
    ///
    /// Returns a dict with "result" (list of caveats) and "version_information".
    fn get_caveats<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let response =
                client.get_caveats().send().await.map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string())
                })?;
            // Extract the inner value which implements Serialize
            let inner = response.into_inner();
            Python::with_gil(|py| to_py_object(py, &inner))
        })
    }

    /// Get a specific caveat by code.
    ///
    /// Args:
    ///     caveat_code: The caveat code to retrieve.
    #[pyo3(signature = (caveat_code))]
    fn get_caveat_by_code<'py>(
        &self,
        py: Python<'py>,
        caveat_code: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let response = client
                .get_caveats_by_caveat_code()
                .caveat_code(&caveat_code)
                .send()
                .await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            let inner = response.into_inner();
            Python::with_gil(|py| to_py_object(py, &inner))
        })
    }

    /// Get all suppressions.
    fn get_suppressions<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let response =
                client.get_suppressions().send().await.map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string())
                })?;
            let inner = response.into_inner();
            Python::with_gil(|py| to_py_object(py, &inner))
        })
    }

    /// Get a specific suppression by code.
    #[pyo3(signature = (suppression_code))]
    fn get_suppression_by_code<'py>(
        &self,
        py: Python<'py>,
        suppression_code: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let response = client
                .get_suppressions_by_suppression_code()
                .suppression_code(&suppression_code)
                .send()
                .await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            let inner = response.into_inner();
            Python::with_gil(|py| to_py_object(py, &inner))
        })
    }

    /// Get all datasets.
    ///
    /// Args:
    ///     measure_code: Optional filter by measure code.
    ///     reported_measure_code: Optional filter by reported measure code.
    #[pyo3(signature = (measure_code=None, reported_measure_code=None))]
    fn get_datasets<'py>(
        &self,
        py: Python<'py>,
        measure_code: Option<Vec<String>>,
        reported_measure_code: Option<Vec<String>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let mut builder = client.get_datasets();
            if let Some(codes) = measure_code {
                builder = builder.measure_code(codes);
            }
            if let Some(codes) = reported_measure_code {
                builder = builder.reported_measure_code(codes);
            }
            let response = builder
                .send()
                .await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            let inner = response.into_inner();
            Python::with_gil(|py| to_py_object(py, &inner))
        })
    }

    /// Get a specific dataset by ID.
    #[pyo3(signature = (dataset_id))]
    fn get_dataset_by_id<'py>(
        &self,
        py: Python<'py>,
        dataset_id: i32,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let response = client
                .get_datasets_by_dataset_id()
                .dataset_id(dataset_id)
                .send()
                .await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            let inner = response.into_inner();
            Python::with_gil(|py| to_py_object(py, &inner))
        })
    }

    /// Get data items for a specific dataset.
    #[pyo3(signature = (dataset_id))]
    fn get_dataset_data_items<'py>(
        &self,
        py: Python<'py>,
        dataset_id: i32,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let response = client
                .get_datasets_by_dataset_id_data_items()
                .dataset_id(dataset_id)
                .send()
                .await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            let inner = response.into_inner();
            Python::with_gil(|py| to_py_object(py, &inner))
        })
    }

    /// Get all measures.
    ///
    /// Args:
    ///     measure_category_code: Optional filter by measure category codes.
    #[pyo3(signature = (measure_category_code=None))]
    fn get_measures<'py>(
        &self,
        py: Python<'py>,
        measure_category_code: Option<Vec<String>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let mut builder = client.get_measures();
            if let Some(codes) = measure_category_code {
                builder = builder.measure_category_code(codes);
            }
            let response = builder
                .send()
                .await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            let inner = response.into_inner();
            Python::with_gil(|py| to_py_object(py, &inner))
        })
    }

    /// Get a specific measure by code.
    #[pyo3(signature = (measure_code))]
    fn get_measure_by_code<'py>(
        &self,
        py: Python<'py>,
        measure_code: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let response = client
                .get_measures_by_measure_code()
                .measure_code(&measure_code)
                .send()
                .await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            let inner = response.into_inner();
            Python::with_gil(|py| to_py_object(py, &inner))
        })
    }

    /// Get data items for a specific measure.
    #[pyo3(signature = (measure_code))]
    fn get_measure_data_items<'py>(
        &self,
        py: Python<'py>,
        measure_code: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let response = client
                .get_measures_by_measure_code_data_items()
                .measure_code(&measure_code)
                .send()
                .await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            let inner = response.into_inner();
            Python::with_gil(|py| to_py_object(py, &inner))
        })
    }

    /// Get available reporting units for a specific measure.
    #[pyo3(signature = (measure_code))]
    fn get_measure_reporting_units_available<'py>(
        &self,
        py: Python<'py>,
        measure_code: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let response = client
                .get_measures_by_measure_code_reporting_units_available()
                .measure_code(&measure_code)
                .send()
                .await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            let inner = response.into_inner();
            Python::with_gil(|py| to_py_object(py, &inner))
        })
    }

    /// Get all measure categories.
    fn get_measure_categories<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let response =
                client.get_measure_categories().send().await.map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string())
                })?;
            let inner = response.into_inner();
            Python::with_gil(|py| to_py_object(py, &inner))
        })
    }

    /// Get a specific measure category by code.
    #[pyo3(signature = (measure_category_code))]
    fn get_measure_category_by_code<'py>(
        &self,
        py: Python<'py>,
        measure_category_code: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let response = client
                .get_measure_categories_by_measure_category_code()
                .measure_category_code(&measure_category_code)
                .send()
                .await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            let inner = response.into_inner();
            Python::with_gil(|py| to_py_object(py, &inner))
        })
    }

    /// Get measures in a specific category.
    #[pyo3(signature = (measure_category_code))]
    fn get_measure_category_measures<'py>(
        &self,
        py: Python<'py>,
        measure_category_code: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let response = client
                .get_measure_categories_by_measure_category_code_measures()
                .measure_category_code(&measure_category_code)
                .send()
                .await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            let inner = response.into_inner();
            Python::with_gil(|py| to_py_object(py, &inner))
        })
    }

    /// Get all reported measures.
    #[pyo3(signature = (reported_measure_category_code=None))]
    fn get_reported_measures<'py>(
        &self,
        py: Python<'py>,
        reported_measure_category_code: Option<Vec<String>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let mut builder = client.get_reported_measures();
            if let Some(codes) = reported_measure_category_code {
                builder = builder.reported_measure_category_code(codes);
            }
            let response = builder
                .send()
                .await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            let inner = response.into_inner();
            Python::with_gil(|py| to_py_object(py, &inner))
        })
    }

    /// Get a specific reported measure by code.
    #[pyo3(signature = (reported_measure_code))]
    fn get_reported_measure_by_code<'py>(
        &self,
        py: Python<'py>,
        reported_measure_code: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let response = client
                .get_reported_measures_by_reported_measure_code()
                .reported_measure_code(&reported_measure_code)
                .send()
                .await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            let inner = response.into_inner();
            Python::with_gil(|py| to_py_object(py, &inner))
        })
    }

    /// Get data items for a specific reported measure.
    #[pyo3(signature = (reported_measure_code))]
    fn get_reported_measure_data_items<'py>(
        &self,
        py: Python<'py>,
        reported_measure_code: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let response = client
                .get_reported_measures_by_reported_measure_code_data_items()
                .reported_measure_code(&reported_measure_code)
                .send()
                .await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            let inner = response.into_inner();
            Python::with_gil(|py| to_py_object(py, &inner))
        })
    }

    /// Get all reported measure categories.
    fn get_reported_measure_categories<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let response = client
                .get_reported_measure_categories()
                .send()
                .await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            let inner = response.into_inner();
            Python::with_gil(|py| to_py_object(py, &inner))
        })
    }

    /// Get a specific reported measure category by code.
    #[pyo3(signature = (reported_measure_category_code))]
    fn get_reported_measure_category_by_code<'py>(
        &self,
        py: Python<'py>,
        reported_measure_category_code: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let response = client
                .get_reported_measure_categories_by_reported_measure_category_code()
                .reported_measure_category_code(&reported_measure_category_code)
                .send()
                .await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            let inner = response.into_inner();
            Python::with_gil(|py| to_py_object(py, &inner))
        })
    }

    /// Get reported measures in a specific category.
    #[pyo3(signature = (reported_measure_category_code))]
    fn get_reported_measure_category_reported_measures<'py>(
        &self,
        py: Python<'py>,
        reported_measure_category_code: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let response = client
                .get_reported_measure_categories_by_reported_measure_category_code_reported_measures()
                .reported_measure_category_code(&reported_measure_category_code)
                .send()
                .await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            let inner = response.into_inner();
            Python::with_gil(|py| to_py_object(py, &inner))
        })
    }

    /// Get all reporting units.
    ///
    /// Args:
    ///     reporting_unit_type_code: Optional filter by reporting unit type codes.
    #[pyo3(signature = (reporting_unit_type_code=None))]
    fn get_reporting_units<'py>(
        &self,
        py: Python<'py>,
        reporting_unit_type_code: Option<Vec<String>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let mut builder = client.get_reporting_units();
            if let Some(codes) = reporting_unit_type_code {
                builder = builder.reporting_unit_type_code(codes);
            }
            let response = builder
                .send()
                .await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            let inner = response.into_inner();
            Python::with_gil(|py| to_py_object(py, &inner))
        })
    }

    /// Get a specific reporting unit by code.
    #[pyo3(signature = (reporting_unit_code))]
    fn get_reporting_unit_by_code<'py>(
        &self,
        py: Python<'py>,
        reporting_unit_code: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let response = client
                .get_reporting_units_by_reporting_unit_code()
                .reporting_unit_code(&reporting_unit_code)
                .send()
                .await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            let inner = response.into_inner();
            Python::with_gil(|py| to_py_object(py, &inner))
        })
    }

    /// Get data items for a specific reporting unit.
    #[pyo3(signature = (reporting_unit_code))]
    fn get_reporting_unit_data_items<'py>(
        &self,
        py: Python<'py>,
        reporting_unit_code: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let response = client
                .get_reporting_units_by_reporting_unit_code_data_items()
                .reporting_unit_code(&reporting_unit_code)
                .send()
                .await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            let inner = response.into_inner();
            Python::with_gil(|py| to_py_object(py, &inner))
        })
    }

    /// Get available measures for a specific reporting unit.
    #[pyo3(signature = (reporting_unit_code))]
    fn get_reporting_unit_measures_available<'py>(
        &self,
        py: Python<'py>,
        reporting_unit_code: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let response = client
                .get_reporting_units_by_reporting_unit_code_measures_available()
                .reporting_unit_code(&reporting_unit_code)
                .send()
                .await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            let inner = response.into_inner();
            Python::with_gil(|py| to_py_object(py, &inner))
        })
    }

    /// Get available bricks for a specific reporting unit.
    #[pyo3(signature = (reporting_unit_code))]
    fn get_reporting_unit_bricks_available<'py>(
        &self,
        py: Python<'py>,
        reporting_unit_code: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let response = client
                .get_reporting_units_by_reporting_unit_code_bricks_available()
                .reporting_unit_code(&reporting_unit_code)
                .send()
                .await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            let inner = response.into_inner();
            Python::with_gil(|py| to_py_object(py, &inner))
        })
    }

    /// Get all reporting unit types.
    fn get_reporting_unit_types<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let response = client
                .get_reporting_unit_types()
                .send()
                .await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            let inner = response.into_inner();
            Python::with_gil(|py| to_py_object(py, &inner))
        })
    }

    /// Get a specific reporting unit type by code.
    #[pyo3(signature = (reporting_unit_type_code))]
    fn get_reporting_unit_type_by_code<'py>(
        &self,
        py: Python<'py>,
        reporting_unit_type_code: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let response = client
                .get_reporting_unit_types_by_reporting_unit_type_code()
                .reporting_unit_type_code(&reporting_unit_type_code)
                .send()
                .await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            let inner = response.into_inner();
            Python::with_gil(|py| to_py_object(py, &inner))
        })
    }

    /// Get available bricks for a specific reporting unit type.
    #[pyo3(signature = (reporting_unit_type_code))]
    fn get_reporting_unit_type_bricks_available<'py>(
        &self,
        py: Python<'py>,
        reporting_unit_type_code: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let response = client
                .get_reporting_unit_types_by_reporting_unit_type_code_bricks_available()
                .reporting_unit_type_code(&reporting_unit_type_code)
                .send()
                .await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            let inner = response.into_inner();
            Python::with_gil(|py| to_py_object(py, &inner))
        })
    }

    /// Get flat data extract by measure category code.
    ///
    /// Returns raw numeric data values. Use for data analysis.
    ///
    /// Args:
    ///     measure_category_code: The measure category code.
    ///     skip: Number of records to skip (pagination).
    ///     top: Number of records to return (max 1000).
    ///     reporting_unit_type_code: Optional filter by reporting unit type codes.
    ///     measure_code: Optional filter by measure codes.
    ///     reporting_unit_code: Optional filter by reporting unit codes.
    ///     start_date: Optional start date filter (ISO format).
    ///     end_date: Optional end date filter (ISO format).
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (measure_category_code, skip=0, top=100, reporting_unit_type_code=None, measure_code=None, reporting_unit_code=None, start_date=None, end_date=None))]
    fn get_flat_data_extract<'py>(
        &self,
        py: Python<'py>,
        measure_category_code: String,
        skip: i32,
        top: u32,
        reporting_unit_type_code: Option<Vec<String>>,
        measure_code: Option<Vec<String>>,
        reporting_unit_code: Option<Vec<String>>,
        start_date: Option<String>,
        end_date: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let top_nonzero = std::num::NonZeroU32::new(top.max(1)).ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>("top must be > 0")
            })?;

            let mut builder = client
                .get_flat_data_extract_by_measure_category_code()
                .measure_category_code(&measure_category_code)
                .skip(skip)
                .top(top_nonzero);

            if let Some(codes) = reporting_unit_type_code {
                builder = builder.reporting_unit_type_code(codes);
            }
            if let Some(codes) = measure_code {
                builder = builder.measure_code(codes);
            }
            if let Some(codes) = reporting_unit_code {
                builder = builder.reporting_unit_code(codes);
            }
            if let Some(date) = start_date {
                builder = builder.start_date(date);
            }
            if let Some(date) = end_date {
                builder = builder.end_date(date);
            }

            let response = builder
                .send()
                .await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            let inner = response.into_inner();
            Python::with_gil(|py| to_py_object(py, &inner))
        })
    }

    /// Get flat formatted data extract by measure category code.
    ///
    /// Returns display-ready formatted values. Use for presentation.
    ///
    /// Args:
    ///     measure_category_code: The measure category code.
    ///     skip: Number of records to skip (pagination).
    ///     top: Number of records to return (max 1000).
    ///     reporting_unit_type_code: Optional filter by reporting unit type codes.
    ///     measure_code: Optional filter by measure codes.
    ///     reporting_unit_code: Optional filter by reporting unit codes.
    ///     start_date: Optional start date filter (ISO format).
    ///     end_date: Optional end date filter (ISO format).
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (measure_category_code, skip=0, top=100, reporting_unit_type_code=None, measure_code=None, reporting_unit_code=None, start_date=None, end_date=None))]
    fn get_flat_formatted_data_extract<'py>(
        &self,
        py: Python<'py>,
        measure_category_code: String,
        skip: i32,
        top: u32,
        reporting_unit_type_code: Option<Vec<String>>,
        measure_code: Option<Vec<String>>,
        reporting_unit_code: Option<Vec<String>>,
        start_date: Option<String>,
        end_date: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let top_nonzero = std::num::NonZeroU32::new(top.max(1)).ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>("top must be > 0")
            })?;

            let mut builder = client
                .get_flat_formatted_data_extract_by_measure_category_code()
                .measure_category_code(&measure_category_code)
                .skip(skip)
                .top(top_nonzero);

            if let Some(codes) = reporting_unit_type_code {
                builder = builder.reporting_unit_type_code(codes);
            }
            if let Some(codes) = measure_code {
                builder = builder.measure_code(codes);
            }
            if let Some(codes) = reporting_unit_code {
                builder = builder.reporting_unit_code(codes);
            }
            if let Some(date) = start_date {
                builder = builder.start_date(date);
            }
            if let Some(date) = end_date {
                builder = builder.end_date(date);
            }

            let response = builder
                .send()
                .await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            let inner = response.into_inner();
            Python::with_gil(|py| to_py_object(py, &inner))
        })
    }

    /// Get available simple download codes.
    fn get_simple_download_codes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let response = client
                .get_simple_downloads_download_codes()
                .send()
                .await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            let inner = response.into_inner();
            Python::with_gil(|py| to_py_object(py, &inner))
        })
    }

    /// Download a simple data file (XLSX).
    ///
    /// Returns the file contents as bytes.
    #[pyo3(signature = (download_code))]
    fn download_simple<'py>(
        &self,
        py: Python<'py>,
        download_code: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            use futures::StreamExt;

            let response = client
                .get_simple_downloads_by_download_code()
                .download_code(&download_code)
                .send()
                .await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

            let mut bytes = Vec::new();
            let mut stream = response.into_inner();
            while let Some(chunk) = stream.next().await {
                let chunk = chunk.map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string())
                })?;
                bytes.extend_from_slice(&chunk);
            }

            Python::with_gil(|py| Ok(pyo3::types::PyBytes::new(py, &bytes).unbind().into_any()))
        })
    }

    /// Get available measure download codes.
    fn get_measure_download_codes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let response = client
                .get_measure_downloads_measure_download_codes()
                .send()
                .await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            let inner = response.into_inner();
            Python::with_gil(|py| to_py_object(py, &inner))
        })
    }

    /// Download a measure data file (XLSX).
    ///
    /// Returns the file contents as bytes.
    #[pyo3(signature = (measure_download_code))]
    fn download_measure<'py>(
        &self,
        py: Python<'py>,
        measure_download_code: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            use futures::StreamExt;

            let response = client
                .get_measure_downloads_by_measure_download_code()
                .measure_download_code(&measure_download_code)
                .send()
                .await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

            let mut bytes = Vec::new();
            let mut stream = response.into_inner();
            while let Some(chunk) = stream.next().await {
                let chunk = chunk.map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string())
                })?;
                bytes.extend_from_slice(&chunk);
            }

            Python::with_gil(|py| Ok(pyo3::types::PyBytes::new(py, &bytes).unbind().into_any()))
        })
    }

    /// Download measure data across reporting units (XLSX).
    ///
    /// Returns the file contents as bytes.
    #[pyo3(signature = (measure_download_code))]
    fn download_measure_across_reporting_units<'py>(
        &self,
        py: Python<'py>,
        measure_download_code: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            use futures::StreamExt;

            let response = client
                .get_measure_downloads_across_reporting_units_by_measure_download_code()
                .measure_download_code(&measure_download_code)
                .send()
                .await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

            let mut bytes = Vec::new();
            let mut stream = response.into_inner();
            while let Some(chunk) = stream.next().await {
                let chunk = chunk.map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string())
                })?;
                bytes.extend_from_slice(&chunk);
            }

            Python::with_gil(|py| Ok(pyo3::types::PyBytes::new(py, &bytes).unbind().into_any()))
        })
    }

    /// Get available reporting unit datasheet codes.
    fn get_reporting_unit_datasheet_codes<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            let response = client
                .get_reporting_units_downloads_datasheet_codes()
                .send()
                .await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            let inner = response.into_inner();
            Python::with_gil(|py| to_py_object(py, &inner))
        })
    }

    /// Download a reporting unit datasheet (XLSX).
    ///
    /// Returns the file contents as bytes.
    #[pyo3(signature = (datasheet_code, reporting_unit_code))]
    fn download_reporting_unit_datasheet<'py>(
        &self,
        py: Python<'py>,
        datasheet_code: String,
        reporting_unit_code: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        future_into_py(py, async move {
            use futures::StreamExt;

            let response = client
                .get_reporting_units_downloads_by_datasheet_code_by_reporting_unit_code()
                .datasheet_code(&datasheet_code)
                .reporting_unit_code(&reporting_unit_code)
                .send()
                .await
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

            let mut bytes = Vec::new();
            let mut stream = response.into_inner();
            while let Some(chunk) = stream.next().await {
                let chunk = chunk.map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string())
                })?;
                bytes.extend_from_slice(&chunk);
            }

            Python::with_gil(|py| Ok(pyo3::types::PyBytes::new(py, &bytes).unbind().into_any()))
        })
    }
}

/// The Python module definition.
#[pymodule]
#[pyo3(name = "_core")]
fn aihw_myhospitals_api(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyClient>()?;
    m.add("DEFAULT_BASE_URL", DEFAULT_BASE_URL)?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
