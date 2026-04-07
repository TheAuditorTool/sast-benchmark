require_relative 'shared'

require 'shellwords'

def list_uploads_safe(req)
  dir = req.param('dir')
  safe_dir = Shellwords.shellescape(dir)
  system("ls -la #{safe_dir}")
  BenchmarkResponse.json({ status: 'listed' })
end
