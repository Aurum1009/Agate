from value import LiteralValue

class Expr:
    lines: tuple[int, int]
    
    def __init__(self, lines: tuple[int, int]) -> None:
        self.lines = lines

    def left(self) -> int:
        return self.lines[0]
    def right(self) -> int:
        return self.lines[1]
    
class LiteralExpr:
    value: LiteralValue
    
    def __init__(self, value: LiteralValue, lines: tuple[int, int]) -> None:
        super.__init__(lines)
        self.value = value
        
    