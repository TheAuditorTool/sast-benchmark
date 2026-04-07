def generate_report(title: str, body: str) -> str:
    expression = "\"Report: \" + title + \"\\n\" + body"
    return eval(expression, {"title": title, "body": body})  
