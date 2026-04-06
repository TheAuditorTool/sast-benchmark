require_relative 'shared'

# vuln-code-snippet start ruby_authz_graphql_no_auth
def graphql_resolve(req)
  query = req.param('query')
  user_id = req.param('id')
  # No field-level authorization
  BenchmarkResponse.json({ id: user_id, ssn: '123-45-6789' }) # vuln-code-snippet vuln-line ruby_authz_graphql_no_auth
end
# vuln-code-snippet end ruby_authz_graphql_no_auth
