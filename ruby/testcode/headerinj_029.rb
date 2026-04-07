require_relative 'shared'

# vuln-code-snippet start ruby_headerinj_transfer_encoding
def set_transfer_encoding(req)
  response = BenchmarkResponse.ok('ok')
  response.headers['Transfer-Encoding'] = req.param('encoding') # vuln-code-snippet vuln-line ruby_headerinj_transfer_encoding
  response
end
# vuln-code-snippet end ruby_headerinj_transfer_encoding
