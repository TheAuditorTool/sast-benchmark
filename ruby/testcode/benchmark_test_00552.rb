require_relative 'shared'
require 'fileutils'

def move_file(req)
  FileUtils.mv(req.param('src'), '/var/safe/dest')
  BenchmarkResponse.json({ ok: true })
end
