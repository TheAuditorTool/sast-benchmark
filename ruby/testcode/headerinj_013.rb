require_relative 'shared'

# vuln-code-snippet start ruby_headerinj_rack_response
def rack_response(req)
  header_name = req.param('header')
  header_value = req.param('value')
  [200, { header_name => header_value }, ['ok']] # vuln-code-snippet vuln-line ruby_headerinj_rack_response
end
# vuln-code-snippet end ruby_headerinj_rack_response
