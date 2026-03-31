require_relative 'shared'

# vuln-code-snippet start ruby_codeinj_integer_parse
def process_age(req)
  raw = req.param('age')
  age = Integer(raw) # vuln-code-snippet safe-line ruby_codeinj_integer_parse
  category = age < 18 ? "minor" : "adult"
  BenchmarkResponse.ok(category)
rescue ArgumentError
  BenchmarkResponse.bad_request("invalid age")
end
# vuln-code-snippet end ruby_codeinj_integer_parse
