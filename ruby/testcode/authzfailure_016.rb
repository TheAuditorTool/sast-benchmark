require_relative 'shared'

# vuln-code-snippet start ruby_authz_token_scope
def api_scoped_action(req)
  token_scope = req.header('X-Token-Scope')
  return BenchmarkResponse.error('forbidden', 403) unless token_scope.include?('admin') # vuln-code-snippet safe-line ruby_authz_token_scope
  BenchmarkResponse.ok('admin action performed')
end
# vuln-code-snippet end ruby_authz_token_scope
