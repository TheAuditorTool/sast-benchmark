require_relative 'shared'
require 'fileutils'

# vuln-code-snippet start ruby_pt_fileutils_cp
def copy_file(req)
  FileUtils.cp(req.param('src'), '/tmp/dest') # vuln-code-snippet vuln-line ruby_pt_fileutils_cp
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_pt_fileutils_cp
