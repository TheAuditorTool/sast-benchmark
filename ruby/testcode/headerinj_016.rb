require_relative 'shared'

# vuln-code-snippet start ruby_headerinj_integer_header
def request_id_header(req)
  id = req.param('id').to_i
  BenchmarkResponse.new(200, 'ok', { 'X-Request-Id' => id.to_s }) # vuln-code-snippet safe-line ruby_headerinj_integer_header
end
# vuln-code-snippet end ruby_headerinj_integer_header
