require_relative 'shared'

# vuln-code-snippet start ruby_headerinj_constant_type
def json_response(req)
  BenchmarkResponse.new(200, '{}', { 'Content-Type' => 'application/json' }) # vuln-code-snippet safe-line ruby_headerinj_constant_type
end
# vuln-code-snippet end ruby_headerinj_constant_type
