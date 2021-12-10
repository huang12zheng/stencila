import io
import json
import traceback

# fmt: off

try:
    from numpy import ndarray
except ImportError:
    class ndarray: pass

try:
    from pandas import DataFrame
except ImportError:
    class DataFrame: pass

try:
    import matplotlib
    import matplotlib.pyplot
    
    matplotlib.use('Agg')
    
    # Monkey patch pyplot.show to return itself to
    # indicates that an image should be returned as an output
    def show(*args, **kwargs):
        return matplotlib.pyplot.show
    matplotlib.pyplot.show = show
    
    MATPLOTLIB_IMPORTED = True
except ImportError:
    MATPLOTLIB_IMPORTED = False

# fmt: on


def decode_value(json_):
    """Decode JSON to a Python value"""
    return json.loads(json_)


def encode_value(value):
    """Decode a Python value to JSON"""
    converted = convert_value(value)
    return json.dumps(converted, default=lambda value: repr(value))


def convert_value(value):
    """Convert a value prior to encoding"""

    # Shortcut for primitive types
    if isinstance(value, (bool, int, float, str)):
        return value

    if isinstance(value, ndarray):
        return convert_ndarray(value)

    if isinstance(value, DataFrame):
        return convert_dataframe(value)

    if MATPLOTLIB_IMPORTED and is_matplotlib(value):
        return convert_matplotlib()

    return value


def convert_ndarray(array):
    """Convert a numpy `ndarray` to an `Array`"""
    array.tolist()


def convert_dataframe(df):
    """Convert a Pandas `Dataframe` to a `Datatable`"""
    import numpy

    columns = []
    for column_name in df.columns:
        column = df[column_name]
        values = convert_ndarray(column)
        if column.dtype in (numpy.bool_, numpy.bool8):
            validator = dict(type="BooleanValidator")
            values = [bool(row) for row in values]
        elif column.dtype in (numpy.int8, numpy.int16, numpy.int32, numpy.int64):
            validator = dict(type="IntegerValidator")
            values = [int(row) for row in values]
        elif column.dtype in (numpy.float16, numpy.float32, numpy.float64):
            validator = dict(type="NumberValidator")
            values = [float(row) for row in values]
        elif column.dtype in (
            numpy.str_,
            numpy.unicode_,
        ):
            validator = dict(type="StringValidator")
        else:
            validator = None

        columns.append(
            dict(
                type="DatatableColumn",
                name=str(column_name),  # Ensure name is a string
                values=values,
                validator=dict(type="ArrayValidator", itemsValidator=validator),
            )
        )

    return dict(type="Datatable", columns=columns)


def is_matplotlib(value):
    """Is the value a matplotlib value or return of a matplotlib call?"""
    from matplotlib.artist import Artist
    from matplotlib.figure import Figure

    if (
        value == matplotlib.pyplot.show
        or isinstance(value, Artist)
        or isinstance(value, Figure)
    ):
        return True

    # This is somewhat crude but allows for calls that return lists of
    # matplotlib types not just single objects e.g. `pyplot.plot()`
    rep = repr(value)
    return rep.startswith("<matplotlib.") or rep.startswith("[<matplotlib.")


def convert_matplotlib():
    """Convert the current matplotlib figure to a `ImageObject`"""
    from matplotlib import pyplot
    import base64

    image = io.BytesIO()
    pyplot.savefig(image, format="png")
    pyplot.close()
    src = "data:image/png;base64," + base64.encodebytes(image.getvalue()).decode()
    return dict(type="ImageObject", contentUrl=src)


def encode_exception(exc):
    """Encode an exception to a `CodeMessage`"""
    code_error = {"type": "CodeError", "errorType": exc.__class__.__name__}

    if hasattr(exc, "message"):
        code_error["errorMessage"] = exc.message
    else:
        code_error["errorMessage"] = str(exc)

    stack_trace = io.StringIO()
    traceback.print_exc(file=stack_trace)
    code_error["stackTrace"] = stack_trace.getvalue()

    return json.dumps(code_error)