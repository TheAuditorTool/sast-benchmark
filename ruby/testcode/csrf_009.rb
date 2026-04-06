require_relative 'shared'

# vuln-code-snippet start ruby_csrf_skip_conditional
def update_settings(req)
  # skip_before_action :verify_authenticity_token, if: -> { request.format.json? }
  settings = req.post('settings')
  BenchmarkResponse.ok("updated: #{settings}") # vuln-code-snippet vuln-line ruby_csrf_skip_conditional
end
# vuln-code-snippet end ruby_csrf_skip_conditional
