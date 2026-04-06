require_relative 'shared'

# vuln-code-snippet start ruby_authz_skip_before
def sensitive_action(req)
  # skip_before_action :check_authorization, only: [:sensitive_action]
  BenchmarkResponse.ok("action performed") # vuln-code-snippet vuln-line ruby_authz_skip_before
end
# vuln-code-snippet end ruby_authz_skip_before
