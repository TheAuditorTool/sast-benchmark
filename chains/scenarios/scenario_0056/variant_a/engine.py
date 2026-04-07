def evaluate_fstring(user_expr: str) -> str:
    code = compile('f"' + user_expr + '"', "<fstring>", "eval")
    return str(eval(code))  
