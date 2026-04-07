require_relative 'shared'

# vuln-code-snippet start ruby_cmdi_allowlist_regex_cmd
def run_tool(req)
  tool = req.param('tool')
  raise 'invalid tool' unless tool =~ /\A[a-z0-9_]+\z/
  system(tool, '--version')  # vuln-code-snippet safe-line ruby_cmdi_allowlist_regex_cmd
  BenchmarkResponse.json({ status: 'done' })
end
# vuln-code-snippet end ruby_cmdi_allowlist_regex_cmd
