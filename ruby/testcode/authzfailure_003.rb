require_relative 'shared'

# vuln-code-snippet start ruby_authz_no_pundit
def update_post(req)
  post_id = req.param('id')
  content = req.post('content')
  # No authorize call
  BenchmarkResponse.ok("post #{post_id} updated") # vuln-code-snippet vuln-line ruby_authz_no_pundit
end
# vuln-code-snippet end ruby_authz_no_pundit
