require_relative 'shared'
require 'shellwords'

# vuln-code-snippet start ruby_cmdi_shellwords_escape
def search_files(req)
  input = req.param('query')
  safe_input = Shellwords.escape(input) # vuln-code-snippet safe-line ruby_cmdi_shellwords_escape
  result = `grep -r #{safe_input} /var/data/`
  BenchmarkResponse.ok(result)
end
# vuln-code-snippet end ruby_cmdi_shellwords_escape
