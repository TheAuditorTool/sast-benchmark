require_relative 'shared'

def handler(req)
  fname = req.param('file').gsub(' ', '_')
  load(File.join('modules', fname + '.rb'))
  BenchmarkResponse.json({ ok: true })
end
