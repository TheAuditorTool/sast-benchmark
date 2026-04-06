require_relative 'shared'

# vuln-code-snippet start ruby_massassign_explicit_attrs
def update_explicit(req)
  name = req.post('name')
  email = req.post('email')
  attrs = { name: name, email: email } # vuln-code-snippet safe-line ruby_massassign_explicit_attrs
  BenchmarkResponse.ok("updated: #{attrs}")
end
# vuln-code-snippet end ruby_massassign_explicit_attrs
