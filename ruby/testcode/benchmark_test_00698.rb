require_relative 'shared'
require 'fileutils'

def copy_file(req)
  FileUtils.cp(req.param('src'), '/tmp/dest')
  BenchmarkResponse.json({ ok: true })
end
