require_relative 'shared'

TEMPLATES = {
  1 => 'templates/invoice.rb',
  2 => 'templates/receipt.rb',
  3 => 'templates/report.rb',
}.freeze

# vuln-code-snippet start ruby_fi_template_from_db_id
def handler(req)
  id        = Integer(req.param('id'))
  tmpl_path = TEMPLATES.fetch(id) # vuln-code-snippet safe-line ruby_fi_template_from_db_id
  load(tmpl_path)
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_fi_template_from_db_id
