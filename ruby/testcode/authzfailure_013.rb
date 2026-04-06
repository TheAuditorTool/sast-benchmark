require_relative 'shared'

# vuln-code-snippet start ruby_authz_unscoped_update
def update_record(req)
  record_id = req.param('id')
  value = req.post('value')
  BenchmarkResponse.ok("record #{record_id} updated to #{value}") # vuln-code-snippet vuln-line ruby_authz_unscoped_update
end
# vuln-code-snippet end ruby_authz_unscoped_update
