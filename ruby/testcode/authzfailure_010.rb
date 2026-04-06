require_relative 'shared'

# vuln-code-snippet start ruby_authz_file_scoped
def download_file_safe(req)
  file_id = req.param('file_id')
  current_user = req.cookie('user_id')
  owner = "user_#{current_user}"
  path = "/uploads/#{owner}/#{file_id}"
  return BenchmarkResponse.error('forbidden', 403) unless path.start_with?("/uploads/#{owner}/") # vuln-code-snippet safe-line ruby_authz_file_scoped
  BenchmarkResponse.ok("downloading: #{path}")
end
# vuln-code-snippet end ruby_authz_file_scoped
