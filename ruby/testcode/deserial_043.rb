require_relative 'shared'

# vuln-code-snippet start ruby_deser_no_marshal_ever
def no_marshal_ever_handler(req)
  # Marshal is never called -- all persistence uses JSON.parse
  data = req.post('data')
  obj = JSON.parse(data)  # vuln-code-snippet safe-line ruby_deser_no_marshal_ever
  BenchmarkResponse.json({ result: obj })
end
# vuln-code-snippet end ruby_deser_no_marshal_ever
