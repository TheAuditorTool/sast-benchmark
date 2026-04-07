require_relative 'shared'

# vuln-code-snippet start ruby_reflect_public_send_file
def handler(req)
  result = File.public_send(req.param('op').to_sym, req.param('path')) # vuln-code-snippet vuln-line ruby_reflect_public_send_file
  BenchmarkResponse.json({ result: result.to_s })
end
# vuln-code-snippet end ruby_reflect_public_send_file
