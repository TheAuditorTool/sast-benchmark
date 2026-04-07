require_relative 'shared'

require 'shellwords'

def list_uploads(req)
  upload_dir = req.param('dir')
  cmd_parts = ['ls', '-la', upload_dir]
  system(Shellwords.join(cmd_parts))
  BenchmarkResponse.json({ status: 'listed' })
end
