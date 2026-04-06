require_relative 'shared'

module CanCanCan
  def self.authorize!(action, resource); true; end
end

# vuln-code-snippet start ruby_authz_cancancan
def manage_resource(req)
  resource_id = req.param('id')
  CanCanCan.authorize!(:manage, { id: resource_id }) # vuln-code-snippet safe-line ruby_authz_cancancan
  BenchmarkResponse.ok("resource #{resource_id} managed")
end
# vuln-code-snippet end ruby_authz_cancancan
