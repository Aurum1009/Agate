class Type:
    is_nullable: bool
    def __init__(self, is_nullable: bool | None) -> None:
        if is_nullable == None:
            self.is_nullable = False
        else:
            self.is_nullable = is_nullable
    def __str__(self) -> str:
        return f"Unknown type: {self.is_nullable}"

    def can_be_null(self) -> bool:
        return self.is_nullable

class AgInt(Type):
    def __init__(self, is_nullable: bool | None) -> None:
        super().__init__(is_nullable)
    def __str__(self) -> str:
        suffix = "?" if self.is_nullable else ""
        return "int" + suffix
    
class AgUint(Type):
    def __init__(self, is_nullable: bool | None) -> None:
        super().__init__(is_nullable)
    def __str__(self) -> str:
        suffix = "?" if self.is_nullable else ""
        return "uint" + suffix
    
class AgFloat(Type):
    def __init__(self, is_nullable: bool | None) -> None:
        super().__init__(is_nullable)
    def __str__(self) -> str:
        suffix = "?" if self.is_nullable else ""
        return "float" + suffix

class AgChar(Type):
    def __init__(self, is_nullable: bool | None) -> None:
        super().__init__(is_nullable)
    def __str__(self) -> str:
        suffix = "?" if self.is_nullable else ""
        return "char" + suffix
    
class AgString(Type):
    def __init__(self, is_nullable: bool | None) -> None:
        super().__init__(is_nullable)
    def __str__(self) -> str:
        suffix = "?" if self.is_nullable else ""
        return "string" + suffix

class AgAny(Type):
    def __init__(self, is_nullable: bool | None) -> None:
        super().__init__(is_nullable)
    def __str__(self) -> str:
        suffix = "?" if self.is_nullable else ""
        return "any" + suffix