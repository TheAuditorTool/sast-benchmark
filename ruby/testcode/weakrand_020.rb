require_relative 'shared'

# vuln-code-snippet start ruby_weakrand_object_id_token
def generate_token(req)
  token = object_id.to_s(36) # vuln-code-snippet vuln-line ruby_weakrand_object_id_token
  BenchmarkResponse.json({ token: token })
end
# vuln-code-snippet end ruby_weakrand_object_id_token
