"""
Python client for the MyHospitals Data API.

This package provides an async Python interface to the Australian Institute of
Health and Welfare (AIHW) MyHospitals Data API.

Example:
    >>> import asyncio
    >>> from aihw_myhospitals_api import Client
    >>>
    >>> async def main():
    ...     client = Client()
    ...     datasets = await client.get_datasets()
    ...     for ds in datasets["result"]:
    ...         print(ds["data_set_name"])
    ...
    >>> asyncio.run(main())
"""

from aihw_myhospitals_api._core import Client, DEFAULT_BASE_URL, __version__

__all__ = ["Client", "DEFAULT_BASE_URL", "__version__"]
