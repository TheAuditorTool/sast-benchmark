require_relative 'shared'

require 'shellwords'

# vuln-code-snippet start ruby_cmdi_shellwords_escape_exec
def list_uploads_safe(req)
  dir = req.param('dir')
  safe_dir = Shellwords.shellescape(dir)
  system("ls -la #{safe_dir}")  # vuln-code-snippet safe-line ruby_cmdi_shellwords_escape_exec
  BenchmarkResponse.json({ status: 'listed' })
end
# vuln-code-snippet end ruby_cmdi_shellwords_escape_exec
