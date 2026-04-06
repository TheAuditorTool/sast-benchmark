require_relative 'shared'

# vuln-code-snippet start ruby_authz_idor_file
def download_file(req)
  file_id = req.param('file_id')
  path = "/uploads/#{file_id}"
  BenchmarkResponse.ok("downloading: #{path}") # vuln-code-snippet vuln-line ruby_authz_idor_file
end
# vuln-code-snippet end ruby_authz_idor_file
