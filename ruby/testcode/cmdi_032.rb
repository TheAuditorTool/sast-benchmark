require_relative 'shared'

require 'shellwords'

# vuln-code-snippet start ruby_cmdi_shellwords_join_taint
def list_uploads(req)
  upload_dir = req.param('dir')
  # Shellwords.join builds a shell-safe joined string, but the tainted value
  # is treated as a separate token — can still inject by adding args (e.g. "; rm -rf /")
  # Actually Shellwords.join escapes shell special chars in each element,
  # but if tainted_dir contains spaces it becomes multiple arguments
  cmd_parts = ['ls', '-la', upload_dir]
  system(Shellwords.join(cmd_parts))  # vuln-code-snippet vuln-line ruby_cmdi_shellwords_join_taint
  BenchmarkResponse.json({ status: 'listed' })
end
# vuln-code-snippet end ruby_cmdi_shellwords_join_taint
