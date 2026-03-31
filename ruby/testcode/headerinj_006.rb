require_relative 'shared'

require 'uri'

# vuln-code-snippet start ruby_headerinj_encode_value
def set_custom_header(req)
  raw_value = req.param('data')
  encoded = URI.encode_www_form_component(raw_value)
  headers = { 'X-Custom-Data' => encoded }  # vuln-code-snippet safe-line ruby_headerinj_encode_value
  BenchmarkResponse.new(200, 'ok', headers)
end
# vuln-code-snippet end ruby_headerinj_encode_value
