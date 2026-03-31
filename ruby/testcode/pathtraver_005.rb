require_relative 'shared'

# vuln-code-snippet start ruby_pt_send_file
def download_public_file(req)
  path = req.param('path')
  content = File.read("/public/#{path}") # vuln-code-snippet vuln-line ruby_pt_send_file
  BenchmarkResponse.ok(content)
end
# vuln-code-snippet end ruby_pt_send_file
