require_relative 'shared'
require 'fileutils'

# vuln-code-snippet start ruby_pt_fileutils_mv
def move_file(req)
  FileUtils.mv(req.param('src'), '/var/safe/dest') # vuln-code-snippet vuln-line ruby_pt_fileutils_mv
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_pt_fileutils_mv
