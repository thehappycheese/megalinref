
import pytest
import re
def test_constructor():
    import megalinref as mlr
    with pytest.raises(
        Exception,
        match=re.escape("Please use Lookup.from_dict() or Lookup.from_binary() to create an instance of this class.")
    ):
        mlr.Lookup()