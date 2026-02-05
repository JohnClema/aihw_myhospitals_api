"""Type stubs for aihw_myhospitals_api."""

from typing import Any, Coroutine, Optional

__version__: str
DEFAULT_BASE_URL: str

class Client:
    """
    Python client for the MyHospitals Data API.

    This client provides async methods to access Australian hospital data
    from the AIHW MyHospitals API.

    Example:
        ```python
        import asyncio
        from aihw_myhospitals_api import Client

        async def main():
            client = Client()
            datasets = await client.get_datasets()
            for ds in datasets["result"]:
                print(ds["data_set_name"])

        asyncio.run(main())
        ```
    """

    def __init__(self, base_url: Optional[str] = None) -> None:
        """
        Create a new client.

        Args:
            base_url: Optional base URL for the API. Defaults to the production URL.
        """
        ...

    # Caveats
    def get_caveats(self) -> Coroutine[Any, Any, dict[str, Any]]:
        """Get all caveats."""
        ...

    def get_caveat_by_code(self, caveat_code: str) -> Coroutine[Any, Any, dict[str, Any]]:
        """Get a specific caveat by code."""
        ...

    # Suppressions
    def get_suppressions(self) -> Coroutine[Any, Any, dict[str, Any]]:
        """Get all suppressions."""
        ...

    def get_suppression_by_code(self, suppression_code: str) -> Coroutine[Any, Any, dict[str, Any]]:
        """Get a specific suppression by code."""
        ...

    # Datasets
    def get_datasets(
        self,
        measure_code: Optional[list[str]] = None,
        reported_measure_code: Optional[list[str]] = None,
    ) -> Coroutine[Any, Any, dict[str, Any]]:
        """Get all datasets."""
        ...

    def get_dataset_by_id(self, dataset_id: int) -> Coroutine[Any, Any, dict[str, Any]]:
        """Get a specific dataset by ID."""
        ...

    def get_dataset_data_items(self, dataset_id: int) -> Coroutine[Any, Any, dict[str, Any]]:
        """Get data items for a specific dataset."""
        ...

    # Measures
    def get_measures(
        self,
        measure_category_code: Optional[list[str]] = None,
    ) -> Coroutine[Any, Any, dict[str, Any]]:
        """Get all measures."""
        ...

    def get_measure_by_code(self, measure_code: str) -> Coroutine[Any, Any, dict[str, Any]]:
        """Get a specific measure by code."""
        ...

    def get_measure_data_items(self, measure_code: str) -> Coroutine[Any, Any, dict[str, Any]]:
        """Get data items for a specific measure."""
        ...

    def get_measure_reporting_units_available(self, measure_code: str) -> Coroutine[Any, Any, dict[str, Any]]:
        """Get available reporting units for a specific measure."""
        ...

    # Measure Categories
    def get_measure_categories(self) -> Coroutine[Any, Any, dict[str, Any]]:
        """Get all measure categories."""
        ...

    def get_measure_category_by_code(self, measure_category_code: str) -> Coroutine[Any, Any, dict[str, Any]]:
        """Get a specific measure category by code."""
        ...

    def get_measure_category_measures(self, measure_category_code: str) -> Coroutine[Any, Any, dict[str, Any]]:
        """Get measures in a specific category."""
        ...

    # Reported Measures
    def get_reported_measures(
        self,
        reported_measure_category_code: Optional[list[str]] = None,
    ) -> Coroutine[Any, Any, dict[str, Any]]:
        """Get all reported measures."""
        ...

    def get_reported_measure_by_code(self, reported_measure_code: str) -> Coroutine[Any, Any, dict[str, Any]]:
        """Get a specific reported measure by code."""
        ...

    def get_reported_measure_data_items(self, reported_measure_code: str) -> Coroutine[Any, Any, dict[str, Any]]:
        """Get data items for a specific reported measure."""
        ...

    # Reported Measure Categories
    def get_reported_measure_categories(self) -> Coroutine[Any, Any, dict[str, Any]]:
        """Get all reported measure categories."""
        ...

    def get_reported_measure_category_by_code(self, reported_measure_category_code: str) -> Coroutine[Any, Any, dict[str, Any]]:
        """Get a specific reported measure category by code."""
        ...

    def get_reported_measure_category_reported_measures(self, reported_measure_category_code: str) -> Coroutine[Any, Any, dict[str, Any]]:
        """Get reported measures in a specific category."""
        ...

    # Reporting Units
    def get_reporting_units(
        self,
        reporting_unit_type_code: Optional[list[str]] = None,
    ) -> Coroutine[Any, Any, dict[str, Any]]:
        """Get all reporting units."""
        ...

    def get_reporting_unit_by_code(self, reporting_unit_code: str) -> Coroutine[Any, Any, dict[str, Any]]:
        """Get a specific reporting unit by code."""
        ...

    def get_reporting_unit_data_items(self, reporting_unit_code: str) -> Coroutine[Any, Any, dict[str, Any]]:
        """Get data items for a specific reporting unit."""
        ...

    def get_reporting_unit_measures_available(self, reporting_unit_code: str) -> Coroutine[Any, Any, dict[str, Any]]:
        """Get available measures for a specific reporting unit."""
        ...

    def get_reporting_unit_bricks_available(self, reporting_unit_code: str) -> Coroutine[Any, Any, dict[str, Any]]:
        """Get available bricks for a specific reporting unit."""
        ...

    # Reporting Unit Types
    def get_reporting_unit_types(self) -> Coroutine[Any, Any, dict[str, Any]]:
        """Get all reporting unit types."""
        ...

    def get_reporting_unit_type_by_code(self, reporting_unit_type_code: str) -> Coroutine[Any, Any, dict[str, Any]]:
        """Get a specific reporting unit type by code."""
        ...

    def get_reporting_unit_type_bricks_available(self, reporting_unit_type_code: str) -> Coroutine[Any, Any, dict[str, Any]]:
        """Get available bricks for a specific reporting unit type."""
        ...

    # Flat Data Extracts
    def get_flat_data_extract(
        self,
        measure_category_code: str,
        skip: int = 0,
        top: int = 100,
        reporting_unit_type_code: Optional[list[str]] = None,
        measure_code: Optional[list[str]] = None,
        reporting_unit_code: Optional[list[str]] = None,
        start_date: Optional[str] = None,
        end_date: Optional[str] = None,
    ) -> Coroutine[Any, Any, dict[str, Any]]:
        """
        Get flat data extract by measure category code.

        Returns raw numeric data values. Use for data analysis.

        Args:
            measure_category_code: The measure category code.
            skip: Number of records to skip (pagination).
            top: Number of records to return (max 1000).
            reporting_unit_type_code: Optional filter by reporting unit type codes.
            measure_code: Optional filter by measure codes.
            reporting_unit_code: Optional filter by reporting unit codes.
            start_date: Optional start date filter (ISO format).
            end_date: Optional end date filter (ISO format).
        """
        ...

    def get_flat_formatted_data_extract(
        self,
        measure_category_code: str,
        skip: int = 0,
        top: int = 100,
        reporting_unit_type_code: Optional[list[str]] = None,
        measure_code: Optional[list[str]] = None,
        reporting_unit_code: Optional[list[str]] = None,
        start_date: Optional[str] = None,
        end_date: Optional[str] = None,
    ) -> Coroutine[Any, Any, dict[str, Any]]:
        """
        Get flat formatted data extract by measure category code.

        Returns display-ready formatted values. Use for presentation.

        Args:
            measure_category_code: The measure category code.
            skip: Number of records to skip (pagination).
            top: Number of records to return (max 1000).
            reporting_unit_type_code: Optional filter by reporting unit type codes.
            measure_code: Optional filter by measure codes.
            reporting_unit_code: Optional filter by reporting unit codes.
            start_date: Optional start date filter (ISO format).
            end_date: Optional end date filter (ISO format).
        """
        ...

    # Downloads
    def get_simple_download_codes(self) -> Coroutine[Any, Any, dict[str, Any]]:
        """Get available simple download codes."""
        ...

    def download_simple(self, download_code: str) -> Coroutine[Any, Any, bytes]:
        """Download a simple data file (XLSX). Returns the file contents as bytes."""
        ...

    def get_measure_download_codes(self) -> Coroutine[Any, Any, dict[str, Any]]:
        """Get available measure download codes."""
        ...

    def download_measure(self, measure_download_code: str) -> Coroutine[Any, Any, bytes]:
        """Download a measure data file (XLSX). Returns the file contents as bytes."""
        ...

    def download_measure_across_reporting_units(self, measure_download_code: str) -> Coroutine[Any, Any, bytes]:
        """Download measure data across reporting units (XLSX). Returns the file contents as bytes."""
        ...

    def get_reporting_unit_datasheet_codes(self) -> Coroutine[Any, Any, dict[str, Any]]:
        """Get available reporting unit datasheet codes."""
        ...

    def download_reporting_unit_datasheet(
        self,
        datasheet_code: str,
        reporting_unit_code: str,
    ) -> Coroutine[Any, Any, bytes]:
        """Download a reporting unit datasheet (XLSX). Returns the file contents as bytes."""
        ...
