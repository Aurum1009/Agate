from ag_types import *;

class NotPositiveError:
    msg: str
    def __init__(self, msg: str) -> None:
        self.msg = msg

class LiteralValue:
    def __init__(self) -> None:
        return

class Value:
    def __init__(self) -> None:
        return
    
    def type_of(self) -> Type:
        return AgAny(False)

class IntValue(Value):
    value: int
    
    def __init__(self, value: int) -> None:
        self.value = value
    def type_of(self) -> Type:
        return AgInt(False)
          
class UintValue(Value):
    value: int
    
    def __init__(self, value: int) -> None:
        if value < 0:
            raise NotPositiveError(f"Value {value} is not positive")
        else:
            self.value = value

class FloatValue(Value, ):
    value: float
    
    def __init__(self, value: float) -> None:
        self.value = value