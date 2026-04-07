require_relative 'shared'

TEMPLATES = {
  1 => 'templates/invoice.rb',
  2 => 'templates/receipt.rb',
  3 => 'templates/report.rb',
}.freeze

def handler(req)
  id        = Integer(req.param('id'))
  tmpl_path = TEMPLATES.fetch(id)
  load(tmpl_path)
  BenchmarkResponse.json({ ok: true })
end
