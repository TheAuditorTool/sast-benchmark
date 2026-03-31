require_relative 'shared'

ALLOWED_TOOLS = %w[uptime hostname date uname].freeze

# vuln-code-snippet start ruby_cmdi_allowlist
def run_tool(req)
  tool = req.param('tool')
  unless ALLOWED_TOOLS.include?(tool)
    return BenchmarkResponse.bad_request("tool not permitted")
  end
  result = `#{tool}` # vuln-code-snippet safe-line ruby_cmdi_allowlist
  BenchmarkResponse.ok(result)
end
# vuln-code-snippet end ruby_cmdi_allowlist
