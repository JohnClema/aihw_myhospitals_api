"""Tests for the Python bindings."""

import pytest


def test_import():
    """Test that the module can be imported."""
    from aihw_myhospitals_api import Client, DEFAULT_BASE_URL, __version__

    assert Client is not None
    assert DEFAULT_BASE_URL == "https://myhospitalsapi.aihw.gov.au"
    assert __version__ is not None


def test_client_creation():
    """Test that a client can be created."""
    from aihw_myhospitals_api import Client

    # Default URL
    client = Client()
    assert client is not None

    # Custom URL
    client = Client("https://custom.example.com")
    assert client is not None


@pytest.mark.asyncio
async def test_get_measure_categories():
    """Test fetching measure categories from the API."""
    from aihw_myhospitals_api import Client

    client = Client()
    response = await client.get_measure_categories()

    assert "result" in response
    assert isinstance(response["result"], list)
    assert len(response["result"]) > 0

    # Check structure of first category
    category = response["result"][0]
    assert "measure_category_code" in category
    assert "measure_category_name" in category


@pytest.mark.asyncio
async def test_get_reporting_unit_types():
    """Test fetching reporting unit types from the API."""
    from aihw_myhospitals_api import Client

    client = Client()
    response = await client.get_reporting_unit_types()

    assert "result" in response
    assert isinstance(response["result"], list)

    # Check expected types exist
    type_codes = [t["reporting_unit_type_code"] for t in response["result"]]
    assert "H" in type_codes  # Hospital
    assert "NAT" in type_codes  # National


@pytest.mark.asyncio
async def test_get_datasets():
    """Test fetching datasets from the API."""
    from aihw_myhospitals_api import Client

    client = Client()
    response = await client.get_datasets()

    assert "result" in response
    assert isinstance(response["result"], list)
    assert len(response["result"]) > 0

    # Check version information is present
    assert "version_information" in response


@pytest.mark.asyncio
async def test_flat_data_extract():
    """Test fetching flat data extract with pagination."""
    from aihw_myhospitals_api import Client

    client = Client()

    # First get a measure category code
    categories = await client.get_measure_categories()
    category_code = categories["result"][0]["measure_category_code"]

    # Fetch data with pagination
    response = await client.get_flat_data_extract(
        measure_category_code=category_code,
        skip=0,
        top=10,
    )

    assert "result" in response or "pagination" in response
