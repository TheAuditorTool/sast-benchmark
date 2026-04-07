require_relative 'shared'
require 'json'

# vuln-code-snippet start ruby_xxe_json_instead_of_xml
def parse_json_instead_handler(req)
  data = req.post('data')
  obj = JSON.parse(data) # vuln-code-snippet safe-line ruby_xxe_json_instead_of_xml
  BenchmarkResponse.json({ result: obj })
end
# vuln-code-snippet end ruby_xxe_json_instead_of_xml
