require_relative 'shared'

# vuln-code-snippet start ruby_ssti_prawn_pdf_template
def handler(req)
  # Prawn PDF context — instance_eval executes user-supplied template string
  instance_eval(req.param('template')) # vuln-code-snippet vuln-line ruby_ssti_prawn_pdf_template
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_ssti_prawn_pdf_template
