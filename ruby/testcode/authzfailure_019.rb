require_relative 'shared'

# vuln-code-snippet start ruby_authz_bulk_delete
def bulk_delete(req)
  ids = req.param('ids').split(',')
  # No ownership check on bulk operation
  BenchmarkResponse.ok("deleted: #{ids.join(', ')}") # vuln-code-snippet vuln-line ruby_authz_bulk_delete
end
# vuln-code-snippet end ruby_authz_bulk_delete
