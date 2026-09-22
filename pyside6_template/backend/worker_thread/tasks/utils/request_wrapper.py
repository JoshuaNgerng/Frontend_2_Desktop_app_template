from functools import wraps
from inspect import signature
from typing import Any, get_type_hints

from pydantic import BaseModel, ValidationError


def flexible_request(fn):
    sig = signature(fn)
    params = list(sig.parameters.values())

    if len(params) < 2:
        raise RuntimeError("Function must have at least 2 params")

    second_param = params[1]
    second_param_name = second_param.name

    type_hints = get_type_hints(fn)
    expected_model = type_hints.get(second_param_name)

    # skip validation mode:
    # def fn(db, data: Any = None)
    skip_validation = (
        expected_model is Any
        and second_param.default is None
    )

    @wraps(fn)
    def wrapper(*args, **kwargs):
        bound = sig.bind_partial(*args, **kwargs)
        bound.apply_defaults()

        # skip validation entirely for placeholder Any=None
        if not skip_validation:
            if second_param_name not in bound.arguments:
                raise ValueError("Request Invalid")

            raw_value = bound.arguments[second_param_name]

            # second param must be a pydantic model type
            if (
                not expected_model
                or not issubclass(expected_model, BaseModel) # type: ignore
                or not isinstance(expected_model, type)
            ):
                raise RuntimeError(
                    f"Second param '{second_param_name}' must be a Pydantic model"
                )

            try:
                # already correct model
                if isinstance(raw_value, expected_model):
                    parsed = raw_value

                # dict -> validate into correct model
                elif isinstance(raw_value, dict):
                    parsed = expected_model.model_validate(raw_value)

                # invalid type
                else:
                    raise ValueError("Request Invalid")

            except ValidationError:
                raise ValueError("Request Invalid")

            # replace validated model
            bound.arguments[second_param_name] = parsed

        result = fn(*bound.args, **bound.kwargs)

        # auto dump pydantic response
        if isinstance(result, BaseModel):
            return result.model_dump()

        return result

    return wrapper