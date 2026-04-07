require_relative 'shared'

def handler(req)
  binding.eval(IO.read("scripts/#{req.param('script')}.rb"))
  BenchmarkResponse.json({ ok: true })
end
