require_relative 'shared'

SANDBOX_DIR = File.expand_path('sandbox').freeze

def handler(req)
  candidate = File.realpath(File.join(SANDBOX_DIR, req.param('f') + '.rb'))
  raise 'outside sandbox' unless candidate.start_with?(SANDBOX_DIR)
  load(candidate)
  BenchmarkResponse.json({ ok: true })
end
