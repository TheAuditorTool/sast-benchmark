require_relative 'shared'

# vuln-code-snippet start ruby_weakrand_uuid
def generate_identifier(req)
  _name = req.param('name')
  uid = SecureRandom.uuid # vuln-code-snippet safe-line ruby_weakrand_uuid
  BenchmarkResponse.ok(uid)
end
# vuln-code-snippet end ruby_weakrand_uuid
